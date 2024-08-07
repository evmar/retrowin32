//! "Shims" are my word for the mechanism for x86 -> retrowin32 (and back) calls.
//! This module implements shims under the x86 emulator.
//!
//! In the simple case, we register Rust functions like kernel32.dll!ExitProcess
//! to associate with a special invalid x86 address.  If the x86 ever jumps to such an
//! address, we forward the call to the registered shim handler.
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

use crate::shims::{BoxFuture, Handler};
use crate::{machine::Machine, shims::Shim};

/// Code that calls from x86 to the host will jump to addresses in this
/// magic range.
/// "fake IAT" => "FIAT" => "F1A7"
const SHIM_BASE: u32 = 0xF1A7_0000;

/// Jumps to memory address SHIM_BASE+x are interpreted as calling shims[x].
/// This is how emulated code calls out to hosting code for e.g. DLL imports.
#[derive(Default)]
pub struct Shims {
    shims: Vec<Result<&'static Shim, String>>,
}

impl Shims {
    /// Returns the (fake) address of the registered function.
    pub fn add(&mut self, shim: Result<&'static Shim, String>) -> u32 {
        let id = SHIM_BASE | self.shims.len() as u32;
        self.shims.push(shim);
        id
    }

    pub fn get(&self, addr: u32) -> Option<Result<&'static Shim, &str>> {
        if is_ip_at_shim_call(addr) {
            let index = (addr & 0x0000_FFFF) as usize;
            self.shims.get(index).map(|result| match result {
                Ok(shim) => Ok(*shim),
                Err(name) => Err(name.as_str()),
            })
        } else {
            None
        }
    }
}

#[inline]
pub fn is_ip_at_shim_call(ip: u32) -> bool {
    ip & 0xFFFF_0000 == SHIM_BASE
}

pub fn handle_shim_call(machine: &mut Machine, shim: &'static Shim) -> Option<BoxFuture<u32>> {
    let esp = machine.emu.x86.cpu_mut().regs.get32(x86::Register::ESP);
    match shim.func {
        Handler::Sync(func) => {
            let ret = unsafe { func(machine, esp) };
            finish_shim_call(machine, shim, ret);
            None
        }
        Handler::Async(func) => {
            let future = unsafe { func(machine, esp) };
            machine.emu.x86.cpu_mut().regs.eip = x86::MAGIC_ADDR;
            Some(future)
        }
    }
}

pub fn finish_shim_call(machine: &mut Machine, shim: &'static Shim, ret: u32) {
    let esp = machine.emu.x86.cpu_mut().regs.get32(x86::Register::ESP);
    let ret_addr = machine.emu.memory.mem().get_pod::<u32>(esp);
    let regs = &mut machine.emu.x86.cpu_mut().regs;
    regs.eip = ret_addr;
    regs.set32(x86::Register::ESP, esp + shim.stack_consumed + 4);
    regs.set32(x86::Register::EAX, ret);

    // Clear registers to make traces clean.
    // eax holds return value; other registers are callee-saved per ABI.
    regs.set32(x86::Register::ECX, 0);
    regs.set32(x86::Register::EDX, 0);
}
