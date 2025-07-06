#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Immediate {
    immediate: u16
}

impl Immediate {
    pub fn new(immediate: u16) -> Self {
        Self { immediate }
    }

    pub fn new_signed(immediate: i32) -> Self {
        if immediate < -65_535 || immediate > 65_535 {
            panic!("Immediate {} out of range, expected -65,535-65,535", immediate);
        }

        Self {
            immediate: immediate as u16
        }
    }

    pub fn immediate(&self) -> u16 {
        self.immediate
    }
    
    pub fn set_immediate(&mut self, immediate: u16) {
        self.immediate = immediate;
    }

    pub fn set_immediate_signed(&mut self, immediate: i32) {
        if immediate < -65_535 || immediate > 65_535 {
            panic!("Immediate {} out of range, expected -65,535-65,535", immediate);
        }

        self.immediate = immediate as u16;
    }
}