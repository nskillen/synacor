use super::WORD;
pub const MEMORY_SIZE: usize = 65_536;

pub struct Memory([WORD; MEMORY_SIZE]);
impl Memory {
    pub fn new() -> Memory {
        Memory([0; MEMORY_SIZE])
    }

    pub fn read(&self, address: usize) -> WORD { self.0[address] }
    pub fn write(&mut self, address: usize, value: WORD) { self.0[address] = value; }
}