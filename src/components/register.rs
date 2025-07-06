#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Register {
    register: u8
}

impl Register {
    pub fn new(register: u8) -> Self {
        Self { register }
    }

    pub fn register(&self) -> u8 {
        self.register
    }
    
    pub fn set_register(&mut self, register: u8) {
        self.register = register;
    }
}