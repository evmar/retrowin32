//! "Shims" are my word for the mechanism for x86 -> retrowin32 (and back) calls.
//!
//! The win32_derive::dllexport attribute on our shim functions wraps them with
//! a prologue/epilogue that does the required stack manipulation to read
//! arguments off the x86 stack and transform them into Rust types, so the Rust
//! functions can act as if they're just being called from Rust.
//!
//! There are two underlying implementations of Shims:
//! 1. shims_cpuemu.rs, which is used with the CPU emulator
//! 2. shims_raw.rs, which is used when executing x86 natively

use crate::Machine;

#[cfg(feature = "cpuemu")]
pub use crate::shims_cpuemu::{become_async, call_x86, Shims};
#[cfg(not(feature = "cpuemu"))]
pub use crate::shims_raw::{call_sync, call_x86, Shims};

pub type Handler = unsafe extern "C" fn(&mut Machine, u32) -> u32;

#[derive(Clone)]
pub struct Shim {
    pub name: &'static str,
    pub func: Handler,
    pub stack_consumed: u32,
    pub is_async: bool,
}
