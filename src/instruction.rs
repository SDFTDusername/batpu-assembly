use crate::assembly_error::AssemblyError;
use crate::components::address::Address;
use crate::components::condition::Condition;
use crate::components::immediate::Immediate;
use crate::components::location::Location;
use crate::components::offset::Offset;
use crate::components::register::Register;
use crate::components::{address, condition, immediate, offset, register};
use crate::{opcode, Binary, Labels};

pub const BITS: u32 = size_of::<Binary>() as u32 * 8;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instruction {
    NoOperation,
    Halt,
    Addition(Register, Register, Register),
    Subtraction(Register, Register, Register),
    BitwiseNOR(Register, Register, Register),
    BitwiseAND(Register, Register, Register),
    BitwiseXOR(Register, Register, Register),
    RightShift(Register, Register),
    LoadImmediate(Register, Immediate),
    AddImmediate(Register, Immediate),
    Jump(Location),
    Branch(Condition, Location),
    Call(Location),
    Return,
    MemoryLoad(Register, Register, Offset),
    MemoryStore(Register, Register, Offset)
}

impl Instruction {
    pub fn index(&self) -> u32 {
        match self {
            Instruction::NoOperation            => 0,
            Instruction::Halt                   => 1,
            Instruction::Addition(_, _, _)      => 2,
            Instruction::Subtraction(_, _, _)   => 3,
            Instruction::BitwiseNOR(_, _, _)    => 4,
            Instruction::BitwiseAND(_, _, _)    => 5,
            Instruction::BitwiseXOR(_, _, _)    => 6,
            Instruction::RightShift(_, _)       => 7,
            Instruction::LoadImmediate(_, _)    => 8,
            Instruction::AddImmediate(_, _)     => 9,
            Instruction::Jump(_)                => 10,
            Instruction::Branch(_, _)           => 11,
            Instruction::Call(_)                => 12,
            Instruction::Return                 => 13,
            Instruction::MemoryLoad(_, _, _)    => 14,
            Instruction::MemoryStore(_, _, _)   => 15
        }
    }
    
    pub fn binary(&self, address: u32, labels: &Labels) -> Result<Binary, AssemblyError> {
        let mut binary: Binary = 0;
        binary |= ((self.index() & opcode::MASK) << (BITS - opcode::BITS)) as Binary;

        let add_register = |binary: &mut Binary, register: &Register, index: u32| {
            *binary |= ((register.register() & register::MASK) << (register::BITS * index)) as Binary;
        };

        let add_immediate = |binary: &mut Binary, immediate: &Immediate| {
            *binary |= (immediate.immediate() & immediate::MASK) as Binary;
        };

        let add_location = |binary: &mut Binary, location: &Location| -> Result<(), AssemblyError> {
            let address = location.get_address(address, labels)?;
            *binary |= (address & address::MASK) as Binary;
            Ok(())
        };

        let add_condition = |binary: &mut Binary, condition: &Condition| {
            *binary |= ((condition.index() & condition::MASK) << (BITS - opcode::BITS - condition::BITS)) as Binary;
        };

        let add_offset = |binary: &mut Binary, offset: &Offset| {
            *binary |= (offset.offset() as u32 & offset::MASK) as Binary;
        };

        match self {
            Instruction::Addition(a, b, c) => {
                add_register(&mut binary, a, 2);
                add_register(&mut binary, b, 1);
                add_register(&mut binary, c, 0);
            },
            Instruction::Subtraction(a, b, c) => {
                add_register(&mut binary, a, 2);
                add_register(&mut binary, b, 1);
                add_register(&mut binary, c, 0);
            },
            Instruction::BitwiseNOR(a, b, c) => {
                add_register(&mut binary, a, 2);
                add_register(&mut binary, b, 1);
                add_register(&mut binary, c, 0);
            },
            Instruction::BitwiseAND(a, b, c) => {
                add_register(&mut binary, a, 2);
                add_register(&mut binary, b, 1);
                add_register(&mut binary, c, 0);
            },
            Instruction::BitwiseXOR(a, b, c) => {
                add_register(&mut binary, a, 2);
                add_register(&mut binary, b, 1);
                add_register(&mut binary, c, 0);
            },
            Instruction::RightShift(a, c) => {
                add_register(&mut binary, a, 2);
                add_register(&mut binary, c, 0);
            },
            Instruction::LoadImmediate(a, immediate) => {
                add_register(&mut binary, a, 2);
                add_immediate(&mut binary, immediate);
            },
            Instruction::AddImmediate(a, immediate) => {
                add_register(&mut binary, a, 2);
                add_immediate(&mut binary, immediate);
            },
            Instruction::Jump(location) => {
                add_location(&mut binary, location)?;
            },
            Instruction::Branch(condition, location) => {
                add_condition(&mut binary, condition);
                add_location(&mut binary, location)?;
            }
            Instruction::Call(location) => {
                add_location(&mut binary, location)?;
            }
            Instruction::MemoryLoad(a, b, offset) => {
                add_register(&mut binary, a, 2);
                add_register(&mut binary, b, 1);
                add_offset(&mut binary, offset);
            },
            Instruction::MemoryStore(a, b, offset) => {
                add_register(&mut binary, a, 2);
                add_register(&mut binary, b, 1);
                add_offset(&mut binary, offset);
            },
            _ => {}
        }
        
        Ok(binary)
    }

