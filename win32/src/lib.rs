mod host;
mod machine;
pub mod pe;
mod segments;
pub mod shims;
pub mod str16;
pub mod trace;
pub mod winapi;

#[cfg(feature = "x86-emu")]
mod machine_emu;

#[cfg(feature = "x86-64")]
mod ldt;
#[cfg(feature = "x86-64")]
mod machine_raw;
#[cfg(feature = "x86-64")]
mod shims_raw;

#[cfg(feature = "x86-unicorn")]
mod machine_unicorn;

pub use host::*;
pub use machine::{Machine, Status};
