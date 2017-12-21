use regex::Regex;
use synacor::WORD;
use synacor::opcode::Opcode;

#[derive(Clone,Copy,Debug,PartialEq)]
pub enum Argument<'a> {
    Register(usize),
    RPointer(usize),
    MPointer(usize),
    Label(&'a str),
    Number(u16),
}

impl<'a> Argument<'a> {
    pub fn as_word(&self) -> WORD {
        match self {
            Argument::Register(r) => *r as WORD + 0x8000,
            Argument::RPointer(p) => *p as WORD + 0x8000,
            Argument::MPointer(m) => *m as WORD,
            Argument::Label(l)    => panic!("Unable to convert label argument '{}' to word", l),
            Argument::Number(n)   => *n as WORD,
        }
    }
}

impl<'a> From<&'a str> for Argument<'a> {
    fn from(s: &'a str) -> Self {
        lazy_static! {
            static ref numeric_rx: Regex  = Regex::new(r"^#(\d+)$").unwrap();
            static ref register_rx: Regex = Regex::new(r"^r(\d)$").unwrap();
            static ref rpointer_rx: Regex = Regex::new(r"^\[r(\d)\]$").unwrap();
            static ref mpointer_rx: Regex = Regex::new(r"^\[(\d+)\]$").unwrap();
            static ref label_rx: Regex = Regex::new(r"^([a-z][\w_]*):?$").unwrap();
        }

        if numeric_rx.is_match(s) {
            Argument::Number(numeric_rx.captures(s).unwrap().get(1).expect(&format!("did not capture in {}", s)[..]).as_str().parse::<u16>().expect(&format!("failed to parse {}", s)[..]))
        } else if register_rx.is_match(s) {
            Argument::Register(register_rx.captures(s).unwrap().get(1).expect(&format!("did not capture in {}", s)[..]).as_str().parse::<usize>().expect(&format!("failed to parse {}", s)[..]))
        } else if rpointer_rx.is_match(s) {
            Argument::RPointer(rpointer_rx.captures(s).unwrap().get(1).expect(&format!("did not capture in {}", s)[..]).as_str().parse::<usize>().expect(&format!("failed to parse {}", s)[..]))
        } else if mpointer_rx.is_match(s) {
            Argument::MPointer(mpointer_rx.captures(s).unwrap().get(1).expect(&format!("did not capture in {}", s)[..]).as_str().parse::<usize>().expect(&format!("failed to parse {}", s)[..]))
        } else if label_rx.is_match(s) {
            Argument::Label(label_rx.captures(s).unwrap().get(1).expect(&format!("did not capture in {}", s)[..]).as_str())
        } else {
            panic!("Unable to match argument '{}' to any pattern!", s);
        }
    }
}

#[derive(Copy,Clone,Debug,PartialEq)]
pub enum TokenType {
    Instruction,
    DataDeclaration,
}

#[derive(Clone,Debug,PartialEq)]
pub struct Token<'t> {
    pub tok_type: TokenType,
    pub label: Option<&'t str>,
    pub offset: usize,
    pub opcode: Option<Opcode>,
    pub args: [Option<Argument<'t>>; 3],
    pub data: Vec<WORD>
}

impl<'t> Token<'t> {
    pub fn new_data() -> Token<'t> {
        static mut num_dd_tokens: usize = 0;

        let t = Token {
            tok_type: TokenType::DataDeclaration,
            label: Some(&format!("dd{}", num_dd_tokens)[..]),
            offset: 0,
            opcode: None,
            args: [None, None, None],
            data: vec![],
        };

        num_dd_tokens += 1;
        t
    }

    pub fn new_instr() -> Token<'t> {
        Token {
            tok_type: TokenType::Instruction,
            label: None,
            offset: 0,
            opcode: None,
            args: [None, None, None],
            data: vec![],
        }
    }

    pub fn size(&self) -> usize {
        match self.tok_type {
            TokenType::DataDeclaration => self.data.len(),
            TokenType::Instruction => 1 + self.opcode.unwrap().argc(), 
        }
    }

    pub fn as_words(&self) -> Vec<WORD> {
        match self.tok_type {
            TokenType::DataDeclaration => self.data.clone(),
            TokenType::Instruction => {
                let mut words = Vec::new();

                words.push(self.opcode.unwrap().into());
                self.args.iter().filter(|a| a.is_some()).for_each(|a| words.push(a.unwrap().as_word()));

                words
            }
        }
    }
}