use crate::assembly_error::AssemblyError;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Offset {
    offset: i8
}

impl Offset {
    pub fn new(offset: i8) -> Result<Self, AssemblyError> {
        Self::error(offset)?;
        Ok(Self { offset })
    }

    pub fn offset(&self) -> i8 {
        self.offset
    }
    
    pub fn set_offset(&mut self, offset: i8) -> Result<(), AssemblyError> {
        Self::error(offset)?;
        
        self.offset = offset;
        Ok(())
    }

    fn error(offset: i8) -> Result<(), AssemblyError> {
        if offset < -8 || offset > 7 {
            Err(AssemblyError::new(format!("Offset {} out of range, expected -8-7", offset)))
        } else {
            Ok(())
        }
    }
}