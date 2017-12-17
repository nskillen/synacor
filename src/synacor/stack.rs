use super::WORD;

pub struct Stack(Vec<WORD>);

impl Stack {
    pub fn new() -> Stack {
        Stack(Vec::new())
    }

    pub fn push(&mut self, value: WORD) {
        self.0.push(value);
    }

    pub fn pop(&mut self) -> Option<WORD> {
        self.0.pop()
    }
}