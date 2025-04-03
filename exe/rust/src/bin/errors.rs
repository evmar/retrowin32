//! Subcommands exercise various ways a program can fail.

#![no_main]
#![no_std]
#![windows_subsystem = "console"]

use exe::print::print;
use windows_sys::Win32::System::{Environment::GetCommandLineA, Threading::ExitProcess};

unsafe fn c_str(ptr: *const u8) -> &'static [u8] {
    let mut len = 0;
    while *ptr.add(len) != 0 {
        len += 1;
    }
    unsafe { core::slice::from_raw_parts(ptr, len) }
}

#[no_mangle]
pub unsafe extern "C" fn mainCRTStartup() {
    let cmd_line = c_str(GetCommandLineA());

    let space = cmd_line
        .iter()
        .position(|&c| c == b' ')
        .map(|i| i + 1)
        .unwrap_or(cmd_line.len());
    let mode = &cmd_line[space..];

    if mode.len() < 2 {
        print(b"usage: errors.exe <mode>\n");
        ExitProcess(1);
    }
    match mode {
        b"exit" => {
            print(b"expect: exit code 2\n");
            ExitProcess(2);
        }
        b"write-null" => {
            // Note: Rust appears to optimize this out if we don't print, eek.
            print(b"writing mem[0]\n");
            let ptr = 0 as *mut u32;
            *ptr = 1;
        }
        b"write-high" => {
            print(b"writing mem[0xFFFF_F000]\n");
            let ptr = 0xFFFF_F000 as *mut u32;
            *ptr = 1;
        }
        _ => {
            print(b"unknown mode: ");
            print(mode);
            print(b"\n");
            ExitProcess(1);
        }
    }
}
