//! "Shims" are my word for the mechanism for x86 -> retrowin32 (and back) calls.
//!
//! This module implements Shims for non-emulated cpu case, using raw 32-bit memory.
//! See doc/x86-64.md for an overview.

use crate::{ldt::LDT, shims::Shim, Machine};
#[cfg(target_arch = "x86_64")]
use memory::Extensions;

type Trampoline = [u8; 16];

/// A region of memory for us to generate code/etc. into.
/// We initialize one of these structs at a low (32-bit) address.
struct ScratchSpace {
    /// 32-bit code to trampoline up to a 64-bit call to a shim, one entry per shim.
    trampolines: [Trampoline; 0x200],
}

pub struct Shims {
    scratch: &'static mut ScratchSpace,

    /// Segment selector for 64-bit code.
    code64_selector: u16,

    shims: Vec<Result<&'static Shim, String>>,
}

static mut MACHINE: *mut Machine = std::ptr::null_mut();
static mut STACK32: u32 = 0;
static mut STACK64: u64 = 0;

unsafe extern "C" fn call64() -> u32 {
    let machine: &mut Machine = &mut *MACHINE;
    // 32-bit stack contents:
    //   4 bytes return address (within scratch.trampolines)
    //   4 bytes segment selector for far return
    //   and the callee expected stack is below that.

    let ret_addr = unsafe { *(STACK32 as *const u32) };

    // Map the return address back to a shim index.
    // Though the return address points at a point after the call instruction in the
    // trampoline, dividing by the size of a trampoline here rounds down
    // to discard that offset.
    let shim_index = (ret_addr - (&machine.emu.shims.scratch.trampolines[0] as *const _ as u32))
        / std::mem::size_of::<Trampoline>() as u32;

    let shim = match &machine.emu.shims.shims[shim_index as usize] {
        Ok(shim) => shim,
        Err(name) => unimplemented!("{}", name),
    };
    (shim.func)(machine, STACK32 + 8)
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
    // clear registers to make traces clean
    // eax holds return value, leave alone
    "xorl %ecx, %ecx",
    // ebx: callee-saved
    "xorl %edx, %edx",
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

    pub fn new(teb: u32, alloc32: impl FnOnce(usize) -> u32) -> Self {
        let mut ldt = Self::init_ldt(teb);

        // Wine marks all of memory as code.
        let code32_selector = ldt.add_entry(0, 0xFFFF_FFFF, true);

        let tramp32_addr = tramp32 as u64;
        assert!(tramp32_addr < 0x1_0000_0000);
        unsafe {
            TRAMP32_M1632 = ((code32_selector as u64) << 32) | tramp32_addr;
        }

        let code64_selector = {
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
        };

        let addr = alloc32(std::mem::size_of::<ScratchSpace>());
        let scratch = unsafe { &mut *(addr as *mut ScratchSpace) };
        Shims {
            scratch,
            code64_selector,
            shims: Default::default(),
        }
    }

    /// HACK: we need a pointer to the Machine, but we get it so late we have to poke it in
    /// way after all the initialization happens...
    pub unsafe fn set_machine_hack(&mut self, machine: *mut Machine, esp: u32) {
        MACHINE = machine;
        STACK32 = esp;
    }

    pub fn add(&mut self, shim: Result<&'static Shim, String>) -> u32 {
        let stack_consumed: u16 = match shim {
            Ok(shim) => shim.stack_consumed,
            Err(_) => 0, // we'll crash when it's hit anyway
        } as u16;

        let shim_index = self.shims.len();
        self.shims.push(shim);

        assert!((trans64 as u64) < 0x1_0000_0000);
        let tramp = [
            // lcalll trans64
            b"\x9a".as_slice(),
            &(trans64 as u32).to_le_bytes(),
            &(self.code64_selector).to_le_bytes(),
            // retl
            b"\xc2",
            &stack_consumed.to_le_bytes(),
        ]
        .concat();

        self.scratch.trampolines[shim_index][..tramp.len()].copy_from_slice(&tramp);

        &self.scratch.trampolines[shim_index] as *const _ as u32
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

        let mem = machine.emu.memory.mem();

        // Push selector and reserve space for return address.
        let mut esp = STACK32;
        esp -= 4;
        mem.put_pod::<u32>(esp, machine.emu.shims.code64_selector as u32);
        esp -= 4;
        let return_addr = esp;

        // Push arguments in reverse order.
        for &arg in args.iter().rev() {
            esp -= 4;
            mem.put_pod::<u32>(esp, arg);
        }
        STACK32 = esp;

        #[allow(unused_mut)]
        let mut ret = 0;
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
