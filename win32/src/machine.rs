use crate::{host, shims::Shims, winapi};
use memory::{Mem, MemImpl};
use std::collections::HashMap;

#[cfg(feature = "x86-emu")]
pub use crate::machine_emu::Machine;
#[cfg(feature = "x86-64")]
pub use crate::machine_raw::Machine;
#[cfg(feature = "x86-unicorn")]
pub use crate::machine_unicorn::Machine;

pub struct LoadedAddrs {
    pub entry_point: u32,
    pub stack_pointer: u32,
}

/// Integrates the X86 CPU emulator with the Windows OS support.
pub struct MachineX<Emulator> {
    pub emu: Emulator,
    pub memory: MemImpl,
    pub host: Box<dyn host::Host>,
    pub state: winapi::State,
    pub shims: Shims,
    pub labels: HashMap<u32, String>,
}

impl<T> MachineX<T> {
    pub fn mem(&self) -> Mem {
        self.memory.mem()
    }
}
