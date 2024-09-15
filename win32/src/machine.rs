use crate::{host, winapi};
use std::{collections::HashMap, path::PathBuf};

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
