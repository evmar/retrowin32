//! Process command line, as exposed in GetCommandLine() and also TEB.

use super::{init::UNICODE_STRING, state::get_state};
use memory::ExtensionsMut;
use win32_system::{System, memory::Memory};
use win32_winapi::String16;

/// Pointers returned by GetCommandLineA and GetCommandLineW.
/// Gross: GetCommandLineA() needs to return a pointer that's never freed,
/// so we need to hang on to both versions of the command line.
#[derive(Default)]
pub struct State {
    /// Command line in process memory, ASCII.
    cmdline8: u32,
    /// Command line in process memory, UTF16.
    cmdline16: u32,
}

impl State {
    fn cmdline8(&mut self, cmdline: &str, memory: &Memory) -> u32 {
        if self.cmdline8 == 0 {
            let mut cmdline = cmdline.to_string();
            cmdline.push(0 as char); // nul terminator

            let ptr = memory
                .process_heap
                .alloc(memory.mem(), cmdline.len() as u32);
            memory
                .mem()
                .sub32_mut(ptr, cmdline.len() as u32)
                .copy_from_slice(cmdline.as_bytes());
            self.cmdline8 = ptr;
        }
        self.cmdline8
    }

    fn cmdline16(&mut self, cmdline: &str, memory: &Memory) -> u32 {
        if self.cmdline16 == 0 {
            let mut cmdline16 = String16::from(cmdline);
            cmdline16.0.push(0); // nul terminator

            let ptr = memory
                .process_heap
                .alloc(memory.mem(), cmdline16.byte_size() as u32);
            memory
                .mem()
                .sub32_mut(ptr, cmdline16.byte_size() as u32)
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

    pub fn as_unicode_string(&mut self, cmdline: &str, memory: &Memory) -> UNICODE_STRING {
        let cmdline16 = self.cmdline16(cmdline, memory);
        let len = cmdline.len();
        UNICODE_STRING {
            Length: len as u16,
            MaximumLength: len as u16,
            Buffer: cmdline16,
        }
    }
}

#[win32_derive::dllexport]
pub fn GetCommandLineA(sys: &dyn System) -> u32 {
    let mut state = get_state(sys);
    state.cmdline.cmdline8(sys.command_line(), &sys.memory())
}

#[win32_derive::dllexport]
pub fn GetCommandLineW(sys: &dyn System) -> u32 {
    let mut state = get_state(sys);
    state.cmdline.cmdline16(sys.command_line(), &sys.memory())
}
