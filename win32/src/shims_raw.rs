//! "Shims" are the word used within retrowin32 for the mechanism for x86 ->
//! retrowin32 (and back) calls.  See doc/shims.md.
//!
//! This module implements Shims for non-emulated cpu case, using raw 32-bit memory.
//! See doc/x86-64.md for an overview.

use crate::{Machine, shims::Shims};
use builtin_kernel32 as kernel32;
use memory::{Extensions, ExtensionsMut, Mem};
use win32_system::dll::Handler;
use win32_winapi::calling_convention::ABIReturn;

static mut MACHINE: *mut Machine = std::ptr::null_mut();

/// The desired state of registers when entering 32-bit mode.
#[repr(C)]
#[derive(Debug)]
pub struct Context32 {
    pub esp: u32,
    pub ds: u16,
    pub fs: u16,
}
static mut CONTEXT32: Context32 = Context32 {
    esp: 0,
    ds: 0,
    fs: 0,
};

/// The desired state of registers when returning to 64-bit mode.
#[repr(C)]
#[derive(Debug)]
pub struct Context64 {
    pub rsp: u64,
    pub ds: u16,
    pub fs: u16,
}
static mut CONTEXT64: Context64 = Context64 {
    rsp: 0,
    ds: 0,
    fs: 0,
};

pub unsafe fn init_context32(mem: Mem, thread: &kernel32::thread::NewThread) {
    let ds = 0x2b;
    unsafe {
        let fs = init_fs(mem, thread.stack_pointer, thread.thread.teb);
        CONTEXT32 = Context32 {
            esp: thread.stack_pointer,
            ds,
            fs,
        };
    }
    #[allow(static_mut_refs)]
    unsafe {
        log::info!("c32 {:x?}", CONTEXT32);
    }
}

unsafe extern "C" fn call64() -> u64 {
    unsafe {
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

        let stack32 = CONTEXT32.esp as *const u32;
        let ret_addr = *stack32.offset(2);
        let shim = match machine.emu.shims.get(ret_addr - 6) {
            Ok(shim) => shim,
            Err(name) => unimplemented!("{}", name),
        };
        let stack_args = CONTEXT32.esp + 16; // stack[4]
        let ret = match shim.func {
            Handler::Sync(func) => func(machine, stack_args),
            Handler::Async(func) => call_sync(func(machine, stack_args).as_mut()),
        };
        match ret {
            ABIReturn::U32(ret) => ret as u64,
            ABIReturn::U64(_) => todo!(),
            ABIReturn::F64(_) => todo!(),
        }
    }
}

// 64-bit code target for 32->64bit transition.
// It's responsible for switching to the 64-bit stack and backing up the appropriate
// registers to transition from stdcall ABI to SysV AMD64 ABI.
// See "Calling conventions" in doc/x86-64.md; the summary is we only need to preserve
// ESI/EDI.
#[cfg(target_arch = "x86_64")]
std::arch::global_asm!(
    "trans64:",
    "_trans64:", // label needs _ prefix on Mac, no prefix on Linux

    "movl %esp, {context32}(%rip)",  // save 32-bit stack
    "leaq {context64}(%rip), %rdx",
    "movq 0x0(%rdx), %rsp", // CONTEXT64.rsp
    "movw 0x8(%rdx), %ds",  // CONTEXT64.ds
    "movw 0xa(%rdx), %fs",  // CONTEXT64.fs

    // Preserve callee-saved registers.
    "pushq %rdi",
    "pushq %rsi",

    "call {call64}", // call 64-bit Rust

    // After call, attempt to clear registers to make execution traces easier to match.
    // eax: holds return value
    "xorl %ecx, %ecx",
    // edx: sometimes used for 64-bit returns
    // ebx: callee-saved
    "popq %rsi",
    "popq %rdi",
    // ebp: callee-saved

    "movq %rsp, {context64}(%rip)",  // save 64-bit stack
    "movl {context32}(%rip), %esp",  // restore 32-bit stack
    "lret",                        // back to 32-bit
    options(att_syntax),
    context32 = sym CONTEXT32,
    context64 = sym CONTEXT64,
    call64 = sym call64,
);

unsafe extern "C" {
    fn trans64();
}

