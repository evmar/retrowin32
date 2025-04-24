pub mod debug;
mod fpu;
mod icache;
pub mod ops;
mod registers;
mod x86;

pub use crate::x86::{CPU, CPUState, X86};
pub use iced_x86::Register;
pub use ops::set_edx_eax;
