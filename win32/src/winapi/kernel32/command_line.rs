//! Process command line.

use super::UNICODE_STRING;
use crate::{str16::String16, winapi::alloc::Arena, Machine};
use memory::{ExtensionsMut, Mem};

/// Process command line, as exposed in GetCommandLine() and also TEB.
/// Gross: GetCommandLineA() needs to return a pointer that's never freed,
/// so we need to hang on to both versions of the command line.
pub struct CommandLine {
    /// Command line, split args.
    pub args: Vec<String>,
    /// Command line, ASCII.
    pub cmdline: u32,
    /// Command line, UTF16.
    pub cmdline16: u32,
    /// Length without trailing nul.
    pub len: usize,
}

impl CommandLine {
    pub fn new(mut cmdline: String, arena: &mut Arena, mem: Mem) -> Self {
        let args = split_cmdline(&cmdline);

        log::debug!("CommandLine: {}", cmdline);
        let len = cmdline.len();
        cmdline.push(0 as char); // nul terminator

        let cmdline8_ptr = arena.alloc(cmdline.len() as u32, 1);
        mem.sub32_mut(cmdline8_ptr, cmdline.len() as u32)
            .copy_from_slice(cmdline.as_bytes());

        let cmdline16 = String16::from(&cmdline);
        let cmdline16_ptr = arena.alloc(cmdline16.byte_size() as u32, 2);
        let mem16: &mut [u16] =
            unsafe { std::mem::transmute(mem.sub32_mut(cmdline16_ptr, cmdline16.0.len() as u32)) };
        mem16.copy_from_slice(&cmdline16.0);

        CommandLine {
            args,
            cmdline: cmdline8_ptr,
            cmdline16: cmdline16_ptr,
            len,
        }
    }

    pub fn as_unicode_string(&self) -> UNICODE_STRING {
        UNICODE_STRING {
            Length: self.len as u16,
            MaximumLength: self.len as u16,
            Buffer: self.cmdline16,
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
    machine.state.kernel32.cmdline.cmdline
}

#[win32_derive::dllexport]
pub fn GetCommandLineW(machine: &mut Machine) -> u32 {
    machine.state.kernel32.cmdline.cmdline16
}
