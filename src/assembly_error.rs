use std::cmp::Ordering;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssemblyError {
    pub description: String,
    pub address: u32
}

impl AssemblyError {
    pub fn new(description: String) -> Self {
        Self { description, address: 0 }
    }

    pub fn new_address(description: String, address: u32) -> Self {
        Self { description, address }
    }
}

impl Display for AssemblyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.address == 0 {
            write!(f, "{}", self.description)
        } else {
            write!(f, "[Address {}] {}", self.address, self.description)
        }
    }
}

impl PartialOrd for AssemblyError {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.address.partial_cmp(&other.address)
    }
}

impl Ord for AssemblyError {
    fn cmp(&self, other: &Self) -> Ordering {
        self.address.cmp(&other.address)
    }
}

impl Error for AssemblyError {}