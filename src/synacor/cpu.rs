use std::collections::VecDeque;
use super::bus::Bus;
use super::{OpRes,WORD};

pub const MAX_MEM_ADDR:  WORD   = 0x7FFF;
pub const MODULO:        WORD   = 0x8000;
pub const NUM_REGISTERS: usize = 8;

#[derive(Debug)]
pub struct Cpu {
    state: CpuState,
    registers: [WORD; NUM_REGISTERS], // arch has 8 16-bit registers
    pc: usize, // instruction pointer
    //sp: usize, // stack pointer
    prev_instructions: VecDeque<Instruction>,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            state: CpuState::NotStarted,
            registers: [0; NUM_REGISTERS],
            pc: 0,
            //sp: 0,
            prev_instructions: VecDeque::new(),
        }
    }

    // pub fn reset(&mut self) {
    //     self.state = CpuState::NotStarted;
    //     self.registers = [0; NUM_REGISTERS];
    //     self.pc = 0;
    //     self.sp = 0;
    // }

    pub fn register_get(&self, reg: usize) -> WORD { self.registers[reg] }
    pub fn register_put(&mut self, reg: usize, value: WORD) { self.registers[reg] = value; }

    pub fn start(&mut self) {
        if self.state != CpuState::NotStarted {
            panic!("Attempted to start CPU from invalid state: {:?}", self.state);
        }
        self.state = CpuState::Running;
    }

    pub fn step(&mut self, bus: &mut Bus) {
        // ensure we're running, first.
        if self.state != CpuState::Running {
            panic!("Attempted to execute instruction while in invalid state: {:?}", self.state);
        }

        // fetch
        if self.pc > 0x7FFF { panic!("PC beyond memory addressing range: {:#04X}", self.pc); }

        let instruction_code = self.fetch(self.pc, bus);
        let instruction = self.decode(instruction_code, bus);
        
        self.prev_instructions.push_front(instruction);
        if self.prev_instructions.len() > 5 { self.prev_instructions.pop_back(); }

        self.execute(instruction, bus);
    }

    pub fn is_running(&self) -> bool {
        self.state == CpuState::Running
    }

    pub fn state(&self) -> &CpuState { &self.state }

    #[cfg(test)]
    pub fn pc(&self) -> usize { self.pc }
    //pub fn sp(&self) -> usize { self.sp }

    /* PRIVATE */

    fn fetch(&self, addr: usize, bus: &Bus) -> WORD { bus.read_word(addr) }

    fn decode(&mut self, instruction_code: WORD, bus: &Bus) -> Instruction {
        let pc = self.pc.clone();
        let rw = |off| bus.read_word(pc + off);

        let instruction = match instruction_code {
             0 => (Opcode::Halt, 0,     0,     0),
             1 => (Opcode::Set,  rw(1), rw(2), 0),
             2 => (Opcode::Push, rw(1), 0,     0),
             3 => (Opcode::Pop,  rw(1), 0,     0),
             4 => (Opcode::Eq,   rw(1), rw(2), rw(3)),
             5 => (Opcode::Gt,   rw(1), rw(2), rw(3)),
             6 => (Opcode::Jmp,  rw(1), 0,     0),
             7 => (Opcode::Jt,   rw(1), rw(2), 0),
             8 => (Opcode::Jf,   rw(1), rw(2), 0),
             9 => (Opcode::Add,  rw(1), rw(2), rw(3)),
            10 => (Opcode::Mult, rw(1), rw(2), rw(3)),
            11 => (Opcode::Mod,  rw(1), rw(2), rw(3)),
            12 => (Opcode::And,  rw(1), rw(2), rw(3)),
            13 => (Opcode::Or,   rw(1), rw(2), rw(3)),
            14 => (Opcode::Not,  rw(1), rw(2), 0),
            15 => (Opcode::Rmem, rw(1), rw(2), 0),
            16 => (Opcode::Wmem, rw(1), rw(2), 0),
            17 => (Opcode::Call, rw(1), 0,     0),
            18 => (Opcode::Ret,  0,     0,     0),
            19 => (Opcode::Out,  rw(1), 0,     0),
            20 => (Opcode::In,   rw(1), 0,     0),
            21 => (Opcode::Noop, 0,     0,     0),
            _  => panic!("Unknown instruction code: {}", instruction_code),
        };

        let pc_advance = match instruction_code {
            0                        => 0, // don't advance IP on HLT
            18 | 21                  => 1, // no arguments
            2 | 3 | 6 | 17 | 19 | 20 => 2, // one argument
            1 | 7 | 8 | 14 | 15 | 16 => 3, // two arguments
            _                        => 4, // three arguments
        };

        self.pc += pc_advance;

        instruction
    }

    fn execute(&mut self, instruction: Instruction, bus: &mut Bus) {
        let res: OpRes<String> = match instruction {
            /*  0 */ (Opcode::Halt, _, _, _) => op::halt(self,bus),
            /*  1 */ (Opcode::Set,  a, b, _) => op::set(self,bus,a,b),
            /*  2 */ (Opcode::Push, a, _, _) => op::push(self,bus,a),
            /*  3 */ (Opcode::Pop,  a, _, _) => op::pop(self,bus,a),
            /*  4 */ (Opcode::Eq,   a, b, c) => op::eq(self,a,b,c),
            /*  5 */ (Opcode::Gt,   a, b, c) => op::gt(self,a,b,c),
            /*  6 */ (Opcode::Jmp,  a, _, _) => op::jmp(self,a),
            /*  7 */ (Opcode::Jt,   a, b, _) => op::jt(self,a,b),
            /*  8 */ (Opcode::Jf,   a, b, _) => op::jf(self,a,b),
            /*  9 */ (Opcode::Add,  a, b, c) => op::add(self,a,b,c),
            /* 10 */ (Opcode::Mult, a, b, c) => op::mult(self,a,b,c),
            /* 11 */ (Opcode::Mod,  a, b, c) => op::rmdr(self,a,b,c),
            /* 12 */ (Opcode::And,  a, b, c) => op::and(self,a,b,c),
            /* 13 */ (Opcode::Or,   a, b, c) => op::or(self,a,b,c),
            /* 14 */ (Opcode::Not,  a, b, _) => op::not(self,a,b),
            /* 15 */ (Opcode::Rmem, a, b, _) => op::rmem(self,bus,a,b),
            /* 16 */ (Opcode::Wmem, a, b, _) => op::wmem(self,bus,a,b),
            /* 17 */ (Opcode::Call, a, _, _) => op::call(self,bus,a),
            /* 18 */ (Opcode::Ret,  _, _, _) => op::ret(self,bus),
            /* 19 */ (Opcode::Out,  a, _, _) => op::outc(self,a),
            /* 20 */ (Opcode::In,   a, _, _) => op::inc(self,a),
            /* 21 */ (Opcode::Noop, _, _, _) => op::noop(),
        };

        if res.is_failure() {
            self.state = CpuState::Error;
            println!("ERROR");
            println!("Instruction failed: {}", res.unwrap_failure());
            println!("CPU State:");
            println!("{:?}", self);
            println!("Last 5 instructions:");
            loop {
                let instr = self.prev_instructions.pop_back();
                if instr.is_none() { break; }
                let instr = instr.unwrap();
                println!("  {:?}", instr);
            }
        }
    }
}

