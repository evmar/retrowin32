//! "Shims" are my word for the mechanism for x86 -> retrowin32 (and back) calls.
//! This module implements shims under the x86 emulator.
//!
//! In the simple case, Rust functions like kernel32.dll!ExitProcess call into our
//! custom kernel32.dll stub, which forwards to retrowin32_syscall.
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
//!      shims::call_x86(some_ptr, vec![args]).await;
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
//! and runs the async state machine.  In the case of call_x86 that means the
//! x86 code eventually invoked there will return control back to async_executor.

use memory::Extensions;

use crate::{machine::Machine, shims::Shim};
use std::collections::HashMap;

pub fn retrowin32_syscall() -> &'static [u8] {
    // sysenter; ret
    b"\x0f\x34\xc3".as_slice()
}

/// Jumps to memory address SHIM_BASE+x are interpreted as calling shims[x].
/// This is how emulated code calls out to hosting code for e.g. DLL imports.
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

pub fn handle_shim_call(machine: &mut Machine) {
    // The calling code looks like:
    //
    // SomeShimFn:
    //   call retrowin32_syscall
    //   ret
    // ...
    // retrowin32_syscall:
    //   syscall   <- triggered this shim call
    //   ret

    // stack looks like:
    //   return address within SomeShimFn (*1*)  <- esp
    //   return address within SomeShimFn caller (*2*)
    //   arg1 to SomeShimFn
    //   arg2 to SomeShimFn

    let regs = &mut machine.emu.x86.cpu_mut().regs;

    // Find the address of SomeShimFn by reading address marked *1* above.
    // The 'call retrowin32_syscall' instruction is ff15+addr, for 6 bytes.
    let esp = regs.get32(x86::Register::ESP);
    let call_len = 6;
    let shim_addr = machine.emu.memory.mem().get_pod::<u32>(esp) - call_len;
    let shim = match machine.emu.shims.get(shim_addr) {
        Ok(shim) => shim,
        Err(name) => unimplemented!("{}", name),
    };
    let crate::shims::Shim { func, is_async, .. } = *shim;

    // Shim fn expects to read the stack starting at address marked *2* above
    let ret = unsafe { func(machine, esp + 4) };
    if !is_async {
        let regs = &mut machine.emu.x86.cpu_mut().regs;
        regs.set32(x86::Register::EAX, ret);

        // Clear registers to make traces clean.
        // eax holds return value; other registers are callee-saved per ABI.
        regs.set32(x86::Register::ECX, 0);
        regs.set32(x86::Register::EDX, 0);
    } else {
        // async func will set up a future to handle returning.
    }
}
