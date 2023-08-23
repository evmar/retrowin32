mod host;
mod machine;
pub mod pe;
mod reader;
pub mod shims;
pub mod trace;
mod winapi;

#[cfg(feature = "cpuemu")]
mod shims_cpuemu;

#[cfg(not(feature = "cpuemu"))]
mod shims_raw;

#[cfg(not(feature = "cpuemu"))]
mod ldt;

pub use host::*;
pub use machine::Machine;
#[cfg(feature = "cpuemu")]
pub use x86::debug::disassemble;

#[macro_use]
extern crate num_derive;
