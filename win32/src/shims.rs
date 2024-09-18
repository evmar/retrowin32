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

use crate::Machine;

#[cfg(feature = "x86-emu")]
pub use crate::shims_emu::Shims;
#[cfg(feature = "x86-64")]
pub use crate::shims_raw::Shims;
#[cfg(feature = "x86-unicorn")]
pub use crate::shims_unicorn::Shims;

#[cfg(feature = "x86-unicorn")]
pub use crate::shims_unicorn::unicorn_loop;

pub type SyncHandler = unsafe fn(&mut Machine, u32) -> u32;
pub type AsyncHandler =
    unsafe fn(&mut Machine, u32) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>>;
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

/// Synchronously evaluate a Future, under the assumption that it is always immediately Ready.
#[allow(deref_nullptr)]
pub fn call_sync<T>(future: std::pin::Pin<&mut impl std::future::Future<Output = T>>) -> T {
    let context: &mut std::task::Context = unsafe { &mut *std::ptr::null_mut() };
    match future.poll(context) {
        std::task::Poll::Pending => unreachable!(),
        std::task::Poll::Ready(t) => t,
    }
}
