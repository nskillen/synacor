use super::memory::Memory;
use super::stack::Stack;

pub struct Bus {
    memory: Memory,
    stack: Stack,
}

impl Bus {
    pub fn new() -> Bus {
        Bus { memory: Memory::new(), stack: Stack::new() }
    }

    //pub fn memory(&mut self) -> &Memory { &self.memory }
    //pub fn stack(&mut self) -> &Stack { &self.stack }

    pub fn read_word(&self, addr: usize) -> u16 { self.memory.read(addr) }
    pub fn write_word(&mut self, addr: usize, value: u16) { self.memory.write(addr, value); }

    pub fn push_word(&mut self, value: u16) { self.stack.push(value); }
    pub fn pop_word(&mut self) -> Option<u16> { self.stack.pop() }

    // pub fn reset(&mut self) {
    //     self.memory = Memory::new();
    //     self.stack = Stack::new();
    // }
}