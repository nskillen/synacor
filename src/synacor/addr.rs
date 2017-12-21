use super::WORD;

#[derive(Debug)]
pub enum Addr {
    Register(usize),
    Immediate(WORD),
}

impl Addr {
    pub fn map(value: WORD) -> Addr {
        match value {
            v if v <  0x8000               => Addr::Immediate(v),
            v if v >= 0x8000 && v < 0x8008 => Addr::Register((v - 0x8000) as usize),
            v                              => panic!("Invalid number: {}", v),
        }
    }
}