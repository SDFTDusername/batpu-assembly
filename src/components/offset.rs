#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Offset {
    offset: i8
}

impl Offset {
    pub fn new(offset: i8) -> Self {
        Self { offset }
    }

    pub fn offset(&self) -> i8 {
        self.offset
    }
    
    pub fn set_offset(&mut self, offset: i8) {
        self.offset = offset;
    }
}