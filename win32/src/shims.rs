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

use crate::{System, calling_convention::ABIReturn};
use std::collections::HashMap;

pub type SyncHandler = unsafe fn(&mut dyn System, u32) -> ABIReturn;
pub type AsyncHandler =
    unsafe fn(
        &mut dyn System,
        u32,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ABIReturn>>>;
#[derive(Debug, Clone, Copy)]
pub enum Handler {
    Sync(SyncHandler),
    Async(AsyncHandler),
}

#[derive(Debug)]
pub struct Shim {
    pub name: &'static str,
    pub func: Handler,
}

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

/// Synchronously evaluate a Future, under the assumption that it is always immediately Ready.
#[allow(deref_nullptr)]
pub fn call_sync<T>(future: std::pin::Pin<&mut impl std::future::Future<Output = T>>) -> T {
    let context: &mut std::task::Context = unsafe { &mut *std::ptr::null_mut() };
    match future.poll(context) {
        std::task::Poll::Pending => unreachable!(),
        std::task::Poll::Ready(t) => t,
    }
}
