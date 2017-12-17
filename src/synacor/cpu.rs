use std::collections::VecDeque;
use super::bus::Bus;

pub const MAX_MEM_ADDR:  u16   = 0x7FFF;
pub const MODULO:        u16   = 0x8000;
pub const NUM_REGISTERS: usize = 8;

#[derive(Debug)]
pub struct Cpu {
    state: CpuState,
    registers: [u16; NUM_REGISTERS], // arch has 8 16-bit registers
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

    pub fn register_get(&self, reg: usize) -> u16 { self.registers[reg] }
    pub fn register_put(&mut self, reg: usize, value: u16) { self.registers[reg] = value; }

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

    fn fetch(&self, addr: usize, bus: &Bus) -> u16 { bus.read_word(addr) }

    fn decode(&mut self, instruction_code: u16, bus: &Bus) -> Instruction {
        let pc = self.pc.clone();
        let rw = |off| bus.read_word(pc + off);

        let instruction = match instruction_code {
             0 => Instruction::Halt,
             1 => Instruction::Set(rw(1), rw(2)),
             2 => Instruction::Push(rw(1)),
             3 => Instruction::Pop(rw(1)),
             4 => Instruction::Eq(rw(1), rw(2), rw(3)),
             5 => Instruction::Gt(rw(1), rw(2), rw(3)),
             6 => Instruction::Jmp(rw(1)),
             7 => Instruction::Jt(rw(1), rw(2)),
             8 => Instruction::Jf(rw(1), rw(2)),
             9 => Instruction::Add(rw(1), rw(2), rw(3)),
            10 => Instruction::Mult(rw(1), rw(2), rw(3)),
            11 => Instruction::Mod(rw(1), rw(2), rw(3)),
            12 => Instruction::And(rw(1), rw(2), rw(3)),
            13 => Instruction::Or(rw(1), rw(2), rw(3)),
            14 => Instruction::Not(rw(1), rw(2)),
            15 => Instruction::Rmem(rw(1), rw(2)),
            16 => Instruction::Wmem(rw(1), rw(2)),
            17 => Instruction::Call(rw(1)),
            18 => Instruction::Ret,
            19 => Instruction::Out(rw(1)),
            20 => Instruction::In(rw(1)),
            21 => Instruction::Noop,
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
        let res: Result<(), String> = match instruction {
            /*  0 */ Instruction::Halt        => op::halt(self,bus),
            /*  1 */ Instruction::Set(a,b)    => op::set(self,bus,a,b),
            /*  2 */ Instruction::Push(a)     => op::push(self,bus,a),
            /*  3 */ Instruction::Pop(a)      => op::pop(self,bus,a),
            /*  4 */ Instruction::Eq(a,b,c)   => op::eq(self,a,b,c),
            /*  5 */ Instruction::Gt(a,b,c)   => op::gt(self,a,b,c),
            /*  6 */ Instruction::Jmp(a)      => op::jmp(self,a),
            /*  7 */ Instruction::Jt(a,b)     => op::jt(self,a,b),
            /*  8 */ Instruction::Jf(a,b)     => op::jf(self,a,b),
            /*  9 */ Instruction::Add(a,b,c)  => op::add(self,a,b,c),
            /* 10 */ Instruction::Mult(a,b,c) => op::mult(self,a,b,c),
            /* 11 */ Instruction::Mod(a,b,c)  => op::rmdr(self,a,b,c),
            /* 12 */ Instruction::And(a,b,c)  => op::and(self,a,b,c),
            /* 13 */ Instruction::Or(a,b,c)   => op::or(self,a,b,c),
            /* 14 */ Instruction::Not(a,b)    => op::not(self,a,b),
            /* 15 */ Instruction::Rmem(a,b)   => op::rmem(self,bus,a,b),
            /* 16 */ Instruction::Wmem(a,b)   => op::wmem(self,bus,a,b),
            /* 17 */ Instruction::Call(a)     => op::call(self,bus,a),
            /* 18 */ Instruction::Ret         => op::ret(self,bus),
            /* 19 */ Instruction::Out(a)      => op::outc(self,a),
            /* 20 */ Instruction::In(a)       => op::inc(self,a),
            /* 21 */ Instruction::Noop        => op::noop(),
        };

        if res.is_err() {
            self.state = CpuState::Error;
            println!("ERROR");
            println!("Instruction failed: {}", res.unwrap_err());
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
enum Instruction {
    Halt,               //  0       - halts execution
    Set  (u16,u16),     //  1 a b   - set register <a> to the value of <b>
    Push (u16),         //  2 a     - push <a> onto the stack
    Pop  (u16),         //  3 a     - remove the top element from the stack, and write to <a>, empty stack = ERR
    Eq   (u16,u16,u16), //  4 a b c - set <a> to 1 if <b> is equal to <c>, set to 0 otherwise
    Gt   (u16,u16,u16), //  5 a b c - set <a> to 1 if <b> is greater than <c>, set to 0 otherwise
    Jmp  (u16),         //  6 a     - jump to <a>
    Jt   (u16,u16),     //  7 a b   - if <a> is non-zero, jump to <b>, aka Jnz
    Jf   (u16,u16),     //  8 a b   - if <a> is zero, jump to <b>, aka Jz
    Add  (u16,u16,u16), //  9 a b c - store into <a> the sum of <b> and <c>, mod MODULO
    Mult (u16,u16,u16), // 10 a b c - store into <a> the product of <b> and <c>, mod MODULO
    Mod  (u16,u16,u16), // 11 a b c - store into <a> the remainder of <b> divided by <c>
    And  (u16,u16,u16), // 12 a b c - store into <a> the bitwise and of <b> and <c>
    Or   (u16,u16,u16), // 13 a b c - store into <a> the bitwise or of <b> and <c>
    Not  (u16,u16),     // 14 a b   - store into <a> the bitwise not of <b>
    Rmem (u16,u16),     // 15 a b   - read memory at address <b>, write to <a>
    Wmem (u16,u16),     // 16 a b   - write value from <b> into memory at address <a>
    Call (u16),         // 17 a     - writes the address of the next instruction to the stack, and jumps to <a>
    Ret,                // 18       - remove the top element from the stack and jump to it. empty stack == HLT
    Out  (u16),         // 19 a     - print the character represented by ASCII(<a>) to the terminal
    In   (u16),         // 20 a     - read a character from the terminal, and store ASCII_VAL(c) => <a>
    Noop,               // 21       - no operation
}

#[derive(Debug,PartialEq)]
pub enum CpuState {
    NotStarted,
    Running,
    Halted,
    Error
}

enum Addr {
    Register(usize),
    Immediate(u16),
}

impl Addr {
    fn map(value: u16) -> Addr {
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

    use super::{Addr,Bus,Cpu,CpuState,MAX_MEM_ADDR,MODULO};

    fn map_val(v: u16, cpu: &Cpu) -> u16 {
        match Addr::map(v) {
            Addr::Register(r) => cpu.register_get(r),
            Addr::Immediate(i) => i,
        }
    }

    fn map_reg(v: u16) -> Result<usize,String> {
        match Addr::map(v) {
            Addr::Register(r) => Ok(r),
            Addr::Immediate(i) => Err(format!("Invalid register: {}", i)),
        }
    }

    fn read_char() -> Result<u16, String> {
        loop {
            let input = std::io::stdin()
            .bytes()
            .next()
            .and_then(|r| r.ok())
            .map(|b| b as u16);

            if input.is_some() && input.unwrap() == 0x0D { continue; } // skip CHR(13) on windows

            return input.ok_or(format!("Error reading from keyboard"))
        }
    }

    /* CPU OPS HERE */

    pub fn halt(cpu: &mut Cpu, _bus: &Bus) -> Result<(), String> {
        cpu.state = CpuState::Halted;
        Ok(())
    }

    pub fn set(cpu: &mut Cpu, _bus: &Bus, a: u16, b: u16) -> Result<(), String> {
        map_reg(a).and_then(|_a| {
            let _b = map_val(b,cpu);
            cpu.register_put(_a, _b);
            Ok(())
        })
    }

    pub fn push(cpu: &mut Cpu, bus: &mut Bus, a: u16) -> Result<(), String> {
        let _a = map_val(a,cpu);
        bus.push_word(_a);
        Ok(())
    }

    pub fn pop(cpu: &mut Cpu, bus: &mut Bus, a: u16) -> Result<(), String> {
        map_reg(a).and_then(|_a| {
            match bus.pop_word() {
                Some(v) => cpu.register_put(_a, v),
                None => return Err(format!("Attempted to pop from empty stack!")),
            }
            Ok(())
        })
    }

    pub fn eq(cpu: &mut Cpu, a: u16, b: u16, c: u16) -> Result<(), String> {
        map_reg(a).and_then(|_a| {
            let _b = map_val(b,cpu);
            let _c = map_val(c,cpu);
            cpu.register_put(_a, if _b == _c { 1 } else { 0 });
            Ok(())
        })
    }

    pub fn gt(cpu: &mut Cpu, a: u16, b: u16, c: u16) -> Result<(), String> {
        map_reg(a).and_then(|_a| {
            let _b = map_val(b,cpu);
            let _c = map_val(c,cpu);
            cpu.register_put(_a, if _b > _c { 1 } else { 0 });
            Ok(())
        })
    }

    pub fn jmp(cpu: &mut Cpu, a: u16) -> Result<(), String> {
        let _a = map_val(a,cpu);
        if _a > MAX_MEM_ADDR {
            Err(format!("Jump target invalid: {:#04X}", _a))
        } else {
            cpu.pc = _a as usize;
            Ok(())
        }
    }

    pub fn jt(cpu: &mut Cpu, a: u16, b: u16) -> Result<(), String> {
        let _a = map_val(a,cpu);
        let _b = map_val(b,cpu);
        if _a != 0 {
            if _b > MAX_MEM_ADDR {
                return Err(format!("Jump target invalid: {:#04X}", _b))
            } else {
                cpu.pc = _b as usize;
            }
        }
        Ok(())
    }

    pub fn jf(cpu: &mut Cpu, a: u16, b: u16) -> Result<(), String> {
        let _a = map_val(a,cpu);
        let _b = map_val(b,cpu);
        if _a == 0 {
            if _b > MAX_MEM_ADDR {
                return Err(format!("Jump target invalid: {:#04X}", _b))
            } else {
                cpu.pc = _b as usize;
            }
        }
        Ok(())
    }

    pub fn add(cpu: &mut Cpu, a: u16, b: u16, c: u16) -> Result<(), String> {
        map_reg(a).and_then(|_a| {
            let _b = map_val(b,cpu);
            let _c = map_val(c,cpu);
            cpu.register_put(_a, (_b + _c) % MODULO);
            Ok(())
        })
    }

    pub fn mult(cpu: &mut Cpu, a: u16, b: u16, c: u16) -> Result<(), String> {
        map_reg(a).and_then(|_a| {
            let _b = map_val(b,cpu);
            let _c = map_val(c,cpu);
            cpu.register_put(_a, ((_b as u32 * _c as u32) % MODULO as u32) as u16);
            Ok(())
        })
    }

    pub fn rmdr(cpu: &mut Cpu, a: u16, b: u16, c: u16) -> Result<(), String> {
        map_reg(a).and_then(|_a| {
            let _b = map_val(b,cpu);
            let _c = map_val(c,cpu);
            cpu.register_put(_a, _b % _c);
            Ok(())
        })
    }

    pub fn and(cpu: &mut Cpu, a: u16, b: u16, c: u16) -> Result<(), String> {
        map_reg(a).and_then(|_a| {
            let _b = map_val(b,cpu);
            let _c = map_val(c,cpu);
            cpu.register_put(_a, _b.bitand(_c));
            Ok(())
        })
    }

    pub fn or(cpu: &mut Cpu, a: u16, b: u16, c: u16) -> Result<(), String> {
        map_reg(a).and_then(|_a| {
            let _b = map_val(b,cpu);
            let _c = map_val(c,cpu);
            cpu.register_put(_a, _b.bitor(_c));
            Ok(())
        })
    }

    pub fn not(cpu: &mut Cpu, a: u16, b: u16) -> Result<(), String> {
        map_reg(a).and_then(|_a| {
            let _b = map_val(b,cpu);
            cpu.register_put(_a, (!_b).bitand(0x7FFF));
            Ok(())
        })
    }

    pub fn rmem(cpu: &mut Cpu, bus: &Bus, a: u16, b: u16) -> Result<(), String> {
        map_reg(a).and_then(|_a| {
            let _b = map_val(b,cpu);
            cpu.register_put(_a, bus.read_word(_b as usize));
            Ok(())
        })
    }

    pub fn wmem(cpu: &mut Cpu, bus: &mut Bus, a: u16, b: u16) -> Result<(), String> {
        let _a = map_val(a,cpu);
        let _b = map_val(b,cpu);
        bus.write_word(_a as usize, _b);
        Ok(())
    }

    pub fn call(cpu: &mut Cpu, bus: &mut Bus, a: u16) -> Result<(), String> {
        let _a = map_val(a,cpu);
        if _a > MAX_MEM_ADDR {
            Err(format!("Called invalid memory address: {:#04X}", _a))
        } else {
            bus.push_word(cpu.pc as u16);
            cpu.pc = _a as usize;
            Ok(())
        }
    }

    pub fn ret(cpu: &mut Cpu, bus: &mut Bus) -> Result<(), String> {
        match bus.pop_word() {
            Some(v) => cpu.pc = v as usize,
            None => return Err(format!("Attempted to return with nothing on the stack")),
        }
        Ok(())
    }

    pub fn outc(cpu: &mut Cpu, a: u16) -> Result<(), String> {
        let _a = map_val(a,cpu);
        print!("{}", _a as u8 as char);
        Ok(())
    }

    pub fn inc(cpu: &mut Cpu, a: u16) -> Result<(), String> {
        map_reg(a).and_then(|_a| {
            read_char().and_then(|c| {
                cpu.register_put(_a, c);
                Ok(())
            })
        })
    }

    pub fn noop() -> Result<(), String> { Ok(()) }
}