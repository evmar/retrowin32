//! Functions to unsafely grab winapi function arguments from an x86 stack.

use super::types::Str16;
use memory::Mem;

/// ArrayWithSize<&[u8]> matches a pair of C arguments like
///    const u8_t* items, size_t len,
pub type ArrayWithSize<'a, T> = Option<&'a [T]>;
pub type ArrayWithSizeMut<'a, T> = Option<&'a mut [T]>;

/// Lowest level trait: given a stack pointer, extract the argument.
/// Implemented by argument types that read multiple things off the stack.
pub trait FromStack<'a> {
    unsafe fn from_stack(mem: Mem<'a>, sp: u32) -> Self;
}

/// Higher level trait: given a value read from the stack, convert.
pub trait FromArg<'a> {
    unsafe fn from_arg(mem: Mem<'a>, arg: u32) -> Self;
}

impl<'a, T: FromArg<'a>> FromStack<'a> for T {
    unsafe fn from_stack(mem: Mem<'a>, sp: u32) -> Self {
        T::from_arg(mem, mem.get::<u32>(sp))
    }
}

impl<'a> FromArg<'a> for u32 {
    unsafe fn from_arg(_mem: Mem<'a>, arg: u32) -> Self {
        arg
    }
}

impl<'a> FromArg<'a> for i32 {
    unsafe fn from_arg(_mem: Mem<'a>, arg: u32) -> Self {
        arg as i32
    }
}

impl<'a> FromArg<'a> for bool {
    unsafe fn from_arg(_mem: Mem<'a>, arg: u32) -> Self {
        arg != 0
    }
}

impl<'a, T: TryFrom<u32>> FromArg<'a> for Result<T, T::Error> {
    unsafe fn from_arg(_mem: Mem<'a>, arg: u32) -> Self {
        T::try_from(arg)
    }
}

fn check_aligned<T: memory::Pod>(ptr: u32) {
    let align = std::mem::align_of::<T>();
    if ptr as usize % align != 0 {
        log::error!("pointer {ptr:x} should be aligned to {align}");
    }
}

impl<'a, T: memory::Pod> FromArg<'a> for Option<&'a T> {
    unsafe fn from_arg(mem: Mem<'a>, arg: u32) -> Self {
        if arg == 0 {
            return None;
        }
        check_aligned::<T>(arg);
        Some(mem.view::<T>(arg))
    }
}

impl<'a, T: memory::Pod> FromArg<'a> for Option<&'a mut T> {
    unsafe fn from_arg(mem: Mem<'a>, arg: u32) -> Self {
        if arg == 0 {
            return None;
        }
        check_aligned::<T>(arg);
        Some(mem.view_mut::<T>(arg))
    }
}

impl<'a> FromStack<'a> for Option<&'a [u8]> {
    unsafe fn from_stack(mem: Mem<'a>, sp: u32) -> Self {
        let addr = mem.get::<u32>(sp);
        let len = mem.get::<u32>(sp + 4);
        if addr == 0 {
            return None;
        }
        Some(&mem.sub(addr, len).as_slice_todo())
    }
}

impl<'a> FromStack<'a> for Option<&'a mut [u8]> {
    unsafe fn from_stack(mem: Mem<'a>, sp: u32) -> Self {
        let addr = mem.get::<u32>(sp);
        let len = mem.get::<u32>(sp + 4);
        if addr == 0 {
            return None;
        }
        Some(mem.sub(addr, len).as_mut_slice_todo())
    }
}

impl<'a> FromStack<'a> for Option<&'a [u16]> {
    unsafe fn from_stack(mem: Mem<'a>, sp: u32) -> Self {
        let addr = mem.get::<u32>(sp);
        let len = mem.get::<u32>(sp + 4);
        if addr == 0 {
            return None;
        }
        Some(std::mem::transmute(mem.sub(addr, len).as_slice_todo()))
    }
}

impl<'a> FromStack<'a> for Option<&'a mut [u16]> {
    unsafe fn from_stack(mem: Mem, sp: u32) -> Self {
        let addr = mem.get::<u32>(sp);
        let len = mem.get::<u32>(sp + 4);
        if addr == 0 {
            return None;
        }
        Some(std::mem::transmute(mem.sub(addr, len).as_mut_slice_todo()))
    }
}

impl<'a> FromArg<'a> for Option<&'a str> {
    unsafe fn from_arg(mem: Mem<'a>, arg: u32) -> Self {
        if arg == 0 {
            return None;
        }
        let strz = mem.slicez(arg).unwrap().to_ascii();
        Some(strz)
    }
}

impl<'a> FromArg<'a> for Option<&'a Str16> {
    unsafe fn from_arg(mem: Mem<'a>, arg: u32) -> Self {
        Str16::from_ptr(mem, arg)
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
impl ToX86 for i32 {
    fn to_raw(&self) -> u32 {
        *self as u32
    }
}
