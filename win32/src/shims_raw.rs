//! "Shims" are my word for the mechanism for x86 -> retrowin32 (and back) calls.
//!
//! This module implements Shims for non-emulated cpu case, using raw 32-bit memory.
//! See doc/x86-64.md for an overview.

use crate::{shims::Shim, Machine};

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
}

pub struct Shims {
    buf: ScratchSpace,
    /// Address that we write a pointer to the Machine to.
    machine_ptr: *mut u8,
    /// Address of the call64 trampoline.
    call64_addr: u32,
    /// Address of the tramp32 trampoline.
    pub tramp32_addr: u32,
}

impl Shims {
    pub fn new(addr: *mut u8, size: u32) -> Self {
        unsafe {
            let mut buf = ScratchSpace::new(addr, size as usize);

            // trampoline_x86-64.s:call64:
            let call64 = buf.write(b"\x57\x56");
            buf.write(b"\x48\xbf");
            let machine_ptr = buf.write(&0u64.to_le_bytes());
            buf.write(
                b"\x48\x8d\x74\x24\x20\
                \xff\x54\x24\x18\
                \x5e\x5f\
                \xca\x08\x00",
            );
            buf.realign();

            // 16:32 selector:address of call64
            let call64_addr = buf.write(&(call64 as u32).to_le_bytes()) as u32;
            buf.write(&(0x2bu32).to_le_bytes());
            buf.realign();

            // trampoline_x86.s:tramp32:
            let tramp32_addr = buf.write(
                b"\xff\xd0\
                \xcb",
            ) as u32;
            buf.realign();

            Shims {
                buf,
                machine_ptr,
                call64_addr,
                tramp32_addr,
            }
        }
    }

    /// HACK: we need a pointer to the Machine, but we get it so late we have to poke it in
    /// way after all the initialization happens...
    pub unsafe fn set_machine(&mut self, machine: *const Machine) {
        let addr = machine as u64;
        std::ptr::copy_nonoverlapping(&addr, self.machine_ptr as *mut u64, 1);
    }

    pub fn add(&mut self, shim: Shim) -> u32 {
        unsafe {
            let target: u64 = shim.func as u64;

            // Code from trampoline_x86.s:

            // pushl high 32 bits of dest
            let tramp_addr = self.buf.write(b"\x68") as u32;
            self.buf.write(&((target >> 32) as u32).to_le_bytes());
            // pushl low 32 bits of dest
            self.buf.write(b"\x68");
            self.buf.write(&(target as u32).to_le_bytes());

            // lcalll *call64_addr
            self.buf.write(b"\xff\x1d");
            self.buf.write(&self.call64_addr.to_le_bytes());

            // retl <16-bit bytes to pop>
            self.buf.write(b"\xc2");
            // TODO revisit stack_consumed, does it include eip or not?
            // We have to -4 here to not include IP.
            let stack_consumed: u16 = shim.stack_consumed.unwrap() as u16 - 4;
            self.buf.write(&stack_consumed.to_le_bytes());
            self.buf.realign();

            tramp_addr
        }
    }

    pub fn add_todo(&mut self, _name: String) -> u32 {
        // trampoline_x86.rs:crash
        unsafe { self.buf.write(b"\xcc\xb8\x01\x00\x00\x00\xff\x20") as u32 }
    }
}

pub fn become_async(
    _machine: &mut Machine,
    _future: std::pin::Pin<Box<dyn std::future::Future<Output = ()>>>,
) {
    todo!()
}

pub struct UnimplFuture {}
impl std::future::Future for UnimplFuture {
    type Output = ();

    fn poll(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        todo!()
    }
}

pub fn async_call(_machine: &mut Machine, _func: u32, _args: Vec<u32>) -> UnimplFuture {
    todo!()
}
