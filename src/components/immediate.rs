use crate::assembly_error::AssemblyError;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Immediate {
    immediate: u8
}

impl Immediate {
    pub fn new(immediate: u8) -> Result<Self, AssemblyError> {
        Self::error(immediate)?;
        Ok(Self { immediate })
    }

    pub fn new_signed(immediate: i16) -> Result<Self, AssemblyError> {
        Self::error_signed(immediate)?;
        Ok(Self {
            immediate: immediate as u8
        })
    }

    pub fn immediate(&self) -> u8 {
        self.immediate
    }
    
    pub fn set_immediate(&mut self, immediate: u8) -> Result<(), AssemblyError> {
        Self::error(immediate)?;
        
        self.immediate = immediate;
        Ok(())
    }

    pub fn set_immediate_signed(&mut self, immediate: i16) -> Result<(), AssemblyError> {
        Self::error_signed(immediate)?;
        
        self.immediate = immediate as u8;
        Ok(())
    }

    fn error(immediate: u8) -> Result<(), AssemblyError> {
        Ok(())
    }

    fn error_signed(immediate: i16) -> Result<(), AssemblyError> {
        if immediate < -128 || immediate > 255 {
            Err(AssemblyError::new(format!("Immediate {} out of range, expected -128-255", immediate)))
        } else {
            Ok(())
        }
    }
}