pub const MEMORY_SIZE: usize = 65_536;

pub struct Memory([u16; MEMORY_SIZE]);
impl Memory {
    pub fn new() -> Memory {
        Memory([0; MEMORY_SIZE])
    }

    pub fn read(&self, address: usize) -> u16 { self.0[address] }
    pub fn write(&mut self, address: usize, value: u16) { self.0[address] = value; }
}