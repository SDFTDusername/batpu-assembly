use crate::assembly_error::AssemblyError;
use crate::instruction::Instruction;
use std::collections::HashMap;

pub mod assembly_error;
pub mod components;
pub mod instruction;

pub type Labels = HashMap<String, usize>;

pub type InstructionVec = Vec<Instruction>;
pub type BinaryVec = Vec<u16>;

pub fn instructions_to_binary(instructions: &InstructionVec, labels: &Labels) -> Result<BinaryVec, Vec<AssemblyError>> {
    let mut errors: Vec<AssemblyError> = Vec::new();

    let binary = instructions
        .iter()
        .enumerate()
        .map(|(i, instruction)| {
            let result = instruction.binary(i, labels);
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