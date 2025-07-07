use crate::assembly_error::AssemblyError;
use crate::components::address;
use crate::components::address::Address;
use crate::components::offset::Offset;
use crate::Labels;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Location {
    Address(Address),
    Offset(Offset),
    Label(String)
}

impl Location {
    pub fn get_address(&self, current_address: u32, labels: &Labels) -> Result<u32, AssemblyError> {
        match self {
            Location::Address(address) => Ok(address.address()),
            Location::Offset(offset) => {
                let address_signed = current_address as i32 + offset.offset();
                let address = address_signed.rem_euclid(address::MAX_POSSIBLE_COUNT as i32) as u32;
                Ok(address)
            },
            Location::Label(label) => {
                let result = labels.get(label);
                match result {
                    Some(value) => Ok(*value),
                    None => Err(AssemblyError::new(format!("Unknown label \"{}\"", label)).into())
                }
            }
        }
    }
}