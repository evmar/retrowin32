//! Support for async implementations of x86->retrowin32->x86 calls.
//!
//! The complex case is when our Rust function needs to call back into x86
//! code.  x86 emulation executes one basic block of instructions at a time, while
//! our Rust shim functions execute to completion synchronously, so the latter
//! cannot directly call the former.
//!
//! To reconcile these, we make functions that call back into x86 into "async"
//! Rust functions, that return a Future.
//!
//! 1) A given shim winapi function like IDirectDraw7::EnumDisplayModes needs
//!    to call back to x86 with each available display mode.
//! 2) To do so, we change its body to the form:
//!
//!    #[win32_derive::dllexport]
//!    async fn EnumDisplayModes(...) -> u32 {
//!      ...setup code...
//!      // Call into x86 function and await its result.
//!      shims::async_call(some_ptr, vec![args]).await;
//!      // Return to x86 caller, as before.
//!      0
//!    }
//! 3) The dllexport transform notices that async type and forwards
//!    to push_async in this module, which redirects the x86 to call
//!    async_executor next.
//! 4) async_executor picks up the Future returned in step #2 and runs it.
//!
//! Concretely when EnumDisplayModes is called, the "simple case" shim logic as
//! described above runs as before, but rather than returning to the caller
//! we instead also push a call to async_executor, which adds itself to the call stack
//! and runs the async state machine.  In the case of async_call that means the
//! x86 code eventually invoked there will return control back to async_executor.

use crate::Machine;

/// Redirect x86 control to async_executor.  Note this has particular requirements on the
/// state of the stack, and is called when a dllexport function is async.
pub fn become_async(
    machine: &mut Machine,
    future: std::pin::Pin<Box<dyn std::future::Future<Output = ()>>>,
) {
    machine.x86.cpu.regs.eip = machine.shims.async_executor;
    machine.shims.future = Some(future);
}

pub struct X86Future {
    // We know the Machine is around for the duration of the future execution.
    // https://github.com/rust-lang/futures-rs/issues/316
    m: *const Machine,
    esp: u32,
}
impl std::future::Future for X86Future {
    type Output = ();

    fn poll(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let machine = unsafe { &*self.m };
        // log::info!("poll esp:{:x} want:{:x}", machine.x86.regs.esp, self.esp);
        if machine.x86.cpu.regs.esp == self.esp {
            std::task::Poll::Ready(())
        } else {
            std::task::Poll::Pending
        }
    }
}

pub fn async_call(machine: &mut Machine, func: u32, args: Vec<u32>) -> X86Future {
    // Save original esp, as that's the marker that we use to know when the call is done.
    let esp = machine.x86.cpu.regs.esp;
    // Push the args in reverse order.
    for &arg in args.iter().rev() {
        x86::ops::push(&mut machine.x86.cpu, machine.memory.mem(), arg);
    }
    x86::ops::push(
        &mut machine.x86.cpu,
        machine.memory.mem(),
        machine.shims.async_executor,
    ); // return address
    machine.x86.cpu.regs.eip = func;
    X86Future { m: machine, esp }
}

#[allow(deref_nullptr)]
pub extern "C" fn async_executor(machine: &mut Machine, _stack_pointer: u32) -> u32 {
    if let Some(mut future) = machine.shims.future.take() {
        // TODO: we don't use the waker at all.  Rust doesn't like us passing a random null pointer
        // here but it seems like nothing accesses it(?).
        //let c = unsafe { std::task::Context::from_waker(&Waker::from_raw(std::task::RawWaker::)) };
        let context: &mut std::task::Context = unsafe { &mut *std::ptr::null_mut() };
        match future.as_mut().poll(context) {
            std::task::Poll::Ready(()) => {}
            std::task::Poll::Pending => {
                if machine.shims.future.is_some() {
                    panic!("multiple pending futures");
                }
                machine.shims.future = Some(future);
            }
        }
    }
    0
}
