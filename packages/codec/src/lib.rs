extern crate core;

pub mod encode;
pub mod decode;
pub mod message;

pub type EnumNumber = u16;
pub type FieldNumber = u8;
pub const FIELD_NUMBER_LEN: usize = 2;
