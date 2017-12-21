enum Breakpoint {
    Instruction(u16),
    RegisterRead(usize),
    RegisterWrite(usize),
    MemoryRead(usize),
    MemoryWrite(usize)
}

pub struct Debugger {
    
}