//! Functions to unsafely grab winapi function arguments from an x86 stack.

use super::types::Str16;
use memory::Mem;

/// ArrayWithSize<&[u8]> matches a pair of C arguments like
///    const u8_t* items, size_t len,
pub type ArrayWithSize<'a, T> = Option<&'a [T]>;
pub type ArrayWithSizeMut<'a, T> = Option<&'a mut [T]>;

pub trait FromX86<'a> {
    unsafe fn from_stack(mem: Mem<'a>, sp: u32) -> Self;
}

impl<'a> FromX86<'a> for u32 {
    unsafe fn from_stack(mem: Mem<'a>, sp: u32) -> Self {
        mem.get::<u32>(sp)
    }
}

impl<'a> FromX86<'a> for i32 {
    unsafe fn from_stack(mem: Mem<'a>, sp: u32) -> Self {
        mem.get::<u32>(sp) as i32
    }
}

impl<'a> FromX86<'a> for bool {
    unsafe fn from_stack(mem: Mem<'a>, sp: u32) -> Self {
        mem.get::<u32>(sp) != 0
    }
}

impl<'a, T: TryFrom<u32>> FromX86<'a> for Result<T, T::Error> {
    unsafe fn from_stack(mem: Mem<'a>, sp: u32) -> Self {
        T::try_from(mem.get::<u32>(sp))
    }
}

fn check_aligned<T: memory::Pod>(ptr: u32) {
    let align = std::mem::align_of::<T>();
    if ptr as usize % align != 0 {
        log::error!("pointer {ptr:x} should be aligned to {align}");
    }
}

impl<'a, T: memory::Pod> FromX86<'a> for Option<&'a T> {
    unsafe fn from_stack(mem: Mem<'a>, sp: u32) -> Self {
        let addr = mem.get::<u32>(sp);
        if addr == 0 {
            return None;
        }
        check_aligned::<T>(addr);
        Some(mem.view::<T>(addr))
    }
}

impl<'a, T: memory::Pod> FromX86<'a> for Option<&'a mut T> {
    unsafe fn from_stack(mem: Mem<'a>, sp: u32) -> Self {
        let addr = mem.get::<u32>(sp);
        if addr == 0 {
            return None;
        }
        check_aligned::<T>(addr);
        Some(mem.view_mut::<T>(addr))
    }
}

impl<'a> FromX86<'a> for Option<&'a [u8]> {
    unsafe fn from_stack(mem: Mem<'a>, sp: u32) -> Self {
        let addr = mem.get::<u32>(sp);
        let len = mem.get::<u32>(sp + 4);
        if addr == 0 {
            return None;
        }
        Some(&mem.sub(addr, len).as_slice_todo())
    }
}

impl<'a> FromX86<'a> for Option<&'a mut [u8]> {
    unsafe fn from_stack(mem: Mem<'a>, sp: u32) -> Self {
        let addr = mem.get::<u32>(sp);
        let len = mem.get::<u32>(sp + 4);
        if addr == 0 {
            return None;
        }
        Some(mem.sub(addr, len).as_mut_slice_todo())
    }
}

impl<'a> FromX86<'a> for Option<&'a [u16]> {
    unsafe fn from_stack(mem: Mem<'a>, sp: u32) -> Self {
        let addr = mem.get::<u32>(sp);
        let len = mem.get::<u32>(sp + 4);
        if addr == 0 {
            return None;
        }
        Some(std::mem::transmute(mem.sub(addr, len).as_slice_todo()))
    }
}

impl<'a> FromX86<'a> for Option<&'a mut [u16]> {
    unsafe fn from_stack(mem: Mem, sp: u32) -> Self {
        let addr = mem.get::<u32>(sp);
        let len = mem.get::<u32>(sp + 4);
        if addr == 0 {
            return None;
        }
        Some(std::mem::transmute(mem.sub(addr, len).as_mut_slice_todo()))
    }
}

impl<'a> FromX86<'a> for Option<&'a str> {
    unsafe fn from_stack(mem: Mem<'a>, sp: u32) -> Self {
        let addr = mem.get::<u32>(sp);
        if addr == 0 {
            return None;
        }
        let strz = mem.slicez(addr).unwrap().to_ascii();
        Some(strz)
    }
}

impl<'a> FromX86<'a> for Option<Str16<'a>> {
    unsafe fn from_stack(mem: Mem<'a>, sp: u32) -> Self {
        let addr = mem.get::<u32>(sp);
        if addr == 0 {
            return None;
        }
        let mem16: &[u16] = {
            let mem = mem.slice(addr..);
            let ptr = mem.as_slice_todo().as_ptr() as *const u16;
            std::slice::from_raw_parts(ptr, mem.len() as usize / 2)
        };
        Some(Str16::from_nul_term(mem16))
    }
}

/// Types that can be returned from a winapi function, passed via EAX.
pub trait ToX86 {
    fn to_raw(&self) -> u32;
}
impl ToX86 for bool {
    fn to_raw(&self) -> u32 {
        if *self {
            1
        } else {
            0
        }
    }
}
impl ToX86 for u32 {
    fn to_raw(&self) -> u32 {
        *self
    }
}
