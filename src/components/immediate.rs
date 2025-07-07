use crate::assembly_error::AssemblyError;
use crate::{bit_consts, signed_bit_consts};

signed_bit_consts!(8);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Immediate {
    immediate: u32
}

impl Immediate {
    pub fn new(immediate: u32) -> Result<Self, AssemblyError> {
        Self::error(immediate)?;
        Ok(Self { immediate })
    }

    pub fn new_signed(immediate: i32) -> Result<Self, AssemblyError> {
        Self::error_signed(immediate)?;
        Ok(Self {
            immediate: immediate as u32
        })
    }

    pub fn immediate(&self) -> u32 {
        self.immediate
    }
    
    pub fn set_immediate(&mut self, immediate: u32) -> Result<(), AssemblyError> {
        Self::error(immediate)?;
        
        self.immediate = immediate;
        Ok(())
    }

    pub fn set_immediate_signed(&mut self, immediate: i32) -> Result<(), AssemblyError> {
        Self::error_signed(immediate)?;
        
        self.immediate = immediate as u32;
        Ok(())
    }

    fn error(immediate: u32) -> Result<(), AssemblyError> {
        if immediate > MAX_VALUE {
            Err(AssemblyError::new(format!("Immediate {} out of range, expected 0-{}", immediate, MAX_VALUE)))
        } else {
            Ok(())
        }
    }

    fn error_signed(immediate: i32) -> Result<(), AssemblyError> {
        if immediate < MIN_SIGNED_VALUE || immediate > MAX_SIGNED_VALUE {
            Err(AssemblyError::new(format!("Immediate {} out of range, expected {}-{}", immediate, MIN_SIGNED_VALUE, MAX_SIGNED_VALUE)))
        } else {
            Ok(())
        }
    }
}