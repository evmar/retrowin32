mod host;
mod machine;
mod pe;
mod reader;
mod shims;
pub mod trace;
mod winapi;

#[cfg(feature = "cpuemu")]
mod future_cpuemu;
#[cfg(feature = "cpuemu")]
use future_cpuemu as future;

#[cfg(not(feature = "cpuemu"))]
mod future_raw;
#[cfg(not(feature = "cpuemu"))]
use future_raw as future;

pub use host::*;
pub use machine::Machine;
#[cfg(feature = "cpuemu")]
pub use x86::debug::disassemble;

#[macro_use]
extern crate num_derive;
