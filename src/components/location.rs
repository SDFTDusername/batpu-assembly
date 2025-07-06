use crate::assembly_error::AssemblyError;
use crate::components::address::Address;
use crate::Labels;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Location {
    Address(Address),
    Offset(isize),
    Label(String)
}

impl Location {
    pub fn get_address(&self, current_address: usize, labels: &Labels) -> Result<usize, AssemblyError> {
        match self {
            Location::Address(address) => Ok(address.address() as usize),
            Location::Offset(offset) => Ok((current_address as isize + offset).rem_euclid(16_777_216) as usize),
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