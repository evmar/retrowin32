//! "Shims" are the word used within retrowin32 for the mechanism for x86 ->
//! retrowin32 (and back) calls.  See doc/shims.md.
//!
//! This module implements Shims for non-emulated cpu case, using raw 32-bit memory.
//! See doc/x86-64.md for an overview.

use crate::{
    ldt::LDT,
    shims::{Handler, Shims},
    Machine,
};
#[cfg(target_arch = "x86_64")]
use memory::ExtensionsMut;

static mut MACHINE: *mut Machine = std::ptr::null_mut();
static mut STACK32: u32 = 0;
static mut STACK64: u64 = 0;

pub unsafe fn set_stack32(esp: u32) {
    STACK32 = esp;
}

unsafe extern "C" fn call64() -> u64 {
    let machine: &mut Machine = &mut *MACHINE;

    // call sequence:
    //   exe:
    //     call WindowsFunc
    //   dll!WindowsFunc:
    //     call retrowin32_syscall
    //   retrowin32_syscall:
    //     lcall trans64

    // 32-bit stack contents:
    //   stack[0]: return address in retrowin32_syscall
    //   stack[1]: segment selector for far return
    //   stack[2]: return address in dll!WindowsFunc
    //   stack[3]: return address in exe
    //   stack[4]: first arg to WindowsFunc

    let stack32 = STACK32 as *const u32;
    let ret_addr = unsafe { *stack32.offset(2) };
    let shim = match machine.emu.shims.get(ret_addr - 6) {
        Ok(shim) => shim,
        Err(name) => unimplemented!("{}", name),
    };
    let stack_args = STACK32 + 16; // stack[4]
    match shim.func {
        Handler::Sync(func) => func(machine, stack_args),
        Handler::Async(_) => unimplemented!(),
    }
}

// trans64 is the code we jump to when transitioning from 32->64-bit.
// It's responsible for switching to the 64-bit stack and backing up the appropriate
// registers to transition from stdcall ABI to SysV AMD64 ABI.
// See "Calling conventions" in doc/x86-64.md; the summary is we only need to preserve
// ESI/EDI.
#[cfg(target_arch = "x86_64")]
std::arch::global_asm!(
    "_trans64:",
    "movl %esp, {stack32}(%rip)",  // save 32-bit stack
    "movq {stack64}(%rip), %rsp",  // switch to 64-bit stack
    "pushq %rdi",                  // preserve edi
    "pushq %rsi",                  // preserve esi
    "call {call64}",               // call 64-bit Rust
    // After call, attempt to clear registers to make execution traces easier to match.
    // eax: holds return value
    "xorl %ecx, %ecx",
    // edx: sometimes used for 64-bit returns
    // ebx: callee-saved
    "popq %rsi",                   // restore esi
    "popq %rdi",                   // restore edi
    // ebp: callee-saved
    "movq %rsp, {stack64}(%rip)",  // save 64-bit stack
    "movl {stack32}(%rip), %esp",  // restore 32-bit stack
    "lret",                        // back to 32-bit
    options(att_syntax),
    stack32 = sym STACK32,
    stack64 = sym STACK64,
    call64 = sym call64,
);

extern "C" {
    fn trans64();
}

// Target for 64->32bit transition.
// Takes function to call in eax.
#[cfg(target_arch = "x86_64")]
std::arch::global_asm!(
    ".code32", // 32-bit x86 code
    ".global _tramp32",
    "_tramp32:",
    "calll *%eax", // regular call to user 32-bit code
    // The user 32-bit code will ret, popping off both the return address pushed by calll
    // and any stdcall args.  What's left on the stack is the far address of 64-bit mode.
    "lretl", // long ret to 64-bit mode
    options(att_syntax),
);

extern "C" {
    fn tramp32();
}

/// A known m16:32 selector+address for the tramp32 function.
static mut TRAMP32_M1632: u64 = 0;

/// Get the segment selector for 64-bit mode,
/// which in other words is the current code segment value.
fn get_code64_selector() -> u16 {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        let mut cs: u16;
        std::arch::asm!(
            "movw %cs,{out:x}",
            out = out(reg) cs,
            options(att_syntax)
        );
        cs
    }
    #[cfg(not(target_arch = "x86_64"))]
    0u16
}

pub fn retrowin32_syscall() -> Vec<u8> {
    [
        // lcalll trans64
        b"\x9a".as_slice(),
        &(trans64 as u32).to_le_bytes(),
        &(get_code64_selector()).to_le_bytes(),
        // retl
        b"\xc3",
    ]
    .concat()
}

