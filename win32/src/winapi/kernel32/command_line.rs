//! Process command line.

use super::UNICODE_STRING;
use crate::{
    winapi::{alloc::Arena, String16},
    Machine,
};
use memory::{ExtensionsMut, Mem};

/// Process command line, as exposed in GetCommandLine() and also TEB.
/// Gross: GetCommandLineA() needs to return a pointer that's never freed,
/// so we need to hang on to both versions of the command line.
#[derive(Default)]
pub struct CommandLine {
    /// Command line as understood by retrowin32.
    string: String,
    /// Command line in process memory, ASCII.
    cmdline8: u32,
    /// Command line in process memory, UTF16.
    cmdline16: u32,
}

impl CommandLine {
    pub fn new(cmdline: String) -> Self {
        CommandLine {
            string: cmdline,
            cmdline8: 0,
            cmdline16: 0,
        }
    }

    fn cmdline8(&mut self, arena: &mut Arena, mem: Mem) -> u32 {
        if self.cmdline8 == 0 {
            let mut cmdline = self.string.clone();
            cmdline.push(0 as char); // nul terminator

            let ptr = arena.alloc(cmdline.len() as u32, 1);
            mem.sub32_mut(ptr, cmdline.len() as u32)
                .copy_from_slice(cmdline.as_bytes());
            self.cmdline8 = ptr;
        }
        self.cmdline8
    }

    fn cmdline16(&mut self, arena: &mut Arena, mem: Mem) -> u32 {
        if self.cmdline16 == 0 {
            let mut cmdline16 = String16::from(&self.string);
            cmdline16.0.push(0); // nul terminator

            let ptr = arena.alloc(cmdline16.byte_size() as u32, 2);
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

    pub fn exe_name(&self) -> String {
        // TODO: we only need to parse the exe name, not the arguments
        split_cmdline(&self.string).swap_remove(0)
    }

    pub fn as_unicode_string(&mut self, arena: &mut Arena, mem: Mem) -> UNICODE_STRING {
        let cmdline16 = self.cmdline16(arena, mem);
        let len = self.string.len();
        UNICODE_STRING {
            Length: len as u16,
            MaximumLength: len as u16,
            Buffer: cmdline16,
        }
    }
}

// TODO: follow the logic for CommandLineToArgvW
// https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-commandlinetoargvw
fn split_cmdline(cmdline: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut arg = String::new();
    let mut in_quote = false;
    for c in cmdline.chars() {
        match c {
            ' ' if !in_quote => {
                if !arg.is_empty() {
                    args.push(arg);
                    arg = String::new();
                }
            }
            '"' => {
                in_quote = !in_quote;
            }
            _ => {
                arg.push(c);
            }
        }
    }
    if !arg.is_empty() {
        args.push(arg);
    }
    args
}

#[win32_derive::dllexport]
pub fn GetCommandLineA(machine: &mut Machine) -> u32 {
    machine
        .state
        .kernel32
        .cmdline
        .cmdline8(&mut machine.state.kernel32.arena, machine.memory.mem())
}

#[win32_derive::dllexport]
pub fn GetCommandLineW(machine: &mut Machine) -> u32 {
    machine
        .state
        .kernel32
        .cmdline
        .cmdline16(&mut machine.state.kernel32.arena, machine.memory.mem())
}
