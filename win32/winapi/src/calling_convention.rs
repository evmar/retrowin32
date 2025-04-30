//! Functions implementing the win32 calling convention:
//! - reading arguments from the stack;
//! - converting return values into a form suitable for putting in registers
//!
//! These functions work with an arbitrary stack pointer (for input) and convert to
//! plain values (for output) so that different machine implementations can do their
//! own control over the stack and moving return values into registers.

use crate::{CStr, Str16};
use memory::{Extensions, ExtensionsMut, Mem};
use std::ops::{Deref, DerefMut};

/// Array<u8> matches a pair of C arguments like
///    const u8_t* items, size_t len,
#[derive(Debug)]
pub struct Array<'a, T> {
    /// Array's underlying buffer is always [u8] for alignment reasons.
    bytes: &'a [u8],
    _marker: std::marker::PhantomData<T>,
}

impl<T> Deref for Array<'_, T> {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        self.bytes
    }
}

impl<'a, T: memory::Pod> FromStack<'a> for Array<'a, T> {
    fn from_stack(mem: Mem<'a>, sp: u32) -> Self {
        let buf = ArrayOut::<'a, T>::from_stack(mem, sp);
        Array {
            bytes: buf.bytes,
            _marker: std::marker::PhantomData,
        }
    }
}

/// ArrayOut<u8> is like Array<u8> but it doesn't print its contents in its Debug impl.
pub struct ArrayOut<'a, T> {
    bytes: &'a mut [u8],
    _marker: std::marker::PhantomData<T>,
}

impl<'a, T: memory::Pod> ArrayOut<'a, T> {
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        self.bytes
    }

    pub fn put_pod(&mut self, index: u32, val: T) {
        let ofs = index * std::mem::size_of::<T>() as u32;
        assert!(ofs as usize + std::mem::size_of::<T>() <= self.bytes.len());
        self.bytes.put_pod::<T>(ofs, val);
    }
}

impl<'a, T: memory::Pod> FromStack<'a> for ArrayOut<'a, T> {
    fn from_stack(mem: Mem<'a>, sp: u32) -> Self {
        let addr = mem.get_pod::<u32>(sp);
        assert!(addr != 0);
        let count = mem.get_pod::<u32>(sp + 4);
        let slice = mem.sub32_mut(addr, count);
        ArrayOut {
            bytes: slice,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<'a, T> std::fmt::Debug for ArrayOut<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if std::mem::size_of::<T>() == 1 {
            write!(f, "[buffer size {}]", self.bytes.len())
        } else {
            write!(
                f,
                "[buffer size {}x{}]",
                self.bytes.len(),
                std::mem::size_of::<T>()
            )
        }
    }
}

impl<T> Deref for ArrayOut<'_, T> {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        self.bytes
    }
}

impl<T> DerefMut for ArrayOut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.bytes
    }
}

impl<'a, T: memory::Pod> FromStack<'a> for Option<ArrayOut<'a, T>> {
    fn from_stack(mem: Mem<'a>, sp: u32) -> Self {
        let addr = mem.get_pod::<u32>(sp);
        if addr == 0 {
            return None;
        }
        return Some(ArrayOut::from_stack(mem, sp));
    }
}

/// Lowest level trait: given a stack pointer, extract the argument.
/// Implemented by argument types that read multiple things off the stack.
pub trait FromStack<'a> {
    fn from_stack(mem: Mem<'a>, sp: u32) -> Self;
}

/// Higher level trait: given a value read from the stack, convert.
pub trait FromArg<'a> {
    fn from_arg(mem: Mem<'a>, arg: u32) -> Self;
}

impl<'a, T: FromArg<'a>> FromStack<'a> for T {
    fn from_stack(mem: Mem<'a>, sp: u32) -> Self {
        T::from_arg(mem, mem.get_pod::<u32>(sp))
    }
}

impl<'a> FromArg<'a> for u8 {
    fn from_arg(_mem: Mem<'a>, arg: u32) -> Self {
        arg as u8
    }
}

impl<'a> FromArg<'a> for i8 {
    fn from_arg(_mem: Mem<'a>, arg: u32) -> Self {
        arg as i8
    }
}

impl<'a> FromArg<'a> for u16 {
    fn from_arg(_mem: Mem<'a>, arg: u32) -> Self {
        arg as u16
    }
}

impl<'a> FromArg<'a> for i16 {
    fn from_arg(_mem: Mem<'a>, arg: u32) -> Self {
        arg as i16
    }
}

impl<'a> FromArg<'a> for u32 {
    fn from_arg(_mem: Mem<'a>, arg: u32) -> Self {
        arg
    }
}

