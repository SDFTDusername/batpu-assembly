use crate::assembly_error::AssemblyError;
use crate::components::immediate::Immediate;
use crate::instruction::Instruction;
use std::collections::HashMap;

pub mod assembly_error;
pub mod components;
pub mod instruction;

pub type Labels = HashMap<String, Immediate>;

pub type InstructionVec = Vec<Instruction>;

pub type Binary = u16;
pub type BinaryVec = Vec<Binary>;

#[macro_export] macro_rules! bit_consts {
    ($bits:expr) => {
        pub const BITS: u32 = $bits;
        pub const MASK: u32 = 1 << BITS - 1;
        
        pub const MAX_POSSIBLE_COUNT: u32 = 2_u32.pow(BITS);
        pub const MAX_VALUE: u32 = MAX_POSSIBLE_COUNT - 1;
    };
}

#[macro_export] macro_rules! signed_bit_consts {
    ($bits:expr) => {
        bit_consts!($bits);
        
        pub const MAX_SIGNED_VALUE: i32 = MAX_VALUE as i32 / 2;
        pub const MIN_SIGNED_VALUE: i32 = -MAX_SIGNED_VALUE - 1;
    };
}

pub fn instructions_to_binary(instructions: &InstructionVec, labels: &Labels) -> Result<BinaryVec, Vec<AssemblyError>> {
    let mut errors: Vec<AssemblyError> = Vec::new();

    let binary = instructions
        .iter()
        .enumerate()
        .map(|(i, instruction)| {
            let result = instruction.binary(i as u32, labels);
            match result {
                Ok(binary) => binary,
                Err(mut error) => {
                    error.line = i + 1;
                    errors.push(error);
                    0
                }
            }
        })
        .collect();

    if !errors.is_empty() {
        return Err(errors);
    }

    Ok(binary)
}

pub fn binary_to_instructions(binary: &BinaryVec) -> Result<InstructionVec, Vec<AssemblyError>> {
    let mut errors: Vec<AssemblyError> = Vec::new();

    let instructions = binary
        .iter()
        .enumerate()
        .map(|(i, binary)| {
            let result = Instruction::instruction(*binary);
            match result {
                Ok(instruction) => instruction,
                Err(mut error) => {
                    error.line = i + 1;
                    errors.push(error);
                    Instruction::NoOperation
                }
            }
        })
        .collect();

    if !errors.is_empty() {
        return Err(errors);
    }

    Ok(instructions)
}