// 32-bit code target for 64->32bit transition.
// Takes function to call in eax.
#[cfg(target_arch = "x86_64")]
std::arch::global_asm!(
    ".code32", // 32-bit x86 code
    ".global trans32",
    ".global _trans32", // symbol needs _ prefix on Mac, no prefix on Linux
    "trans32:",
    "_trans32:",
    "movl %ebp, %esp", // STACK32 passed from caller in ebp
    "calll *%eax",     // regular call to user 32-bit code
    // The user 32-bit code will ret, popping off both the return address pushed by calll
    // and any stdcall args.  What's left on the stack is the far address of 64-bit mode.
    "lretl", // long ret to 64-bit mode
    options(att_syntax),
);

unsafe extern "C" {
    fn trans32();
}

/// A known m16:32 selector+address for the trans32 function.
static mut TRANS32_M1632: u64 = 0;

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
    pub fn init() {
        #[cfg(target_os = "linux")]
        let code32_selector = {
            let index = 4; // GDT_ENTRY_DEFAULT_USER32_CS in the kernel
            (index << 3) | 0b011
        };
        #[cfg(not(target_os = "linux"))]
        let code32_selector = crate::ldt::init();

        let trans32_addr = trans32 as u64;
        assert!(trans32_addr < 0x1_0000_0000);
        unsafe {
            TRANS32_M1632 = ((code32_selector as u64) << 32) | trans32_addr;
        }
    }

    /// HACK: we need a pointer to the Machine, but we get it so late we have to poke it in
    /// way after all the initialization happens...
    pub unsafe fn set_machine_hack(&mut self, machine: *mut Machine) {
        unsafe {
            MACHINE = machine;
        }
    }
}

/// Set up a segment selector fo the FS segment.
/// Returns a segment selector that points at fs_addr.
fn init_fs(mem: Mem, scratch_addr: u32, fs_addr: u32) -> u16 {
    // see alloc_fs_sel in wine

    #[repr(C)]
    #[allow(non_camel_case_types)]
    #[derive(Debug, Default)]
    struct user_desc {
        entry_number: u32,
        base_addr: u32,
        limit: u32,
        /*
        unsigned int  seg_32bit:1;
        unsigned int  contents:2;
        unsigned int  read_exec_only:1;
        unsigned int  limit_in_pages:1;
        unsigned int  seg_not_present:1;
        unsigned int  useable:1;
        */
        flags: u8,
    }
    unsafe impl ::memory::Pod for user_desc {}

    let gdt_index = unsafe {
        // Call syscall(SYS_set_thread_area) to allocate a GDT entry for the FS segment.

        // let SYS_get_thread_area = 0xf4u32;
        #[allow(non_snake_case)]
        let SYS_set_thread_area = 0xf3u32;

        // We need to pass a user_desc* to the syscall, and it needs to
        // be within the 32-bit address space.
        //
        // TODO: wine actually adjusts esp before making this syscall, but it seems to me
        // that the kernel already has its own separate stack?
        let user_desc_addr = scratch_addr - std::mem::size_of::<user_desc>() as u32;
        mem.put_pod::<user_desc>(
            user_desc_addr,
            user_desc {
                entry_number: -1i32 as u32, // -1 means auto-allocate
                base_addr: fs_addr,
                limit: 0xFFF,
                flags: 0b0100_0001, // useable | 32bit
            },
        );

        let mut ret: u32;
        std::arch::asm!(
            "push rbx",
            "mov ebx, {addr:e}",
            "int 0x80", // 32-bit syscall
            "pop rbx",
            inout("eax") SYS_set_thread_area => ret,
            addr = in(reg) user_desc_addr,
        );
        if ret != 0 {
            let err = std::io::Error::last_os_error();
            panic!("SYS_set_thread_area failed: {err}");
        }

        let user_desc = mem.get_pod::<user_desc>(user_desc_addr);
        log::info!("set_thread_area: {user_desc:#x?}");
        user_desc.entry_number as u16
    };
    if gdt_index != 12 {
        // See Linux kernel /arch/x86/include/asm/segment.h, definition of GS_TLS_SEL.
        // They are hardcoded values.
        panic!("expected GDT entry for FS to be 12, got {gdt_index}");
    }
    (gdt_index << 3) | 0b011 // selector: index << 3 | RPL
}

