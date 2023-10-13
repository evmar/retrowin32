mod host;
mod machine;
pub mod pe;
mod reader;
pub mod shims;
pub mod trace;
mod winapi;

#[cfg(feature = "x86-emu")]
mod shims_emu;

#[cfg(not(feature = "x86-emu"))]
mod shims_raw;

#[cfg(not(feature = "x86-emu"))]
mod ldt;

pub use host::*;
pub use machine::Machine;
#[cfg(feature = "x86-emu")]
pub use x86::debug::disassemble;

#[macro_use]
extern crate num_derive;
