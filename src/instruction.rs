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
    Multiplication(Register, Register, Register),
    Division(Register, Register, Register),
    
    BitwiseNOR(Register, Register, Register),
    BitwiseAND(Register, Register, Register),
    BitwiseXOR(Register, Register, Register),
    
    RightShift(Register, Register, Register),
    LeftShift(Register, Register, Register),
    
    LoadImmediate(Register, Immediate),
    AddImmediate(Register, Immediate),
    
    Jump(Location),
    Branch(Condition, Location),
    Call(Location),
    Return,
    
    MemoryLoad(Register, Register, Offset),
    MemoryStore(Register, Register, Offset),
    
    PushStack(Register),
    PopStack(Register)
}

impl Instruction {
    pub fn index(&self) -> u8 {
        match self {
            Instruction::NoOperation             => 0,
            Instruction::Halt                    => 1,
            
            Instruction::Addition(_, _, _)       => 2,
            Instruction::Subtraction(_, _, _)    => 3,
            Instruction::Multiplication(_, _, _) => 4,
            Instruction::Division(_, _, _)       => 5,
            
            Instruction::BitwiseNOR(_, _, _)     => 6,
            Instruction::BitwiseAND(_, _, _)     => 7,
            Instruction::BitwiseXOR(_, _, _)     => 8,
            
            Instruction::RightShift(_, _, _)     => 9,
            Instruction::LeftShift(_, _, _)      => 10,
            
            Instruction::LoadImmediate(_, _)     => 11,
            Instruction::AddImmediate(_, _)      => 12,
            
            Instruction::Jump(_)                 => 13,
            Instruction::Branch(_, _)            => 14,
            Instruction::Call(_)                 => 15,
            Instruction::Return                  => 16,
            
            Instruction::MemoryLoad(_, _, _)     => 17,
            Instruction::MemoryStore(_, _, _)    => 18,
            
            Instruction::PushStack(_)            => 19,
            Instruction::PopStack(_)             => 20
        }
    }
    
    pub fn binary(&self, address: usize, labels: &Labels) -> Result<u32, AssemblyError> {
        let mut binary: u32 = 0;
        binary |= (self.index() as u32 & 0b1_1111) << 27;

        match self {
            Instruction::Addition(a, b, c) => {
                binary |= (a.register() as u32) << 16;
                binary |= (b.register() as u32) << 8;
                binary |= c.register() as u32;
            },
            Instruction::Subtraction(a, b, c) => {
                binary |= (a.register() as u32) << 16;
                binary |= (b.register() as u32) << 8;
                binary |= c.register() as u32;
            },
            Instruction::Multiplication(a, b, c) => {
                binary |= (a.register() as u32) << 16;
                binary |= (b.register() as u32) << 8;
                binary |= c.register() as u32;
            },
            Instruction::Division(a, b, c) => {
                binary |= (a.register() as u32) << 16;
                binary |= (b.register() as u32) << 8;
                binary |= c.register() as u32;
            },
            Instruction::BitwiseNOR(a, b, c) => {
                binary |= (a.register() as u32) << 16;
                binary |= (b.register() as u32) << 8;
                binary |= c.register() as u32;
            },
            Instruction::BitwiseAND(a, b, c) => {
                binary |= (a.register() as u32) << 16;
                binary |= (b.register() as u32) << 8;
                binary |= c.register() as u32;
            },
            Instruction::BitwiseXOR(a, b, c) => {
                binary |= (a.register() as u32) << 16;
                binary |= (b.register() as u32) << 8;
                binary |= c.register() as u32;
            },
            Instruction::RightShift(a, b, c) => {
                binary |= (a.register() as u32) << 16;
                binary |= (b.register() as u32) << 8;
                binary |= c.register() as u32;
            },
            Instruction::LeftShift(a, b, c) => {
                binary |= (a.register() as u32) << 16;
                binary |= (b.register() as u32) << 8;
                binary |= c.register() as u32;
            },
            Instruction::LoadImmediate(a, immediate) => {
                binary |= (a.register() as u32) << 16;
                binary |= immediate.immediate() as u32 & 0b1111_1111_1111_1111;
            },
            Instruction::AddImmediate(a, immediate) => {
                binary |= (a.register() as u32) << 16;
                binary |= immediate.immediate() as u32 & 0b1111_1111_1111_1111;
            },
            Instruction::Jump(label) => {
                let address = label.get_address(address, labels)?;
                binary |= address as u32 & 0b1111_1111_1111_1111_1111_1111;
            },
            Instruction::Branch(condition, label) => {
                binary |= (condition.index() as u32 & 0b111) << 24;

                let address = label.get_address(address, labels)?;
                binary |= address as u32 & 0b1111_1111_1111_1111_1111_1111;
            },
            Instruction::Call(label) => {
                let address = label.get_address(address, labels)?;
                binary |= address as u32 & 0b1111_1111_1111_1111_1111_1111;
            },
            Instruction::MemoryLoad(a, b, offset) => {
                binary |= (a.register() as u32) << 16;
                binary |= (b.register() as u32) << 8;
                binary |= offset.offset() as u32 & 0b1111_1111;
            },
            Instruction::MemoryStore(a, b, offset) => {
                binary |= (a.register() as u32) << 16;
                binary |= (b.register() as u32) << 8;
                binary |= offset.offset() as u32 & 0b1111_1111;
            },
            Instruction::PushStack(a) => {
                binary |= (a.register() as u32) << 16;
            },
            Instruction::PopStack(a) => {
                binary |= (a.register() as u32) << 16;
            },
            _ => {}
        }
        
        Ok(binary)
    }