impl Shims {
    fn init_ldt(teb: u32) -> LDT {
        let mut ldt = LDT::default();

        // NOTE: OSX seems extremely sensitive to the values used here, where like
        // using a span size that is not exactly 0xFFF causes the entry to be rejected.
        let fs_sel = ldt.add_entry(teb, 0xFFF, false);
        unsafe {
            std::arch::asm!(
                "mov fs,{fs_sel:x}",
                fs_sel = in(reg) fs_sel
            );
        }

        // Rosetta doesn't appear to care about ds/es, but native x86 needs them.
        let sel = ldt.add_entry(0, 0xFFFF_FFFF, false);
        unsafe {
            std::arch::asm!(
                "mov ds,{sel:x}",
                "mov es,{sel:x}",
                sel = in(reg) sel,
            );
        }

        ldt
    }

    pub fn new(teb: u32) -> Self {
        let mut ldt = Self::init_ldt(teb);

        // Wine marks all of memory as code.
        let code32_selector = ldt.add_entry(0, 0xFFFF_FFFF, true);

        let tramp32_addr = tramp32 as u64;
        assert!(tramp32_addr < 0x1_0000_0000);
        unsafe {
            TRAMP32_M1632 = ((code32_selector as u64) << 32) | tramp32_addr;
        }

        Shims::default()
    }

    /// HACK: we need a pointer to the Machine, but we get it so late we have to poke it in
    /// way after all the initialization happens...
    pub unsafe fn set_machine_hack(&mut self, machine: *mut Machine) {
        MACHINE = machine;
    }
}

pub async fn call_x86(machine: &mut Machine, func: u32, args: Vec<u32>) -> u32 {
    // TODO: x86_64-apple-darwin vs x86_64-pc-windows-msvc calling conventions differ!
    #[cfg(target_arch = "x86_64")]
    unsafe {
        // To jump between 64/32 we need to stash some m16:32 pointers, and in particular to
        // be able to return to our 64-bit RIP we put it on the stack and lret to it.
        //
        // So we lay out the 32-bit stack like this before going into assembly:
        //   return address (tramp32ret)
        //   arg0
        //   ...
        //   argN
        //   64-bit return address
        //   64-bit segment selector
        //
        // lcall tramp32 pushes m16:32 to the saved spot,
        // and then tramp32 switches esp to point to the top of this stack.
        // When tramp32 returns it pops the m16:32.

        let mem = machine.memory.mem();

        // Push selector and reserve space for return address.
        let mut esp = STACK32;
        esp -= 4;
        mem.put_pod::<u32>(esp, get_code64_selector() as u32);
        esp -= 4;
        let return_addr = esp;

        // Push arguments in reverse order.
        for &arg in args.iter().rev() {
            esp -= 4;
            mem.put_pod::<u32>(esp, arg);
        }
        STACK32 = esp;

        let mut ret;
        std::arch::asm!(
            // We need to back up all non-scratch registers (rbx/rbp),
            // because even callee-saved registers will only be saved as 32-bit,
            // clobbering any 64-bit values.
            // In particular getting this wrong manifests as rbp losing its high bits after this call.
            "pushq %rbx",
            "pushq %rbp",
            "movl $2f, (%rcx)",            // after jmp, ret to the "2" label below
            "movq %rsp, {stack64}(%rip)",  // save 64-bit stack
            "movl {stack32}(%rip), %esp",  // switch to 32-bit stack
            "xorl %ebx, %ebx",
            "xorl %ecx, %ecx",
            "ljmpl *{tramp32_m1632}(%rip)",            // jump to 32-bit tramp32
            // It will return here (set above in return_addr):
            "2:",
            "movl %esp, {stack32}(%rip)",  // save 32-bit stack
            "movq {stack64}(%rip), %rsp",  // restore 64-bit stack
            "popq %rbp",
            "popq %rbx",
            options(att_syntax),
            // A 32-bit call may call back into 64-bit Rust code, so it may clobber
            // 64-bit registers.  Mark this code as if it's a 64-bit call.
            clobber_abi("system"),

            // We try to clear all registers so that traces line up across invocations,
            // so mark each one as clobbered and use them above explicitly.
            inout("eax") func => ret,  // passed to tramp32
            inout("ecx") return_addr as u32 => _,
            // ebx is preserved/restored
            inout("edx") 0 => _,
            inout("esi") 0 => _,
            inout("edi") 0 => _,
            // ebp is preserved/restored
            // esp is preserved/restored
            tramp32_m1632 = sym TRAMP32_M1632,
            stack64 = sym STACK64,
            stack32 = sym STACK32,
        );

        ret
    }

    #[cfg(not(target_arch = "x86_64"))] // just to keep editor from getting confused
    unsafe {
        _ = machine;
        _ = func;
        _ = args;
        _ = STACK64;
        _ = tramp32;
        call64();
        todo!()
    }
}