pub async fn call_x86(machine: &mut Machine, func: u32, args: Vec<u32>) -> u32 {
    // TODO: x86_64-apple-darwin vs x86_64-pc-windows-msvc calling conventions differ!
    #[cfg(target_arch = "x86_64")]
    unsafe {
        debug_assert!(TRANS32_M1632 != 0);

        // To jump between 64/32 we need to stash some m16:32 pointers, and in particular to
        // be able to return to our 64-bit RIP we put it on the stack and lret to it.
        //
        // So we lay out the 32-bit stack like this before calling the executable:
        //   return address (in trans32)
        //   arg0
        //   ...
        //   argN
        //   return address (in here)  \_ these two are the m16:32 far return address
        //   segment selector          /
        //
        // lcall trans32 pushes m16:32 to the saved spot,
        // and then trans32 switches esp to point to the top of this stack.
        // When trans32 returns it pops the m16:32.

        let mem = machine.memory.mem();

        let initial_esp = CONTEXT32.esp;
        CONTEXT32.esp -= 8; // Space for m16:32 return address

        // Push arguments in reverse order.
        for &arg in args.iter().rev() {
            CONTEXT32.esp -= 4;
            mem.put_pod::<u32>(CONTEXT32.esp, arg);
        }

        let mut ret;
        std::arch::asm!(
            // We need to back up all non-scratch registers (rbx/rbp),
            // because even callee-saved registers will only be saved as 32-bit,
            // clobbering any 64-bit values.
            // In particular getting this wrong manifests as rbp losing its high bits after this call.
            "pushq %rbx",
            "pushq %rbp",

            "movq %rsp, {context64}(%rip)",  // save 64-bit stack
            "movl %ecx, %esp",               // switch to 32-bit stack

            "leaq {context32}(%rip), %rdx",
            "movl (%rdx), %ebp", // CONTEXT32.esp
            "movw 4(%rdx), %ds", // CONTEXT32.ds
            "movw 6(%rdx), %fs", // CONTEXT32.fs

            // Clear registers to make execution traces easier to match.
            // eax: parameter to trans32
            "xorl %ecx, %ecx",
            "xorl %edx, %edx",
            "xorl %ebx, %ebx",
            "xorl %esi, %esi",
            "xorl %edi, %edi",
            // esp: set for trans32
            // ebp: parameter to trans32

            "lcall *{trans32_m1632}(%rip)", // call 32-bit trans32

            "movl %esp, {context32}(%rip)",  // save 32-bit stack

            "leaq {context64}(%rip), %rdx",
            "movq (%rdx), %rsp",   // CONTEXT64.rsp
            "movw 0xa(%rdx), %fs", // CONTEXT64.fs

            "popq %rbp",
            "popq %rbx",
            options(att_syntax),

            // A 32-bit call may call back into 64-bit Rust code, so it may clobber
            // 64-bit registers.  Mark this code as if it's a 64-bit call.
            clobber_abi("system"),

            // We try to clear all registers so that traces line up across invocations,
            // so mark each one as clobbered and use them above explicitly.
            inout("eax") func => ret,  // passed to trans32
            inout("ecx") initial_esp as u32 => _,
            out("edx") _,
            // ebx is preserved/restored
            out("esi") _,
            out("edi") _,
            // ebp is preserved/restored
            // esp is preserved/restored
            trans32_m1632 = sym TRANS32_M1632,
            context64 = sym CONTEXT64,
            context32 = sym CONTEXT32,
        );

        ret
    }

    #[cfg(not(target_arch = "x86_64"))] // just to keep editor from getting confused
    unsafe {
        _ = machine;
        _ = func;
        _ = args;
        _ = CONTEXT64;
        _ = trans32;
        call64();
        todo!()
    }
}

/// Synchronously evaluate a Future, under the assumption that it is always immediately Ready.
#[allow(deref_nullptr)]
pub fn call_sync<T, F>(future: std::pin::Pin<&mut F>) -> T
where
    F: std::future::Future<Output = T> + ?Sized,
{
    let mut context = std::task::Context::from_waker(futures::task::noop_waker_ref());
    match future.poll(&mut context) {
        std::task::Poll::Pending => unreachable!(),
        std::task::Poll::Ready(t) => t,
    }
}
