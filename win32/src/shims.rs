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
//! The more complex case is when our Rust function needs to call back into x86
//! code.  x86 emulation executes one basic block of instructions at a time, while
//! our Rust shim functions execute to completion synchronously, so the latter
//! cannot directly call the former.
//!
//! To reconcile these, we hand-transform functions that call back into x86
//! into coroutines that use a closure for state.  The high-level mechanism is:
//!
//! 1) A given shim winapi function like IDirectDraw7::EnumDisplayModes needs
//!    to call back to x86 with each available display mode.
//! 2) To do so, we change its return type to the type `ShimCallback`, and its
//!    body to the form:
//!
//!    #[win32_derive::dllexport]
//!    fn EnumDisplayModes(...) -> ShimCallback {
//!      let mut state = ...;
//!      let async_fn = Box::new(move || {
//!         ...do some work based on "state", potentially update state, and
//!         return either:
//!         1) CallbackStep::Call(addr, vec![args])
//!            to mean "call the x86 code addr with given arguments and return here"
//!         2) CallbackStep::Done(ret)
//!            to mean "return to the x86 caller"
//!      });
//!      return async_fn;
//!    }
//! 3) The dllexport transform notices that ShimCallback return type and forwards
//!    to push_callback in this module, which redirects the x86 to call
//!    callback_helper next.
//! 4) callback_helper picks up the closure returned in step #2 and runs it.
//!
//! Concretely when EnumDisplayModes is called, the "simple case" shim logic as
//! described above runs as before, but rather than returning to the caller
//! we instead push a call to callback_helper, which adds itself to the call stack
//! and runs the state machine.  In the case of CallbackStep::Call that means the
//! x86 code eventually invoked there will return control back to callback_helper.
//!
//! The above omits some detail, e.g. we also allow the Rust code to put some locals
//! onto the x86 stack.

use crate::machine::Machine;
use std::collections::HashMap;

/// Code that calls from x86 to the host will jump to addresses in this
/// magic range.
/// "fake IAT" => "FIAT" => "F1A7"
pub const SHIM_BASE: u32 = 0xF1A7_0000;

struct Shim {
    name: String,
    handler: Option<fn(&mut Machine)>,
}

/// Shims that want to call back into x86 code return one of these,
/// which is a closure that carries its state across multiple returns.
pub type ShimCallback = (u32, Box<dyn FnMut(&mut Machine) -> CallbackStep>);

/// Steps within a ShimCallback can either call into an x86 address or
/// return and clean up.
#[derive(Debug)]
pub enum CallbackStep {
    /// Call first arg with arguments on stack.
    Call(u32, Vec<u32>),
    /// Done, and return argument via eax.
    Done(u32),
}

/// Jumps to memory address SHIM_BASE+x are interpreted as calling shims[x].
/// This is how emulated code calls out to hosting code for e.g. DLL imports.
pub struct Shims {
    shims: Vec<Shim>,
    /// Asynchronous callbacks, as described in the module documentation.
    /// Hash key is the value of esp when entering callback_helper.
    callback_fns: HashMap<u32, ShimCallback>,
    /// Address of callback_helper() shim entry point.
    callback_helper: u32,
}

// Drives the async state machine forward.  Note this is registered as a shim but it
// does its own management of the stack, including esp.
fn callback_helper(machine: &mut Machine) {
    let id = machine.x86.regs.esp;
    // We must remove the callback from the list of callback fns for lifetime reasons.
    let (stack_space, mut callback) = machine.shims.callback_fns.remove(&id).unwrap();
    match callback(machine) {
        CallbackStep::Call(addr, args) => {
            // Re-register the callback.
            machine
                .shims
                .callback_fns
                .insert(id, (stack_space, callback));

            // Call the provided function next.
            for &arg in args.iter().rev() {
                x86::ops::push(&mut machine.x86, arg);
            }
            x86::ops::push(&mut machine.x86, machine.shims.callback_helper); // return address
            machine.x86.regs.eip = addr;
        }
        CallbackStep::Done(ret) => {
            machine.x86.regs.esp += stack_space;
            machine.x86.regs.eax = ret;
            machine.x86.regs.eip = x86::ops::pop(&mut machine.x86);
        }
    }
}

/// Redirect x86 control to callback_helper.  Note this has particular requirements on the
/// state of the stack, and is called when a dllexport function returns a ShimCallback.
pub fn push_callback(
    machine: &mut Machine,
    return_address: u32,
    (stack_space, callback): ShimCallback,
) {
    x86::ops::push(&mut machine.x86, return_address); // where to go when we're done
    machine.x86.regs.esp -= stack_space;
    machine
        .shims
        .add_callback(machine.x86.regs.esp, (stack_space, callback));
    machine.x86.regs.eip = machine.shims.callback_helper;
}

impl Shims {
    pub fn new() -> Self {
        let mut shims = Shims {
            shims: Vec::new(),
            callback_fns: HashMap::new(),
            callback_helper: 0,
        };
        shims.callback_helper =
            shims.add("retrowin32 callback helper".into(), Some(callback_helper));
        shims
    }

    /// Returns the (fake) address of the registered function.
    pub fn add(&mut self, name: String, handler: Option<fn(&mut Machine)>) -> u32 {
        let id = SHIM_BASE | self.shims.len() as u32;
        self.shims.push(Shim { name, handler });
        id
    }

    pub fn get(&self, addr: u32) -> Option<&fn(&mut Machine)> {
        let index = (addr & 0x0000_FFFF) as usize;
        match self.shims.get(index) {
            Some(shim) => {
                if let Some(handler) = &shim.handler {
                    return Some(handler);
                }
                log::error!("unimplemented: {}", shim.name);
            }
            None => log::error!("unknown import reference at {:x}", addr),
        };
        None
    }

    pub fn lookup(&self, name: &str) -> Option<u32> {
        if let Some(idx) = self.shims.iter().position(|shim| shim.name == name) {
            return Some(SHIM_BASE | idx as u32);
        }
        None
    }

    pub fn add_callback(&mut self, id: u32, callback: ShimCallback) {
        self.callback_fns.insert(id, callback);
    }
}
