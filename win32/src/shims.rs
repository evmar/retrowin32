//! "Shims" are my word for the mechanism for x86 -> retrowin32 (and back) calls.
//!
//! In the simple case, we register Rust functions like kernel32.dll!ExitProcess
//! to associate with a special invalid x86 address.  If the x86 ever jumps to such an
//! address, we forward the call to the registered shim handler.
//!
//! The win32_derive::dllexport attribute on our shim functions wraps them with
//! a prologue/epilogue that does the required stack manipulation to read
//! arguments off the x86 stack and transform them into Rust types, so the Rust
//! functions can act as if they're just being called from Rust.
//!
//! The complex case is when our Rust function needs to call back into x86
//! code.  See `future.rs` for more.

use crate::machine::Machine;

/// Code that calls from x86 to the host will jump to addresses in this
/// magic range.
/// "fake IAT" => "FIAT" => "F1A7"
pub const SHIM_BASE: u32 = 0xF1A7_0000;

pub type Handler = unsafe extern "C" fn(&mut Machine, u32) -> u32;

#[derive(Clone)]
pub struct Shim {
    pub name: &'static str,
    pub func: Handler,
    pub stack_consumed: Option<u32>,
}

/// Jumps to memory address SHIM_BASE+x are interpreted as calling shims[x].
/// This is how emulated code calls out to hosting code for e.g. DLL imports.
pub struct Shims {
    shims: Vec<Shim>,
    #[cfg(feature = "cpuemu")]
    pub async_state: crate::future::AsyncState,
}

impl Shims {
    pub fn new() -> Self {
        let mut shims = Shims {
            shims: Vec::new(),
            #[cfg(feature = "cpuemu")]
            async_state: crate::future::AsyncState::default(),
        };
        #[cfg(feature = "cpuemu")]
        {
            shims.async_state = crate::future::AsyncState::new(&mut shims);
        }
        shims
    }

    /// Returns the (fake) address of the registered function.
    pub fn add(&mut self, shim: Shim) -> u32 {
        let id = SHIM_BASE | self.shims.len() as u32;
        self.shims.push(shim);
        id
    }

    pub fn get(&self, addr: u32) -> &Shim {
        let index = (addr & 0x0000_FFFF) as usize;
        match self.shims.get(index) {
            Some(shim) => shim,
            None => panic!("unknown import reference at {:x}", addr),
        }
    }

    pub fn lookup(&self, name: &str) -> Option<u32> {
        if let Some(idx) = self.shims.iter().position(|shim| shim.name == name) {
            return Some(SHIM_BASE | idx as u32);
        }
        None
    }
}
