pub struct Stack(Vec<u16>);

impl Stack {
    pub fn new() -> Stack {
        Stack(Vec::new())
    }

    pub fn push(&mut self, value: u16) {
        self.0.push(value);
    }

    pub fn pop(&mut self) -> Option<u16> {
        self.0.pop()
    }
}