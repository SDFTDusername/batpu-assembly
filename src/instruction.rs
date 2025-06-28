use crate::assembly_error::AssemblyError;
use crate::components::address::Address;
use crate::components::condition::Condition;
use crate::components::immediate::Immediate;
use crate::components::location::Location;
use crate::components::offset::Offset;
use crate::components::register::Register;
use crate::Labels;

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
    pub fn index(&self) -> u8 {
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
    
    pub fn binary(&self, labels: &Labels) -> Result<u16, AssemblyError> {
        let mut binary: u16 = 0;
        binary |= (self.index() as u16 & 0b1111) << 12;

        match self {
            Instruction::Addition(a, b, c) => {
                binary |= (a.register() as u16 & 0b1111) << 8;
                binary |= (b.register() as u16 & 0b1111) << 4;
                binary |= c.register() as u16 & 0b1111;
            },
            Instruction::Subtraction(a, b, c) => {
                binary |= (a.register() as u16 & 0b1111) << 8;
                binary |= (b.register() as u16 & 0b1111) << 4;
                binary |= c.register() as u16 & 0b1111;
            },
            Instruction::BitwiseNOR(a, b, c) => {
                binary |= (a.register() as u16 & 0b1111) << 8;
                binary |= (b.register() as u16 & 0b1111) << 4;
                binary |= c.register() as u16 & 0b1111;
            },
            Instruction::BitwiseAND(a, b, c) => {
                binary |= (a.register() as u16 & 0b1111) << 8;
                binary |= (b.register() as u16 & 0b1111) << 4;
                binary |= c.register() as u16 & 0b1111;
            },
            Instruction::BitwiseXOR(a, b, c) => {
                binary |= (a.register() as u16 & 0b1111) << 8;
                binary |= (b.register() as u16 & 0b1111) << 4;
                binary |= c.register() as u16 & 0b1111;
            },
            Instruction::RightShift(a, c) => {
                binary |= (a.register() as u16 & 0b1111) << 8;
                binary |= c.register() as u16 & 0b1111;
            },
            Instruction::LoadImmediate(a, immediate) => {
                binary |= (a.register() as u16 & 0b1111) << 8;
                binary |= immediate.immediate() as u16 & 0b1111_1111;
            },
            Instruction::AddImmediate(a, immediate) => {
                binary |= (a.register() as u16 & 0b1111) << 8;
                binary |= immediate.immediate() as u16 & 0b1111_1111;
            },
            Instruction::Jump(label) => {
                let address = label.get_address(labels)?;
                binary |= address as u16 & 0b11_1111_1111;
            },
            Instruction::Branch(condition, label) => {
                binary |= (condition.index() as u16 & 0b11) << 10;

                let address = label.get_address(labels)?;
                binary |= address as u16 & 0b11_1111_1111;
            },
            Instruction::Call(label) => {
                let address = label.get_address(labels)?;
                binary |= address as u16 & 0b11_1111_1111;
            },
            Instruction::MemoryLoad(a, b, offset) => {
                binary |= (a.register() as u16 & 0b1111) << 8;
                binary |= (b.register() as u16 & 0b1111) << 4;
                binary |= offset.offset() as u16 & 0b1111;
            },
            Instruction::MemoryStore(a, b, offset) => {
                binary |= (a.register() as u16 & 0b1111) << 8;
                binary |= (b.register() as u16 & 0b1111) << 4;
                binary |= offset.offset() as u16 & 0b1111;
            },
            _ => {}
        }
        
        Ok(binary)
    }

    pub fn instruction(binary: u16) -> Result<Instruction, AssemblyError> {
        let opcode = (binary >> 12) & 0b1111;

        let instruction = match opcode {
            0 => { // NoOperation
                Instruction::NoOperation
            },
            1 => { // Halt
                Instruction::Halt
            },
            2 => { // Addition
                Instruction::Addition(
                    Register::new(((binary >> 8) & 0b1111) as u8),
                    Register::new(((binary >> 4) & 0b1111) as u8),
                    Register::new((binary & 0b1111) as u8)
                )
            },
            3 => { // Subtraction
                Instruction::Subtraction(
                    Register::new(((binary >> 8) & 0b1111) as u8),
                    Register::new(((binary >> 4) & 0b1111) as u8),
                    Register::new((binary & 0b1111) as u8)
                )
            },
            4 => { // BitwiseNOR
                Instruction::BitwiseNOR(
                    Register::new(((binary >> 8) & 0b1111) as u8),
                    Register::new(((binary >> 4) & 0b1111) as u8),
                    Register::new((binary & 0b1111) as u8)
                )
            },
            5 => { // BitwiseAND
                Instruction::BitwiseAND(
                    Register::new(((binary >> 8) & 0b1111) as u8),
                    Register::new(((binary >> 4) & 0b1111) as u8),
                    Register::new((binary & 0b1111) as u8)
                )
            },
            6 => { // BitwiseXOR
                Instruction::BitwiseXOR(
                    Register::new(((binary >> 8) & 0b1111) as u8),
                    Register::new(((binary >> 4) & 0b1111) as u8),
                    Register::new((binary & 0b1111) as u8)
                )
            },
            7 => { // RightShift
                Instruction::RightShift(
                    Register::new(((binary >> 8) & 0b1111) as u8),
                    Register::new((binary & 0b1111) as u8)
                )
            },
            8 => { // LoadImmediate
                Instruction::LoadImmediate(
                    Register::new(((binary >> 8) & 0b1111) as u8),
                    Immediate::new((binary & 0b1111_1111) as u8)
                )
            },
            9 => { // AddImmediate
                Instruction::AddImmediate(
                    Register::new(((binary >> 8) & 0b1111) as u8),
                    Immediate::new((binary & 0b1111_1111) as u8)
                )
            },
            10 => { // Jump
                Instruction::Jump(
                    Location::Address(Address::new(binary & 0b11_1111_1111))
                )
            },
            11 => { // Branch
                Instruction::Branch(
                    Condition::from_index(((binary >> 10) & 0b11) as u8)?,
                    Location::Address(Address::new(binary & 0b11_1111_1111))
                )
            },
            12 => { // Call
                Instruction::Call(
                    Location::Address(Address::new(binary & 0b11_1111_1111))
                )
            },
            13 => { // Return
                Instruction::Return
            },
            14 => { // MemoryLoad
                Instruction::MemoryLoad(
                    Register::new(((binary >> 8) & 0b1111) as u8),
                    Register::new(((binary >> 4) & 0b1111) as u8),
                    Offset::new(((((binary & 0b1111) as u8) << 4) as i8) >> 4)
                )
            },
            15 => { // MemoryStore
                Instruction::MemoryStore(
                    Register::new(((binary >> 8) & 0b1111) as u8),
                    Register::new(((binary >> 4) & 0b1111) as u8),
                    Offset::new(((((binary & 0b1111) as u8) << 4) as i8) >> 4)
                )
            },
            _ => {
                return Err(AssemblyError::new(format!("Unknown opcode {} ({:#006b})", opcode, opcode)))
            }
        };

        Ok(instruction)
    }
}