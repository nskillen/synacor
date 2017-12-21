use super::cpu::Cpu;
use super::bus::Bus;
use super::WORD;

pub struct Vm {
    cpu: Cpu,
    bus: Bus,
    debug_mode: bool,
}

impl Vm {
    pub fn new() -> Vm {
        Vm {
            cpu: Cpu::new(),
            bus: Bus::new(),
            debug_mode: false,
        }
    }

    pub fn set_debug(&mut self, val: bool) { self.debug_mode = val; }

    #[cfg(test)]
    pub fn cpu(&self) -> &Cpu { &self.cpu }

    pub fn load_memory(&mut self, program: Vec<WORD>) {
        for (idx,instr) in program.into_iter().enumerate() {
            if self.debug_mode { println!("Loaded word @{:#06X}: {:#06X}", idx, instr); }
            self.bus.write_word(idx, instr);
        }
    }

    pub fn run(&mut self) -> &super::cpu::CpuState {
        self.cpu.start();
        while self.cpu.is_running() {
            self.cpu.step(&mut self.bus, self.debug_mode);
        }
        self.cpu.state()
    }
}