    pub fn instruction(binary: Binary) -> Result<Instruction, AssemblyError> {
        let opcode = (binary >> (BITS - opcode::BITS)) as u32 & opcode::MASK;

        let get_register = |binary: Binary, index: u32| -> Result<Register, AssemblyError> {
            let register = (binary as u32 >> (register::BITS * index)) & register::MASK;
            Register::new(register)
        };

        let get_immediate = |binary: Binary| -> Immediate {
            let immediate = binary as u32 & immediate::MASK;
            Immediate::new(immediate)
        };

        let get_location = |binary: Binary| -> Result<Location, AssemblyError> {
            let address = binary as u32 & address::MASK;
            let address = Address::new(address)?;
            Ok(Location::Address(address))
        };

        let get_condition = |binary: Binary| -> Result<Condition, AssemblyError> {
            let condition = (binary as u32 >> (BITS - opcode::BITS - condition::BITS)) & condition::MASK;
            Condition::from_index(condition)
        };

        let get_offset = |binary: Binary| -> Result<Offset, AssemblyError> {
            let offset = binary as u32 & offset::MASK;
            let offset_signed = (offset << 16) as i32 >> 16;
            Offset::new(offset_signed)
        };

        let instruction = match opcode {
            0 => { // NoOperation
                Instruction::NoOperation
            },
            1 => { // Halt
                Instruction::Halt
            },
            2 => { // Addition
                Instruction::Addition(
                    get_register(binary, 2)?,
                    get_register(binary, 1)?,
                    get_register(binary, 0)?
                )
            },
            3 => { // Subtraction
                Instruction::Subtraction(
                    get_register(binary, 2)?,
                    get_register(binary, 1)?,
                    get_register(binary, 0)?
                )
            },
            4 => { // BitwiseNOR
                Instruction::BitwiseNOR(
                    get_register(binary, 2)?,
                    get_register(binary, 1)?,
                    get_register(binary, 0)?
                )
            },
            5 => { // BitwiseAND
                Instruction::BitwiseAND(
                    get_register(binary, 2)?,
                    get_register(binary, 1)?,
                    get_register(binary, 0)?
                )
            },
            6 => { // BitwiseXOR
                Instruction::BitwiseXOR(
                    get_register(binary, 2)?,
                    get_register(binary, 1)?,
                    get_register(binary, 0)?
                )
            },
            7 => { // RightShift
                Instruction::RightShift(
                    get_register(binary, 2)?,
                    get_register(binary, 0)?
                )
            },
            8 => { // LoadImmediate
                Instruction::LoadImmediate(
                    get_register(binary, 2)?,
                    get_immediate(binary)
                )
            },
            9 => { // AddImmediate
                Instruction::AddImmediate(
                    get_register(binary, 2)?,
                    get_immediate(binary)
                )
            },
            10 => { // Jump
                Instruction::Jump(
                    get_location(binary)?
                )
            },
            11 => { // Branch
                Instruction::Branch(
                    get_condition(binary)?,
                    get_location(binary)?
                )
            },
            12 => { // Call
                Instruction::Call(
                    get_location(binary)?
                )
            },
            13 => { // Return
                Instruction::Return
            },
            14 => { // MemoryLoad
                Instruction::MemoryLoad(
                    get_register(binary, 2)?,
                    get_register(binary, 1)?,
                    get_offset(binary)?
                )
            },
            15 => { // MemoryStore
                Instruction::MemoryStore(
                    get_register(binary, 2)?,
                    get_register(binary, 1)?,
                    get_offset(binary)?
                )
            },
            _ => {
                return Err(AssemblyError::new(format!("Unknown opcode {} ({:#bits$b})", opcode, opcode, bits=opcode::BITS as usize + 2)))
            }
        };

        Ok(instruction)
    }
}