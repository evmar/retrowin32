mod host;
mod machine;
mod pe;
mod reader;
mod shims;
pub mod trace;
mod winapi;

#[cfg(feature = "cpuemu")]
mod shims_cpuemu;

#[cfg(not(feature = "cpuemu"))]
mod shims_raw;

pub use host::*;
pub use machine::Machine;
#[cfg(feature = "cpuemu")]
pub use x86::debug::disassemble;

#[macro_use]
extern crate num_derive;
