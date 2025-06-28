use crate::assembly_error::AssemblyError;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Condition {
    Zero,
    NotZero,
    Carry,
    NotCarry
}

impl Condition {
    pub fn index(&self) -> u8 {
        match self {
            Condition::Zero => 0,
            Condition::NotZero => 1,
            Condition::Carry => 2,
            Condition::NotCarry => 3
        }
    }
    
    pub fn from_index(condition: u8) -> Result<Self, AssemblyError> {
        match condition {
            0 => Ok(Condition::Zero),
            1 => Ok(Condition::NotZero),
            2 => Ok(Condition::Carry),
            3 => Ok(Condition::NotCarry),
            _ => {
                Err(AssemblyError::new(format!("Unknown condition {} ({:#004b})", condition, condition)))
            }
        }
    }
}