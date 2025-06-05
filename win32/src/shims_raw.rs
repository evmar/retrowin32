//! "Shims" are the word used within retrowin32 for the mechanism for x86 ->
//! retrowin32 (and back) calls.  See doc/shims.md.
//!
//! This module implements Shims for non-emulated cpu case, using raw 32-bit memory.
//! See doc/x86-64.md for an overview.

// Don't complain when we println!(..., STATIC.foo) in an unsafe block.
#![allow(static_mut_refs)]

use crate::{Machine, shims::Shims};
use builtin_kernel32 as kernel32;
use memory::{Extensions, ExtensionsMut, Mem};
use win32_system::dll::Handler;
use win32_winapi::calling_convention::ABIReturn;

static mut MACHINE: *mut Machine = std::ptr::null_mut();

/// The desired CPU state when entering 32-bit mode.
#[repr(C)]
#[derive(Debug)]
pub struct Context32 {
    pub esp: u32,
    pub ds: u16,
    pub fs: u16,
    /// Used for Windows APIs that require a pointer to the TEB.
    #[cfg(target_os = "linux")]
    pub fs_addr: u32,
}
static mut CONTEXT32: Context32 = Context32 {
    esp: 0,
    ds: 0,
    fs: 0,
    #[cfg(target_os = "linux")]
    fs_addr: 0,
};

/// The desired CPU state when returning to 64-bit mode.
#[repr(C)]
#[derive(Debug)]
pub struct Context64 {
    pub rsp: u64,
    #[cfg(target_os = "linux")]
    pub fs_addr: u64,
    // Segment registers in 64-bit mode are always 0.
}
static mut CONTEXT64: Context64 = Context64 {
    rsp: 0,
    #[cfg(target_os = "linux")]
    fs_addr: 0,
};

pub unsafe fn init_thread(mem: Mem, thread: &kernel32::thread::NewThread) {
    #[cfg(not(target_os = "linux"))]
    {
        // Set up fs to point at the TEB.
        // NOTE: OSX seems extremely sensitive to the values used here, where like
        // using a span size that is not exactly 0xFFF causes the entry to be rejected.
        let fs = crate::ldt::add_entry(thread.thread.teb, 0xFFF, false);
        std::arch::asm!(
            "mov fs, {fs:x}",
            fs = in(reg) fs
        );
    }

    #[cfg(target_os = "linux")]
    {
        let fs_sel = init_fs(mem, thread.stack_pointer - 0x1000, thread.thread.teb as u32);
        unsafe {
            CONTEXT32.esp = thread.stack_pointer;
            CONTEXT32.fs_addr = thread.thread.teb;
            CONTEXT32.fs = fs_sel;
        }
    }
}

