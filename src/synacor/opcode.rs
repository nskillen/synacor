use super::WORD;

#[derive(Clone,Copy,Debug,PartialEq)]
pub enum Opcode {
    Halt, Set,  Push, Pop, Eq,
    Gt,   Jmp,  Jt,   Jf,  Add,
    Mult, Mod,  And,  Or,  Not,
    Rmem, Wmem, Call, Ret, Out,
    In,   Noop,
}

impl Opcode {
    pub fn argc(&self) -> usize {
        match self {
            Opcode::Halt => 0,
            Opcode::Set  => 2,
            Opcode::Push => 1,
            Opcode::Pop  => 1,
            Opcode::Eq   => 3,
            Opcode::Gt   => 3,
            Opcode::Jmp  => 1,
            Opcode::Jt   => 2,
            Opcode::Jf   => 2,
            Opcode::Add  => 3,
            Opcode::Mult => 3,
            Opcode::Mod  => 3,
            Opcode::And  => 3,
            Opcode::Or   => 3,
            Opcode::Not  => 2,
            Opcode::Rmem => 2,
            Opcode::Wmem => 2,
            Opcode::Call => 1,
            Opcode::Ret  => 0,
            Opcode::Out  => 1,
            Opcode::In   => 1,
            Opcode::Noop => 0,
        }
    }

    pub fn try_from<'s>(s: &'s str) -> Option<Self> {
        match s {
            "halt" | "hlt" => Some(Opcode::Halt),
            "set"          => Some(Opcode::Set),
            "push"         => Some(Opcode::Push),
            "pop"          => Some(Opcode::Pop),
            "eq"           => Some(Opcode::Eq),
            "gt"           => Some(Opcode::Gt),
            "jmp"          => Some(Opcode::Jmp),
            "jt" | "jnz"   => Some(Opcode::Jt),
            "jf" | "jz"    => Some(Opcode::Jf),
            "add"          => Some(Opcode::Add),
            "mult"         => Some(Opcode::Mult),
            "mod"          => Some(Opcode::Mod),
            "and"          => Some(Opcode::And),
            "or"           => Some(Opcode::Or),
            "not"          => Some(Opcode::Not),
            "rmem"         => Some(Opcode::Rmem),
            "wmem"         => Some(Opcode::Wmem),
            "call"         => Some(Opcode::Call),
            "ret"          => Some(Opcode::Ret),
            "out"          => Some(Opcode::Out),
            "in"           => Some(Opcode::In),
            "noop"         => Some(Opcode::Noop),
            _              => None,
        }
    }
}

impl From<WORD> for Opcode {
    fn from(w: WORD) -> Self {
        match w {
             0 => Opcode::Halt,
             1 => Opcode::Set,
             2 => Opcode::Push,
             3 => Opcode::Pop,
             4 => Opcode::Eq,
             5 => Opcode::Gt,
             6 => Opcode::Jmp,
             7 => Opcode::Jt,
             8 => Opcode::Jf,
             9 => Opcode::Add,
            10 => Opcode::Mult,
            11 => Opcode::Mod,
            12 => Opcode::And,
            13 => Opcode::Or,
            14 => Opcode::Not,
            15 => Opcode::Rmem,
            16 => Opcode::Wmem,
            17 => Opcode::Call,
            18 => Opcode::Ret,
            19 => Opcode::Out,
            20 => Opcode::In,
            21 => Opcode::Noop,
            _ => panic!("Invalid opcode: {}", w),
        }
    }
}

impl From<Opcode> for WORD {
    fn from(o: Opcode) -> Self {
        match o {
            Opcode::Halt => 0,
            Opcode::Set  => 1,
            Opcode::Push => 2,
            Opcode::Pop  => 3,
            Opcode::Eq   => 4,
            Opcode::Gt   => 5,
            Opcode::Jmp  => 6,
            Opcode::Jt   => 7,
            Opcode::Jf   => 8,
            Opcode::Add  => 9,
            Opcode::Mult => 10,
            Opcode::Mod  => 11,
            Opcode::And  => 12,
            Opcode::Or   => 13,
            Opcode::Not  => 14,
            Opcode::Rmem => 15,
            Opcode::Wmem => 16,
            Opcode::Call => 17,
            Opcode::Ret  => 18,
            Opcode::Out  => 19,
            Opcode::In   => 20,
            Opcode::Noop => 21,
        }
    }
}