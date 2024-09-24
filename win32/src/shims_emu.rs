//! "Shims" are the word used within retrowin32 for the mechanism for x86 ->
//! retrowin32 (and back) calls.  See doc/shims.md.
//!
//! This module implements shims under the x86 emulator.

use memory::Extensions;

use crate::{machine::Machine, shims::Handler};

pub fn retrowin32_syscall() -> &'static [u8] {
    // sysenter; ret
    b"\x0f\x34\xc3".as_slice()
}

pub fn handle_shim_call(machine: &mut Machine) {
    // The calling code looks like:
    //
    // foo.dll!Foo:
    //   call retrowin32_syscall
    //   ret
    // ...
    // retrowin32_syscall:
    //   syscall   <- triggered this shim call
    //   ret

    // stack looks like:
    //   return address within foo.dll!Foo (*1*)  <- esp
    //   return address within foo.exe (*2*)
    //   arg1 to Foo
    //   arg2 to Foo

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
    // Shim fn expects to read the stack starting at address marked *2* above
    match shim.func {
        Handler::Sync(func) => {
            let ret = unsafe { func(machine, esp + 4) };
            let regs = &mut machine.emu.x86.cpu_mut().regs;
            regs.set32(x86::Register::EAX, ret);

            // Clear registers to make traces clean.
            // eax holds return value; other registers are callee-saved per ABI.
            regs.set32(x86::Register::ECX, 0);
            regs.set32(x86::Register::EDX, 0);
        }

        Handler::Async(func) => {
            let eip = regs.eip; // return address
            let future = unsafe { func(machine, esp + 4) };
            machine.emu.x86.cpu_mut().call_async(future, eip);
        }
    }
}
