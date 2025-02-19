//! Functions implementing the win32 calling convention:
//! - reading arguments from the stack;
//! - converting return values into a form suitable for putting in registers
//!
//! These functions work with an arbitrary stack pointer (for input) and convert to
//! plain values (for output) so that different machine implementations can do their
//! own control over the stack and moving return values into registers.

use crate::winapi::{CStr, Str16};
use memory::{str16, Extensions, ExtensionsMut, Mem};

/// ArrayWithSize<u8> matches a pair of C arguments like
///    const u8_t* items, size_t len,
pub type ArrayWithSize<'a, T> = Option<&'a [T]>;

/// ArrayWithSizeMut<u8> matches a pair of C arguments like
///    u8_t* items, size_t len,
/// it's a wrapper type to provide a custom Debug implementation that doesn't
/// dump the internal contents.
pub struct ArrayWithSizeMut<'a, T>(Option<&'a mut [T]>);

impl<'a, T> std::fmt::Debug for ArrayWithSizeMut<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            None => write!(f, "None"),
            Some(buf) => write!(f, "[buffer size {}]", buf.len()),
        }
    }
}

impl<'a, T> ArrayWithSizeMut<'a, T> {
    pub fn to_option(self) -> Option<&'a mut [T]> {
        self.0
    }
    pub fn unwrap(self) -> &'a mut [T] {
        self.0.unwrap()
    }
}

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
        T::from_arg(mem, mem.get_pod::<u32>(sp))
    }
}

impl<'a> FromArg<'a> for u8 {
    unsafe fn from_arg(_mem: Mem<'a>, arg: u32) -> Self {
        arg as u8
    }
}

impl<'a> FromArg<'a> for i8 {
    unsafe fn from_arg(_mem: Mem<'a>, arg: u32) -> Self {
        arg as i8
    }
}

impl<'a> FromArg<'a> for u16 {
    unsafe fn from_arg(_mem: Mem<'a>, arg: u32) -> Self {
        arg as u16
    }
}

impl<'a> FromArg<'a> for i16 {
    unsafe fn from_arg(_mem: Mem<'a>, arg: u32) -> Self {
        arg as i16
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

impl<'a, T: memory::Pod> FromArg<'a> for Option<&'a T> {
    unsafe fn from_arg(mem: Mem<'a>, arg: u32) -> Self {
        if arg == 0 {
            return None;
        }
        // TODO: we cannot guarantee stack args are aligned.
        Some(mem.get_aligned_ref::<T>(arg))
    }
}

impl<'a, T: memory::Pod> FromArg<'a> for Option<&'a mut T> {
    unsafe fn from_arg(mem: Mem<'a>, arg: u32) -> Self {
        if arg == 0 {
            return None;
        }
        // TODO: we cannot guarantee stack args are aligned.
        Some(mem.get_aligned_ref_mut::<T>(arg))
    }
}

impl<'a, T: memory::Pod> FromStack<'a> for Option<&'a [T]> {
    unsafe fn from_stack(mem: Mem<'a>, sp: u32) -> Self {
        let addr = mem.get_pod::<u32>(sp);
        let count = mem.get_pod::<u32>(sp + 4);
        if addr == 0 {
            return None;
        }
        let slice = mem.sub32(addr, count);
        Some(std::slice::from_raw_parts(
            slice.as_ptr() as *const _,
            count as usize,
        ))
    }
}

impl<'a, T: memory::Pod> FromStack<'a> for ArrayWithSizeMut<'a, T> {
    unsafe fn from_stack(mem: Mem<'a>, sp: u32) -> Self {
        let addr = mem.get_pod::<u32>(sp);
        let count = mem.get_pod::<u32>(sp + 4);
        if addr == 0 {
            return ArrayWithSizeMut(None);
        }
        let slice = mem.sub32_mut(addr, count);
        ArrayWithSizeMut(Some(std::slice::from_raw_parts_mut(
            slice.as_mut_ptr() as *mut _,
            count as usize,
        )))
    }
}

impl<'a> FromArg<'a> for Option<&'a str> {
    unsafe fn from_arg(mem: Mem<'a>, arg: u32) -> Self {
        if arg == 0 {
            return None;
        }
        let strz = str16::expect_ascii(mem.slicez(arg));
        Some(strz)
    }
}

impl<'a> FromArg<'a> for Option<&'a CStr> {
    unsafe fn from_arg(mem: Mem<'a>, arg: u32) -> Self {
        if arg == 0 {
            return None;
        }
        Some(CStr::from_ptr(mem.get_ptr::<u8>(arg) as *const _))
    }
}

impl<'a> FromArg<'a> for Option<&'a Str16> {
    unsafe fn from_arg(mem: Mem<'a>, arg: u32) -> Self {
        Str16::from_nul_term_ptr(mem, arg)
    }
}

/// VarArgs marks a function as cdecl and grabs the stack pointer for the callee.
#[derive(Debug)]
pub struct VarArgs(u32);
impl VarArgs {
    pub fn pop<'a, T: FromArg<'a>>(&mut self, mem: Mem<'a>) -> T {
        let value = unsafe { T::from_stack(mem, self.0) };
        self.0 += 4; // TODO: should expose stack_consumed for use here and switch to FromStack
        value
    }
}
impl<'a> FromStack<'a> for VarArgs {
    unsafe fn from_stack(_mem: Mem<'a>, sp: u32) -> Self {
        VarArgs(sp)
    }
}

/// Types that can be returned from a winapi function, passed via EAX and sometimes EDX.
pub trait ABIReturn {
    fn into_abireturn(self) -> u64;
}

impl ABIReturn for bool {
    fn into_abireturn(self) -> u64 {
        if self {
            1
        } else {
            0
        }
    }
}

impl ABIReturn for u64 {
    fn into_abireturn(self) -> u64 {
        self
    }
}

impl ABIReturn for u32 {
    fn into_abireturn(self) -> u64 {
        self as u64
    }
}

impl ABIReturn for i32 {
    fn into_abireturn(self) -> u64 {
        self as u32 as u64
    }
}

impl ABIReturn for () {
    fn into_abireturn(self) -> u64 {
        0
    }
}
