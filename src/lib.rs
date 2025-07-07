use crate::components::immediate::Immediate;
use crate::instruction::Instruction;
use std::collections::HashMap;

pub mod assembly_error;
pub mod components;
pub mod instruction;
pub mod helper;

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