use crate::assembly_error::AssemblyError;
use crate::instruction::Instruction;
use crate::{BinaryVec, InstructionVec, Labels};

pub fn instructions_to_binary(instructions: &InstructionVec, labels: &Labels) -> Result<BinaryVec, Vec<AssemblyError>> {
    let mut errors: Vec<AssemblyError> = Vec::new();

    let binary = instructions
        .iter()
        .enumerate()
        .map(|(address, instruction)| {
            let result = instruction.binary(address as u32, labels);
            match result {
                Ok(binary) => binary,
                Err(mut error) => {
                    error.address = address as u32;
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
        .map(|(address, binary)| {
            let result = Instruction::instruction(*binary);
            match result {
                Ok(instruction) => instruction,
                Err(mut error) => {
                    error.address = address as u32;
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