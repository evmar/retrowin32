mod host;
mod machine;
pub mod pe;
mod reader;
pub mod shims;
pub mod trace;
mod winapi;

#[cfg(feature = "x86-emu")]
mod shims_emu;
#[cfg(feature = "x86-64")]
mod shims_raw;
#[cfg(feature = "x86-unicorn")]
mod shims_unicorn;

#[cfg(feature = "x86-64")]
mod ldt;
#[cfg(any(feature = "x86-64", feature = "x86-unicorn"))]
mod segments;

pub use host::*;
pub use machine::Machine;
#[cfg(feature = "x86-emu")]
pub use x86::debug::disassemble;

#[macro_use]
extern crate num_derive;
