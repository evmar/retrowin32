//! "Shims" are my word for the mechanism for x86 -> retrowin32 (and back) calls.
//!
//! The win32_derive::dllexport attribute on our shim functions wraps them with
//! a prologue/epilogue that does the required stack manipulation to read
//! arguments off the x86 stack and transform them into Rust types, so the Rust
//! functions can act as if they're just being called from Rust.
//!
//! There are three underlying implementations of Shims:
//! 1. shims_emu.rs, which is used with the in-tree CPU emulator
//! 2. shims_raw.rs, which is used when executing x86 natively
//! 3. shims_unicorn.rs, which is used with the Unicorn CPU emulator

use builtin_kernel32 as kernel32;
use kernel32::loader;
use memory::ExtensionsMut;
use std::collections::HashMap;
use win32_system::{dll::Shim, memory::Memory};

#[derive(Default)]
pub struct Shims {
    shims: HashMap<u32, Result<&'static Shim, String>>,
}

impl Shims {
    pub fn register(&mut self, addr: u32, shim: Result<&'static Shim, String>) {
        self.shims.insert(addr, shim);
    }

    pub fn get(&self, addr: u32) -> Result<&Shim, &str> {
        match self.shims.get(&addr) {
            Some(Ok(shim)) => Ok(shim),
            Some(Err(name)) => Err(name),
            None => panic!("unknown import reference at {:x}", addr),
        }
    }
}

/// Return the loader::Module for the magic retrowin32.dll module, which
/// provides the retrowin32_syscall function needed for x86<->win32 calls.
pub fn retrowin32_dll_module(memory: &Memory, retrowin32_syscall: &[u8]) -> loader::Module {
    // Always put the syscall stub at the same address,
    // so different emulators match on memory layout.
    assert!(retrowin32_syscall.len() <= 8);
    let addr = 0x1000 - 8;
    memory
        .mem()
        .sub32_mut(addr, retrowin32_syscall.len() as u32)
        .copy_from_slice(retrowin32_syscall);
    let names = HashMap::from([("retrowin32_syscall".into(), addr)]);
    let exports = loader::Exports {
        names,
        ..Default::default()
    };

    loader::Module {
        name: "retrowin32.dll".into(),
        // use a non-zero base address so it doesn't register as the null HMODULE
        image_base: 1,
        exports,
        ..Default::default() // rest of fields unused
    }
}
