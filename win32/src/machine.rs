use crate::loader;
use crate::{host, winapi};
use std::collections::HashMap;

#[cfg(feature = "x86-emu")]
pub use crate::machine_emu::Machine;
#[cfg(feature = "x86-emu")]
pub use crate::machine_emu::MemImpl;
#[cfg(feature = "x86-64")]
pub use crate::machine_raw::Machine;
#[cfg(feature = "x86-64")]
pub use crate::machine_raw::MemImpl;
#[cfg(feature = "x86-unicorn")]
pub use crate::machine_unicorn::Machine;
#[cfg(feature = "x86-unicorn")]
pub use crate::machine_unicorn::MemImpl;

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
    pub external_dlls: Vec<String>,
    pub status: Status,
}

impl<Emu> MachineX<Emu> {
    /// Hackily make a null pointer, for use in tests when we know the pointer isn't needed.
    #[cfg(test)]
    pub fn null() -> &'static mut MachineX<Emu> {
        #[allow(invalid_value)]
        unsafe {
            std::mem::transmute(0usize)
        }
    }

    pub fn set_external_dlls(&mut self, dlls: &[String]) {
        self.external_dlls = dlls
            .iter()
            .map(|dll| loader::normalize_module_name(dll))
            .collect();
    }
}

/// Status of the machine/process.  Separate from CPU state because multiple threads
/// can be in different states.
#[derive(Default)]
pub enum Status {
    /// Running normally.
    #[default]
    Running,
    /// All threads are blocked awaiting results.
    Blocked,
    /// CPU error.
    Error {
        message: String,
        // TODO:
        // signal: u8
    },
    /// Hit a breakpoint.
    DebugBreak,
    /// Process exited.
    Exit(u32),
}

impl Status {
    pub fn is_running(&self) -> bool {
        matches!(self, Status::Running)
    }
}
