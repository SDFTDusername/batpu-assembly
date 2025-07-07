use crate::assembly_error::AssemblyError;
use crate::bit_consts;

bit_consts!(4);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Register {
    register: u32
}

impl Register {
    pub fn new(register: u32) -> Result<Self, AssemblyError> {
        Self::error(register)?;
        Ok(Self { register })
    }

    pub fn register(&self) -> u32 {
        self.register
    }
    
    pub fn set_register(&mut self, register: u32) -> Result<(), AssemblyError> {
        Self::error(register)?;
        
        self.register = register;
        Ok(())
    }

    fn error(register: u32) -> Result<(), AssemblyError> {
        if register > MAX_VALUE {
            Err(AssemblyError::new(format!("Register {} out of range, expected 0-{}", register, MAX_VALUE)))
        } else {
            Ok(())
        }
    }
}