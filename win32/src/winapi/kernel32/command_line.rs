//! Process command line, as exposed in GetCommandLine() and also TEB.

use super::{arena::Arena, init::UNICODE_STRING};
use crate::Machine;
use memory::{ExtensionsMut, Mem};
use win32_winapi::String16;

/// Pointers returned by GetCommandLineA and GetCommandLineW.
/// Gross: GetCommandLineA() needs to return a pointer that's never freed,
/// so we need to hang on to both versions of the command line.
#[derive(Default)]
pub struct CommandLineState {
    /// Command line in process memory, ASCII.
    cmdline8: u32,
    /// Command line in process memory, UTF16.
    cmdline16: u32,
}

impl CommandLineState {
    fn cmdline8(&mut self, cmdline: &str, arena: &mut Arena, mem: Mem) -> u32 {
        if self.cmdline8 == 0 {
            let mut cmdline = cmdline.to_string();
            cmdline.push(0 as char); // nul terminator

            let ptr = arena.alloc(cmdline.len() as u32);
            mem.sub32_mut(ptr, cmdline.len() as u32)
                .copy_from_slice(cmdline.as_bytes());
            self.cmdline8 = ptr;
        }
        self.cmdline8
    }

    fn cmdline16(&mut self, cmdline: &str, arena: &mut Arena, mem: Mem) -> u32 {
        if self.cmdline16 == 0 {
            let mut cmdline16 = String16::from(cmdline);
            cmdline16.0.push(0); // nul terminator

            let ptr = arena.alloc(cmdline16.byte_size() as u32);
            mem.sub32_mut(ptr, cmdline16.byte_size() as u32)
                .copy_from_slice(unsafe {
                    std::slice::from_raw_parts::<u8>(
                        cmdline16.0.as_ptr() as *const _,
                        cmdline16.byte_size(),
                    )
                });
            self.cmdline16 = ptr;
        }
        self.cmdline16
    }

    pub fn as_unicode_string(
        &mut self,
        cmdline: &str,
        arena: &mut Arena,
        mem: Mem,
    ) -> UNICODE_STRING {
        let cmdline16 = self.cmdline16(cmdline, arena, mem);
        let len = cmdline.len();
        UNICODE_STRING {
            Length: len as u16,
            MaximumLength: len as u16,
            Buffer: cmdline16,
        }
    }
}

#[win32_derive::dllexport]
pub fn GetCommandLineA(machine: &mut Machine) -> u32 {
    machine.state.kernel32.cmdline.cmdline8(
        &machine.process.cmdline.string,
        &mut machine.state.kernel32.arena,
        machine.memory.mem(),
    )
}

#[win32_derive::dllexport]
pub fn GetCommandLineW(machine: &mut Machine) -> u32 {
    machine.state.kernel32.cmdline.cmdline16(
        &machine.process.cmdline.string,
        &mut machine.state.kernel32.arena,
        machine.memory.mem(),
    )
}
