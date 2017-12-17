use super::cpu::Cpu;
use super::bus::Bus;
use super::WORD;

pub struct Vm {
    cpu: Cpu,
    bus: Bus,
}

impl Vm {
    pub fn new() -> Vm {
        Vm {
            cpu: Cpu::new(),
            bus: Bus::new(),
        }
    }

    #[cfg(test)]
    pub fn cpu(&self) -> &Cpu { &self.cpu }
    //pub fn bus(&self) -> &Bus { &self.bus }

    // pub fn reset(&mut self) {
    //     self.cpu.reset();
    //     self.bus.reset();
    // }

    pub fn load_memory(&mut self, program: Vec<WORD>) {
        for (idx,instr) in program.into_iter().enumerate() {
            self.bus.write_word(idx, instr);
        }
    }

    pub fn run(&mut self) -> &super::cpu::CpuState {
        self.cpu.start();
        while self.cpu.is_running() {
            self.cpu.step(&mut self.bus);
        }
        self.cpu.state()
    }
}