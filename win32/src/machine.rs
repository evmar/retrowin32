use crate::shims::Shim;
use crate::{host, winapi};
use std::collections::HashMap;
use std::path::PathBuf;

#[cfg(feature = "x86-emu")]
pub use crate::machine_emu::{Machine, MemImpl};
#[cfg(feature = "x86-64")]
pub use crate::machine_raw::{Machine, MemImpl};
#[cfg(feature = "x86-unicorn")]
pub use crate::machine_unicorn::{Machine, MemImpl};

pub struct LoadedAddrs {
    pub entry_point: u32,
    pub stack_pointer: u32,
}

/// Integrates the X86 CPU emulator with the Windows OS support.
pub struct MachineX<Emu> {
    pub emu: Emu,
    pub host: Box<dyn host::Host>,
    pub state: winapi::State,
    pub labels: HashMap<u32, String>,
    pub exe_path: PathBuf,
}

pub enum StopReason {
    /// The CPU is running normally.
    None,
    /// The CPU is blocked polling a future.
    Blocked,
    /// The CPU hit a debug breakpoint.
    Breakpoint { eip: u32 },
    /// The CPU hit a shim call.
    ShimCall(&'static Shim),
    /// The CPU encountered an error.
    Error {
        message: String,
        signal: u8,
        eip: u32,
    },
    /// The CPU exited with a status code.
    Exit { code: u32 },
}
