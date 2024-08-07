pub mod debug;
mod fpu;
mod icache;
pub mod ops;
mod registers;
mod x86;

pub use crate::x86::{CPUState, CPU, MAGIC_ADDR, X86};
pub use iced_x86::Register;
pub use registers::Flags;