#[derive(Clone,Copy,Debug)]
pub enum Opcode {
    Halt, Set,  Push, Pop, Eq,
    Gt,   Jmp,  Jt,   Jf,  Add,
    Mult, Mod,  And,  Or,  Not,
    Rmem, Wmem, Call, Ret, Out,
    In,   Noop,
}
type Instruction = (Opcode,WORD,WORD,WORD);

#[derive(Debug,PartialEq)]
pub enum CpuState {
    NotStarted,
    Running,
    Halted,
    Error
}

enum Addr {
    Register(usize),
    Immediate(WORD),
}

impl Addr {
    fn map(value: WORD) -> Addr {
        match value {
            v if v <  0x8000               => Addr::Immediate(v),
            v if v >= 0x8000 && v < 0x8008 => Addr::Register((v - 0x8000) as usize),
            v                              => panic!("Invalid number: {}", v),
        }
    }
}

mod op {
    use std;
    use std::io::Read;
    use std::ops::{BitAnd,BitOr};
    use super::{OpRes,WORD};
    use super::OpRes::{Success,Failure};

    use super::{Addr,Bus,Cpu,CpuState,MAX_MEM_ADDR,MODULO};

    fn map_val(v: WORD, cpu: &Cpu) -> WORD {
        match Addr::map(v) {
            Addr::Register(r) => cpu.register_get(r),
            Addr::Immediate(i) => i,
        }
    }