unsafe extern "C" fn call64() -> u64 {
    unsafe {
        let machine: &mut Machine = &mut *MACHINE;

        // 32-bit call sequence:
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

        set_fs_addr_64(CONTEXT64.fs_addr as u64);
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
#[cfg(target_arch = "x86_64")]
std::arch::global_asm!(
    "trans64:",
    "_trans64:", // label needs _ prefix on Mac, no prefix on Linux

     // 64-bit segment registers: all 0
    "xorl %ecx, %ecx",
    "movw %cx, %ds",
    "movw %cx, %fs",

    // save 32-bit context
    "movl %esp, {context32}(%rip)", // CONTEXT32.esp

    // restore 64-bit context
    "movq {context64}(%rip), %rsp", // CONTEXT64.rsp
    // fs_addr restored in call64()

    // Preserve callee-saved registers, as expected by win32 ABI.
    // See "Calling conventions" in doc/x86-64.md: preserve EBX, ESI, EDI, and EBP.
    // When we call into Rust, it will preserve following sysV: RBX, RBP, and some r*.
    // So we only need to save ESI and EDI.
    "pushq %rdi",
    "pushq %rsi",

    "call {call64}", // call into Rust

    "popq %rsi",
    "popq %rdi",

    // save 64-bit context
    "movq %rsp, {context64}(%rip)",  // CONTEXT64.rsp

    // restore 32-bit context
    "leaq {context32}(%rip), %rcx",
    "movl 0x0(%rcx), %esp", // CONTEXT32.esp
    "movw 0x4(%rcx), %ds",  // CONTEXT32.ds
    "movw 0x6(%rcx), %fs",  // CONTEXT32.fs

    // After call, attempt to clear registers to make execution traces easier to match.
    // eax: holds return value
    "xorl %ecx, %ecx",
    // edx: sometimes used for 64-bit returns
    // ebx, ebp: callee-saved
    // esi, edi: restored above

    "lret", // back to 32-bit
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
        unsafe {
            CONTEXT64.fs_addr = get_fs_addr_64();
        }
        let code32_selector = {
            #[cfg(target_os = "linux")]
            {
                // See doc/x86-64.md's discussion of segmentation for where these constants come from.
                let cs = {
                    const GDT_ENTRY_DEFAULT_USER32_CS: u16 = 4;
                    (GDT_ENTRY_DEFAULT_USER32_CS << 3) | 0b011
                };
                let ds = {
                    const GDT_ENTRY_DEFAULT_USER_DS: u16 = 5;
                    (GDT_ENTRY_DEFAULT_USER_DS << 3) | 0b011
                };
                unsafe { CONTEXT32.ds = ds };
                cs
            }

            #[cfg(not(target_os = "linux"))]
            {
                let code32_selector = crate::ldt::init();
                todo!("context32 init");
                code32_selector
            }
        };

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

/// Set the address fs points to, 64-bit mode only.
fn set_fs_addr_64(fs_addr: u64) {
    const ARCH_SET_FS: u64 = 0x1002;
    let ret = unsafe { libc::syscall(libc::SYS_arch_prctl, ARCH_SET_FS, fs_addr) };
    if ret != 0 {
        let err = std::io::Error::last_os_error();
        panic!("SYS_arch_prctl(ARCH_SET_FS) failed: {err}");
    }
}

/// Get the address fs points to, 64-bit mode only.
fn get_fs_addr_64() -> u64 {
    let mut fs: u64 = 0;
    const ARCH_GET_FS: u64 = 0x1003;
    let ret = unsafe { libc::syscall(libc::SYS_arch_prctl, ARCH_GET_FS, &raw mut fs) };
    if ret != 0 {
        let err = std::io::Error::last_os_error();
        panic!("SYS_arch_prctl(ARCH_GET_FS) failed: {err}");
    }
    fs
}

pub fn get_fs_addr_32() -> u32 {
    unsafe { CONTEXT32.fs_addr }
}

/// Set up a segment selector for the FS segment.
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
        let user_desc_addr = scratch_addr;
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
        user_desc.entry_number as u16
    };
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

        let mut x86_ret;
        std::arch::asm!(
            // This function is marked as clobber_abi("system") so it will preserve
            // the registers that are expected to be preserved by the system ABI.
            // But we need to manually preserve the callee-saved registers: RBX, RBP.
            // Even if the callee attempts to preserve them, it will only preserve
            // them as 32-bit registers.
            // In particular getting this wrong manifests as rbp losing its high bits after this call.
            "pushq %rbx",
            "pushq %rbp",

            // save 64-bit context
            "movq %rsp, {context64}(%rip)", // CONTEXT64.rsp

            // restore 32-bit context; note esp!=ebp
            "movl %ecx, %esp", // switch to 32-bit stack pointing at initial_esp
            "leaq {context32}(%rip), %rcx",
            "movl 0x0(%rcx), %ebp", // CONTEXT32.esp
            "movw 0x4(%rcx), %ds",  // CONTEXT32.ds
            "movw 0x6(%rcx), %fs",  // CONTEXT32.fs

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

            // 64-bit segment registers: all 0
            "xorl %ecx, %ecx",
            "movw %cx, %ds",
            "movw %cx, %fs",

            // save 32-bit context
            "movl %esp, {context32}(%rip)", // CONTEXT32.esp

            // restore 64-bit context
            "movq {context64}(%rip), %rsp", // CONTEXT64.rsp
            // fs_addr restored below after asm block

            "popq %rbp",
            "popq %rbx",
            options(att_syntax),

            // A 32-bit call may call back into 64-bit Rust code, so it may clobber
            // 64-bit registers.  Mark this code as if it's a 64-bit call.
            clobber_abi("system"),

            // We try to clear all registers so that traces line up across invocations,
            // so mark each one as clobbered and use them above explicitly.
            inout("eax") func => x86_ret,  // passed to trans32
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

        set_fs_addr_64(CONTEXT64.fs_addr as u64);

        x86_ret
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
