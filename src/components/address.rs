use crate::assembly_error::AssemblyError;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Address {
    address: u16
}

impl Address {
    pub fn new(address: u16) -> Result<Self, AssemblyError> {
        Self::error(address)?;
        Ok(Self { address })
    }

    pub fn address(&self) -> u16 {
        self.address
    }
    
    pub fn set_address(&mut self, address: u16) -> Result<(), AssemblyError> {
        Self::error(address)?;
        
        self.address = address;
        Ok(())
    }
    
    fn error(address: u16) -> Result<(), AssemblyError> {
        if address > 1023 {
            Err(AssemblyError::new(format!("Address {} out of range, expected 0-1023", address)))
        } else {
            Ok(())
        }
    }
}