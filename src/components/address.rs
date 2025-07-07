use crate::assembly_error::AssemblyError;
use crate::bit_consts;

bit_consts!(10);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Address {
    address: u32
}

impl Address {
    pub fn new(address: u32) -> Result<Self, AssemblyError> {
        Self::error(address)?;
        Ok(Self { address })
    }

    pub fn address(&self) -> u32 {
        self.address
    }
    
    pub fn set_address(&mut self, address: u32) -> Result<(), AssemblyError> {
        Self::error(address)?;
        
        self.address = address;
        Ok(())
    }
    
    fn error(address: u32) -> Result<(), AssemblyError> {
        if address > MAX_VALUE {
            Err(AssemblyError::new(format!("Address {} out of range, expected 0-{}", address, MAX_VALUE)))
        } else {
            Ok(())
        }
    }
}