use super::memory::Memory;
use super::stack::Stack;
use super::WORD;

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

    pub fn read_word(&self, addr: usize) -> WORD { self.memory.read(addr) }
    pub fn write_word(&mut self, addr: usize, value: WORD) { self.memory.write(addr, value); }

    pub fn push_word(&mut self, value: WORD) { self.stack.push(value); }
    pub fn pop_word(&mut self) -> Result<WORD,String> { self.stack.pop().ok_or(format!("Attempted to pop from empty stack")) }

    // pub fn reset(&mut self) {
    //     self.memory = Memory::new();
    //     self.stack = Stack::new();
    // }
}