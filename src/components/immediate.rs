use crate::{bit_consts, signed_bit_consts};

signed_bit_consts!(8);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Immediate {
    immediate: u32
}

impl Immediate {
    pub fn new(immediate: u32) -> Self {
        Self {
            immediate: immediate.rem_euclid(MAX_POSSIBLE_COUNT)
        }
    }

    pub fn new_signed(immediate: i32) -> Self {
        Self {
            immediate: immediate.rem_euclid(MAX_POSSIBLE_COUNT as i32) as u32
        }
    }

    pub fn immediate(&self) -> u32 {
        self.immediate
    }
    
    pub fn set_immediate(&mut self, immediate: u32) {
        self.immediate = immediate.rem_euclid(MAX_POSSIBLE_COUNT);
    }

    pub fn set_immediate_signed(&mut self, immediate: i32) {
        self.immediate = immediate.rem_euclid(MAX_POSSIBLE_COUNT as i32) as u32;
    }
}