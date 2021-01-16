// #![no_std]
#![feature(core_intrinsics)]
#![feature(untagged_unions)]

pub mod function;
pub mod instruction;
pub mod module_system;
pub mod registers;
pub mod state;
pub mod table;
pub mod utils;
pub mod value;

// #[cfg(feature = "test")]
pub mod test;
