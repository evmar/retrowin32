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
pub use crate::shims_emu::{become_async, block};
#[cfg(feature = "x86-unicorn")]
pub use crate::shims_unicorn::unicorn_loop;

pub type Handler = unsafe fn(&mut Machine, u32) -> u32;

pub struct Shim {
    pub name: &'static str,
    pub func: Handler,
    /// Number of stack bytes popped by arguments.
    /// For cdecl calling convention (used in varargs) this is 0.
    pub stack_consumed: u32,
    pub is_async: bool,
}

pub struct UnimplFuture {}
impl std::future::Future for UnimplFuture {
    type Output = ();

    fn poll(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        std::task::Poll::Ready(())
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