    fn map_reg(v: WORD) -> Result<usize,String> {
        match Addr::map(v) {
            Addr::Register(r) => Ok(r),
            Addr::Immediate(i) => Err(format!("Invalid register: {}", i)),
        }
    }

    fn read_char() -> Result<WORD, String> {
        loop {
            let input = std::io::stdin()
            .bytes()
            .next()
            .and_then(|r| r.ok())
            .map(|b| b as WORD);

            if input.is_some() && input.unwrap() == 0x0D { continue; } // skip CHR(13) on windows

            return input.ok_or(format!("Error reading from keyboard"))
        }
    }

    /* CPU OPS HERE */

    pub fn halt(cpu: &mut Cpu, _bus: &Bus) -> OpRes<String> {
        cpu.state = CpuState::Halted;
        Success
    }

    pub fn set(cpu: &mut Cpu, _bus: &Bus, a: WORD, b: WORD) -> OpRes<String> {
        let reg = map_reg(a)?;
        let data = map_val(b,cpu);
        cpu.register_put(reg, data);
        Success
    }

    pub fn push(cpu: &mut Cpu, bus: &mut Bus, a: WORD) -> OpRes<String> {
        let _a = map_val(a,cpu);
        bus.push_word(_a);
        Success
    }

    pub fn pop(cpu: &mut Cpu, bus: &mut Bus, a: WORD) -> OpRes<String> {
        let reg = map_reg(a)?;
        let data = bus.pop_word()?;
        cpu.register_put(reg, data);
        Success
    }

    pub fn eq(cpu: &mut Cpu, a: WORD, b: WORD, c: WORD) -> OpRes<String> {
        let reg = map_reg(a)?;
        let _b = map_val(b,cpu);
        let _c = map_val(c,cpu);
        cpu.register_put(reg, if _b == _c { 1 } else { 0 });
        Success
    }

    pub fn gt(cpu: &mut Cpu, a: WORD, b: WORD, c: WORD) -> OpRes<String> {
        let reg = map_reg(a)?;
        let _b = map_val(b,cpu);
        let _c = map_val(c,cpu);
        cpu.register_put(reg, if _b > _c { 1 } else { 0 });
        Success
    }

    pub fn jmp(cpu: &mut Cpu, a: WORD) -> OpRes<String> {
        let _a = map_val(a,cpu);
        if _a > MAX_MEM_ADDR {
            Failure(format!("Jump target invalid: {:#04X}", _a))
        } else {
            cpu.pc = _a as usize;
            Success
        }
    }

    pub fn jt(cpu: &mut Cpu, a: WORD, b: WORD) -> OpRes<String> {
        let _a = map_val(a,cpu);
        let _b = map_val(b,cpu);
        if _a != 0 {
            if _b > MAX_MEM_ADDR {
                return Failure(format!("Jump target invalid: {:#04X}", _b))
            } else {
                cpu.pc = _b as usize;
            }
        }
        Success
    }