impl<'a> FromArg<'a> for i32 {
    fn from_arg(_mem: Mem<'a>, arg: u32) -> Self {
        arg as i32
    }
}

impl<'a> FromArg<'a> for bool {
    fn from_arg(_mem: Mem<'a>, arg: u32) -> Self {
        arg != 0
    }
}

impl<'a> FromStack<'a> for f64 {
    fn from_stack(mem: Mem<'a>, sp: u32) -> Self {
        mem.get_pod::<f64>(sp)
    }
}

impl<'a, T: TryFrom<u32>> FromArg<'a> for Result<T, T::Error> {
    fn from_arg(_mem: Mem<'a>, arg: u32) -> Self {
        T::try_from(arg)
    }
}

impl<'a, T: memory::Pod> FromArg<'a> for Option<&'a T> {
    fn from_arg(mem: Mem<'a>, arg: u32) -> Self {
        if arg == 0 {
            return None;
        }
        // TODO: we cannot guarantee stack args are aligned.
        Some(mem.get_aligned_ref::<T>(arg))
    }
}

impl<'a, T: memory::Pod> FromArg<'a> for Option<&'a mut T> {
    fn from_arg(mem: Mem<'a>, arg: u32) -> Self {
        if arg == 0 {
            return None;
        }
        // TODO: we cannot guarantee stack args are aligned.
        Some(mem.get_aligned_ref_mut::<T>(arg))
    }
}

impl<'a, T: memory::Pod> FromStack<'a> for Option<&'a [T]> {
    fn from_stack(mem: Mem<'a>, sp: u32) -> Self {
        let addr = mem.get_pod::<u32>(sp);
        let count = mem.get_pod::<u32>(sp + 4);
        if addr == 0 {
            return None;
        }
        let slice = mem.sub32(addr, count);
        let ptr = slice.as_ptr() as *const T;
        memory::check_aligned(ptr);
        // Safety: Pod allows coercion from bytes.
        Some(unsafe { std::slice::from_raw_parts(slice.as_ptr() as *const _, count as usize) })
    }
}

impl<'a> FromArg<'a> for Option<&'a str> {
    fn from_arg(mem: Mem<'a>, arg: u32) -> Self {
        if arg == 0 {
            return None;
        }
        let Some(strz) = mem.slicez(arg).try_ascii() else {
            todo!("non-ascii string; change winapi function to accept bytes");
        };
        Some(strz)
    }
}

impl<'a> FromArg<'a> for Option<&'a CStr> {
    fn from_arg(mem: Mem<'a>, arg: u32) -> Self {
        if arg == 0 {
            return None;
        }
        let buf = mem.slice(arg..);
        Some(CStr::from_bytes_until_nul(buf).unwrap())
    }
}

impl<'a> FromArg<'a> for Option<&'a Str16> {
    fn from_arg(mem: Mem<'a>, arg: u32) -> Self {
        Str16::from_nul_term_ptr(mem, arg)
    }
}

/// VarArgs marks a function as cdecl and grabs the stack pointer for the callee.
#[derive(Debug)]
pub struct VarArgs(u32);
impl VarArgs {
    pub fn pop<'a, T: FromArg<'a>>(&mut self, mem: Mem<'a>) -> T {
        let value = T::from_stack(mem, self.0);
        self.0 += 4; // TODO: should expose stack_consumed for use here and switch to FromStack
        value
    }
}
impl<'a> FromStack<'a> for VarArgs {
    fn from_stack(_mem: Mem<'a>, sp: u32) -> Self {
        VarArgs(sp)
    }
}

/// Types that can be returned from a winapi function.
pub enum ABIReturn {
    U32(u32), // via eax
    U64(u64), // via edx:eax
    F64(f64), // via st(0)
}

impl From<u32> for ABIReturn {
    fn from(value: u32) -> Self {
        ABIReturn::U32(value)
    }
}

impl From<u64> for ABIReturn {
    fn from(value: u64) -> Self {
        ABIReturn::U64(value)
    }
}

impl From<f64> for ABIReturn {
    fn from(value: f64) -> Self {
        ABIReturn::F64(value)
    }
}

impl From<bool> for ABIReturn {
    fn from(value: bool) -> Self {
        if value { 1u32.into() } else { 0u32.into() }
    }
}

impl From<i32> for ABIReturn {
    fn from(value: i32) -> Self {
        (value as u32).into()
    }
}

impl From<()> for ABIReturn {
    fn from(_: ()) -> Self {
        0u32.into()
    }
}