    pub fn instruction(binary: u32) -> Result<Instruction, AssemblyError> {
        let opcode = (binary >> 27) & 0b1_1111;

        let instruction = match opcode {
            0 => { // NoOperation
                Instruction::NoOperation
            },
            1 => { // Halt
                Instruction::Halt
            },
            2 => { // Addition
                Instruction::Addition(
                    Register::new(((binary >> 16) & 0b1111_1111) as u8),
                    Register::new(((binary >> 8) & 0b1111_1111) as u8),
                    Register::new((binary & 0b1111_1111) as u8)
                )
            },
            3 => { // Subtraction
                Instruction::Subtraction(
                    Register::new(((binary >> 16) & 0b1111_1111) as u8),
                    Register::new(((binary >> 8) & 0b1111_1111) as u8),
                    Register::new((binary & 0b1111_1111) as u8)
                )
            },
            4 => { // Multiplication
                Instruction::Multiplication(
                    Register::new(((binary >> 16) & 0b1111_1111) as u8),
                    Register::new(((binary >> 8) & 0b1111_1111) as u8),
                    Register::new((binary & 0b1111_1111) as u8)
                )
            },
            5 => { // Division
                Instruction::Division(
                    Register::new(((binary >> 16) & 0b1111_1111) as u8),
                    Register::new(((binary >> 8) & 0b1111_1111) as u8),
                    Register::new((binary & 0b1111_1111) as u8)
                )
            },
            6 => { // BitwiseNOR
                Instruction::BitwiseNOR(
                    Register::new(((binary >> 16) & 0b1111_1111) as u8),
                    Register::new(((binary >> 8) & 0b1111_1111) as u8),
                    Register::new((binary & 0b1111_1111) as u8)
                )
            },
            7 => { // BitwiseAND
                Instruction::BitwiseAND(
                    Register::new(((binary >> 16) & 0b1111_1111) as u8),
                    Register::new(((binary >> 8) & 0b1111_1111) as u8),
                    Register::new((binary & 0b1111_1111) as u8)
                )
            },
            8 => { // BitwiseXOR
                Instruction::BitwiseXOR(
                    Register::new(((binary >> 16) & 0b1111_1111) as u8),
                    Register::new(((binary >> 8) & 0b1111_1111) as u8),
                    Register::new((binary & 0b1111_1111) as u8)
                )
            },
            9 => { // RightShift
                Instruction::RightShift(
                    Register::new(((binary >> 16) & 0b1111_1111) as u8),
                    Register::new(((binary >> 8) & 0b1111_1111) as u8),
                    Register::new((binary & 0b1111_1111) as u8)
                )
            },
            10 => { // LeftShift
                Instruction::LeftShift(
                    Register::new(((binary >> 16) & 0b1111_1111) as u8),
                    Register::new(((binary >> 8) & 0b1111_1111) as u8),
                    Register::new((binary & 0b1111_1111) as u8)
                )
            },
            11 => { // LoadImmediate
                Instruction::LoadImmediate(
                    Register::new(((binary >> 16) & 0b1111_1111) as u8),
                    Immediate::new((binary & 0b1111_1111_1111_1111) as u16)
                )
            },
            12 => { // AddImmediate
                Instruction::AddImmediate(
                    Register::new(((binary >> 16) & 0b1111_1111) as u8),
                    Immediate::new((binary & 0b1111_1111_1111_1111) as u16)
                )
            },
            13 => { // Jump
                Instruction::Jump(
                    Location::Address(Address::new(binary & 0b1111_1111_1111_1111_1111_1111))
                )
            },
            14 => { // Branch
                Instruction::Branch(
                    Condition::from_index(((binary >> 24) & 0b111) as u8)?,
                    Location::Address(Address::new(binary & 0b1111_1111_1111_1111_1111_1111))
                )
            },
            15 => { // Call
                Instruction::Call(
                    Location::Address(Address::new(binary & 0b1111_1111_1111_1111_1111_1111))
                )
            },
            16 => { // Return
                Instruction::Return
            },
            17 => { // MemoryLoad
                Instruction::MemoryLoad(
                    Register::new(((binary >> 16) & 0b1111_1111) as u8),
                    Register::new(((binary >> 8) & 0b1111_1111) as u8),
                    Offset::new(((((binary & 0b1111_1111) as u8) << 4) as i8) >> 4)
                )
            },
            18 => { // MemoryStore
                Instruction::MemoryStore(
                    Register::new(((binary >> 16) & 0b1111_1111) as u8),
                    Register::new(((binary >> 8) & 0b1111_1111) as u8),
                    Offset::new(((((binary & 0b1111_1111) as u8) << 4) as i8) >> 4)
                )
            },
            19 => { // PushStack
                Instruction::PushStack(
                    Register::new(((binary >> 16) & 0b1111_1111) as u8)
                )
            },
            20 => { // PopStack
                Instruction::PopStack(
                    Register::new(((binary >> 16) & 0b1111_1111) as u8)
                )
            },
            _ => {
                return Err(AssemblyError::new(format!("Unknown opcode {} ({:#006b})", opcode, opcode)))
            }
        };

        Ok(instruction)
    }
}