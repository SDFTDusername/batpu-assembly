use crate::assembly_error::AssemblyError;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Register {
    register: u8
}

impl Register {
    pub fn new(register: u8) -> Result<Self, AssemblyError> {
        Self::error(register)?;
        Ok(Self { register })
    }

    pub fn register(&self) -> u8 {
        self.register
    }
    
    pub fn set_register(&mut self, register: u8) -> Result<(), AssemblyError> {
        Self::error(register)?;
        
        self.register = register;
        Ok(())
    }

    fn error(register: u8) -> Result<(), AssemblyError> {
        if register > 15 {
            Err(AssemblyError::new(format!("Register {} out of range, expected 0-15", register)))
        } else {
            Ok(())
        }
    }
}