use crate::assembly_error::AssemblyError;
use crate::{bit_consts, signed_bit_consts};

signed_bit_consts!(4);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Offset {
    offset: i32
}

impl Offset {
    pub fn new(offset: i32) -> Result<Self, AssemblyError> {
        Self::error(offset)?;
        Ok(Self { offset })
    }

    pub fn offset(&self) -> i32 {
        self.offset
    }
    
    pub fn set_offset(&mut self, offset: i32) -> Result<(), AssemblyError> {
        Self::error(offset)?;
        
        self.offset = offset;
        Ok(())
    }

    fn error(offset: i32) -> Result<(), AssemblyError> {
        if offset < MIN_SIGNED_VALUE || offset > MAX_SIGNED_VALUE {
            Err(AssemblyError::new(format!("Offset {} out of range, expected {}-{}", offset, MIN_SIGNED_VALUE, MAX_SIGNED_VALUE)))
        } else {
            Ok(())
        }
    }
}