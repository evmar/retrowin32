//! "Shims" are my word for the mechanism for x86 -> retrowin32 (and back) calls.
//!
//! This module implements Shims for non-emulated cpu case, using raw 32-bit memory.
//! See doc/x86-64.md for an overview.

use crate::{ldt::LDT, shims::Shim, Machine};

/// Wraps a region of low (32-bit) memory for us to generate code/etc. into.
struct ScratchSpace {
    ptr: *mut u8,
    len: usize,
    ofs: usize,
}

impl Default for ScratchSpace {
    fn default() -> Self {
        Self {
            ptr: std::ptr::null_mut(),
            len: 0,
            ofs: 0,
        }
    }
}

impl ScratchSpace {
    fn new(ptr: *mut u8, len: usize) -> Self {
        ScratchSpace { ptr, len, ofs: 0 }
    }

    /// Realign current write offset.  This probably doesn't matter but it makes
    /// reading the output a little easier.
    fn realign(&mut self) {
        let align = 8;
        self.ofs = self.ofs + (align - 1) & !(align - 1);
        if self.ofs > self.len {
            panic!("overflow");
        }
    }

    /// Write some data to the scratch space, returning the address it was written to.
    unsafe fn write(&mut self, buf: &[u8]) -> *mut u8 {
        let ptr = self.ptr.add(self.ofs);
        std::ptr::copy_nonoverlapping(buf.as_ptr(), ptr, buf.len());
        self.ofs += buf.len();
        if self.ofs > self.len {
            panic!("overflow");
        }
        ptr
    }

    unsafe fn write_many(&mut self, bufs: &[&[u8]]) -> *mut u8 {
        let ptr = self.write(bufs[0]);
        for buf in &bufs[1..] {
            self.write(buf);
        }
        ptr
    }
}

pub struct Shims {
    buf: ScratchSpace,

    /// Segment selector for 32-bit code.
    code32_selector: u16,

    shims: Vec<Shim>,
}

static mut MACHINE: *mut Machine = std::ptr::null_mut();
static mut STACK32: u32 = 0;
static mut STACK64: u64 = 0;

unsafe extern "C" fn call64(shim_index: u32) -> u32 {
    let machine: &mut Machine = &mut *MACHINE;
    let shim = &machine.shims.shims[shim_index as usize];
    // Stack contents:
    //   4 bytes edi +
    //   8 bytes far return address
    //   and the callee expected stack is after that.
    (shim.func)(machine, STACK32 + 12)
}

// trans64 is the code we jump to when transitioning from 32->64-bit.
// It's responsible for switching to the 64-bit stack and backing up the appropriate
// registers to transition from stdcall ABI to SysV AMD64 ABI.
// See "Calling conventions" in doc/x86-64.md; the summary is we only need to preserve
// ESI/EDI.  EDI was already saved (due to being used to pass shim_index).
#[cfg(target_arch = "x86_64")]
std::arch::global_asm!(
    "_trans64:",
    "movl %esp, {stack32}(%rip)",  // save 32-bit stack
    "movq {stack64}(%rip), %rsp",  // switch to 64-bit stack
    "pushq %rsi",                  // preserve esi
    "call {call64}",               // call 64-bit Rust
    "popq %rsi",                   // restore esi
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
    "_tramp32:",
    "calll *%eax", // regular call to user 32-bit code
    // The user 32-bit code will ret, popping off both the return address pushed by calll
    // and any stdcall args.  What's left on the stack is the far address of 64-bit mode.
    "lretl", // long ret to 64-bit mode
    options(att_syntax),
);

extern "C" {
    static tramp32: [u8; 0];
}

impl Shims {
    pub fn new(ldt: &mut LDT, addr: *mut u8, size: u32) -> Self {
        // Wine marks all of memory as code.
        let code32_selector = ldt.add_entry(0, 0xFFFF_FFFF, true);

        let buf = ScratchSpace::new(addr, size as usize);
        Shims {
            buf,
            code32_selector,
            shims: Default::default(),
        }
    }

    /// HACK: we need a pointer to the Machine, but we get it so late we have to poke it in
    /// way after all the initialization happens...
    pub unsafe fn set_machine_hack(&mut self, machine: *mut Machine, esp: u32) {
        MACHINE = machine;
        STACK32 = esp;
    }

    pub fn add(&mut self, shim: Shim) -> u32 {
        let shim_index = self.shims.len();
        self.shims.push(shim.clone());
        unsafe {
            assert!((trans64 as u64) < 0x1_0000_0000);

            // TODO revisit stack_consumed, does it include eip or not?
            // We have to -4 here to not include IP.
            let stack_consumed: u16 = shim.stack_consumed as u16 - 4;

            // trampoline_x86.s:tramp64
            let tramp_addr = self.buf.write_many(&[
                // pushl %edi
                b"\x57",
                // movl shim_index, %edi
                b"\xbf",
                &(shim_index as u32).to_le_bytes(),
                // lcalll 64_bit_selector, trans64
                b"\x9a",
                &(trans64 as u32).to_le_bytes(),
                &(0x2bu16).to_le_bytes(),
                // popl %edi
                b"\x5f",
                // retl <16-bit bytes to pop>
                b"\xc2",
                &stack_consumed.to_le_bytes(),
            ]) as u32;

            self.buf.realign();

            tramp_addr
        }
    }

    pub fn add_todo(&mut self, _name: String) -> u32 {
        // trampoline_x86.rs:crash
        unsafe { self.buf.write(b"\xcc\xb8\x01\x00\x00\x00\xff\x20") as u32 }
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

pub fn call_x86(machine: &mut Machine, func: u32, args: Vec<u32>) -> UnimplFuture {
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
        let code64_selector = 0x2b;
        STACK32 -= 4;
        mem.put::<u32>(STACK32, code64_selector);
        STACK32 -= 4;
        let return_addr = STACK32;

        // Push arguments in reverse order.
        for &arg in args.iter().rev() {
            STACK32 -= 4;
            mem.put::<u32>(STACK32, arg);
        }

        let m1632: u64 = ((machine.shims.code32_selector as u64) << 32) | (tramp32.as_ptr() as u64);

        std::arch::asm!(
            // We need to back up all non-scratch registers (rbx/rbp),
            // because even callee-saved registers will only be saved as 32-bit,
            // clobbering any 64-bit values.
            // In particular getting this wrong manifests as rbp losing its high bits after this call.
            "pushq %rbx",
            "pushq %rbp",
            "movl $2f, ({return_addr:e})", // after jmp, ret to the "2" label below
            "movq %rsp, {stack64}(%rip)",  // save 64-bit stack
            "movl {stack32}(%rip), %esp",  // switch to 32-bit stack
            "ljmpl *({m1632})",            // jump to 32-bit tramp32
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
            in("eax") func,  // passed to tramp32
            return_addr = in(reg) return_addr as u32,
            stack64 = sym STACK64,
            stack32 = sym STACK32,
            m1632 = in(reg) &m1632,
        );

        UnimplFuture {}
    }

    #[cfg(not(target_arch = "x86_64"))] // just to keep editor from getting confused
    unsafe {
        _ = machine.shims.code32_selector;
        _ = machine;
        _ = func;
        _ = args;
        _ = STACK64;
        _ = tramp32;
        call64(0);
        todo!()
    }
}
