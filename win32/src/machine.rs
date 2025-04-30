use win32_system::host;
use win32_system::memory::Memory;

use crate::System;
use crate::{loader, winapi};

#[cfg(feature = "x86-emu")]
pub use crate::machine_emu::Machine;
#[cfg(feature = "x86-64")]
pub use crate::machine_raw::Machine;
#[cfg(feature = "x86-unicorn")]
pub use crate::machine_unicorn::Machine;

/// Integrates the X86 CPU emulator with the Windows OS support.
pub struct MachineX<Emu> {
    pub emu: Emu,
    pub memory: Memory,
    pub host: Box<dyn host::Host>,
    pub state: winapi::State,
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

    pub fn set_external_dlls(&mut self, dlls: Vec<String>) {
        self.external_dlls = dlls
            .into_iter()
            .map(|dll| loader::normalize_module_name(&dll))
            .collect();
    }
}

impl System for Machine {
    fn mem(&self) -> memory::Mem {
        self.mem()
    }

    fn machine(&mut self) -> *mut () {
        self as *mut _ as *mut _
    }

    fn host(&mut self) -> &mut dyn host::Host {
        self.host.as_mut()
    }

    fn call_x86(
        &mut self,
        func: u32,
        args: Vec<u32>,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32> + '_>> {
        Box::pin(self.call_x86(func, args))
    }
}

/// Status of the machine/process.  Separate from CPU state because multiple threads
/// can be in different states.
#[derive(PartialEq, Eq, Default)]
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