    pub fn jf(cpu: &mut Cpu, a: WORD, b: WORD) -> OpRes<String> {
        let _a = map_val(a,cpu);
        let _b = map_val(b,cpu);
        if _a == 0 {
            if _b > MAX_MEM_ADDR {
                return Failure(format!("Jump target invalid: {:#04X}", _b))
            } else {
                cpu.pc = _b as usize;
            }
        }
        Success
    }

    pub fn add(cpu: &mut Cpu, a: WORD, b: WORD, c: WORD) -> OpRes<String> {
        let reg = map_reg(a)?;
        let _b = map_val(b,cpu);
        let _c = map_val(c,cpu);
        cpu.register_put(reg, (_b + _c) % MODULO);
        Success
    }

    pub fn mult(cpu: &mut Cpu, a: WORD, b: WORD, c: WORD) -> OpRes<String> {
        let reg = map_reg(a)?;
        let _b = map_val(b,cpu);
        let _c = map_val(c,cpu);
        cpu.register_put(reg, ((_b as u32 * _c as u32) % MODULO as u32) as WORD);
        Success
    }

    pub fn rmdr(cpu: &mut Cpu, a: WORD, b: WORD, c: WORD) -> OpRes<String> {
        let reg = map_reg(a)?;
        let _b = map_val(b,cpu);
        let _c = map_val(c,cpu);
        cpu.register_put(reg, _b % _c);
        Success
    }

    pub fn and(cpu: &mut Cpu, a: WORD, b: WORD, c: WORD) -> OpRes<String> {
        let reg = map_reg(a)?;
        let _b = map_val(b,cpu);
        let _c = map_val(c,cpu);
        cpu.register_put(reg, _b.bitand(_c));
        Success
    }

    pub fn or(cpu: &mut Cpu, a: WORD, b: WORD, c: WORD) -> OpRes<String> {
        let reg = map_reg(a)?;
        let _b = map_val(b,cpu);
        let _c = map_val(c,cpu);
        cpu.register_put(reg, _b.bitor(_c));
        Success
    }

    pub fn not(cpu: &mut Cpu, a: WORD, b: WORD) -> OpRes<String> {
        let reg = map_reg(a)?;
        let _b = map_val(b,cpu);
        cpu.register_put(reg, (!_b).bitand(0x7FFF));
        Success
    }

    pub fn rmem(cpu: &mut Cpu, bus: &Bus, a: WORD, b: WORD) -> OpRes<String> {
        let reg = map_reg(a)?;
        let _b = map_val(b,cpu);
        cpu.register_put(reg, bus.read_word(_b as usize));
        Success
    }

    pub fn wmem(cpu: &mut Cpu, bus: &mut Bus, a: WORD, b: WORD) -> OpRes<String> {
        let _a = map_val(a,cpu);
        let _b = map_val(b,cpu);
        bus.write_word(_a as usize, _b);
        Success
    }

    pub fn call(cpu: &mut Cpu, bus: &mut Bus, a: WORD) -> OpRes<String> {
        let _a = map_val(a,cpu);
        if _a > MAX_MEM_ADDR {
            Failure(format!("Called invalid memory address: {:#04X}", _a))
        } else {
            bus.push_word(cpu.pc as WORD);
            cpu.pc = _a as usize;
            Success
        }
    }

    pub fn ret(cpu: &mut Cpu, bus: &mut Bus) -> OpRes<String> {
        let data = bus.pop_word()?;
        cpu.pc = data as usize;
        Success
    }

    pub fn outc(cpu: &mut Cpu, a: WORD) -> OpRes<String> {
        let _a = map_val(a,cpu);
        print!("{}", _a as u8 as char);
        Success
    }

    pub fn inc(cpu: &mut Cpu, a: WORD) -> OpRes<String> {
        let reg = map_reg(a)?;
        let c = read_char()?;
        cpu.register_put(reg, c);
        Success
    }

    pub fn noop() -> OpRes<String> { Success }
}