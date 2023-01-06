//! Functions to unsafely grab winapi function arguments from an X86.

use crate::{memory::Memory, x86::X86};

unsafe fn smuggle<T: ?Sized>(x: &T) -> &'static T {
    std::mem::transmute(x)
}
unsafe fn smuggle_mut<T: ?Sized>(x: &mut T) -> &'static mut T {
    std::mem::transmute(x)
}

pub trait FromX86: Sized {
    unsafe fn from_raw(_raw: u32) -> Self {
        unimplemented!()
    }
    unsafe fn from_x86(x86: &mut X86) -> Self {
        Self::from_raw(x86.pop())
    }
}
impl FromX86 for u32 {
    unsafe fn from_raw(raw: u32) -> Self {
        raw
    }
}
impl FromX86 for bool {
    unsafe fn from_raw(raw: u32) -> Self {
        raw != 0
    }
}
impl<T: crate::memory::Pod> FromX86 for Option<&T> {
    unsafe fn from_x86(x86: &mut X86) -> Self {
        let addr = x86.pop();
        if addr == 0 {
            None
        } else {
            Some(smuggle(x86.mem.view::<T>(addr)))
        }
    }
}
impl<T: crate::memory::Pod> FromX86 for Option<&mut T> {
    unsafe fn from_x86(x86: &mut X86) -> Self {
        let addr = x86.pop();
        if addr == 0 {
            None
        } else {
            Some(smuggle_mut(x86.mem.view_mut::<T>(addr)))
        }
    }
}
impl FromX86 for &mut [u8] {
    unsafe fn from_x86(x86: &mut X86) -> Self {
        let ofs = x86.pop() as usize;
        let len = x86.pop() as usize;
        smuggle_mut(&mut x86.mem[ofs..ofs + len])
    }
}
impl FromX86 for &str {
    unsafe fn from_x86(x86: &mut X86) -> Self {
        let ofs = x86.pop() as usize;
        let strz = x86.mem[ofs..].read_strz();
        smuggle(strz)
    }
}

pub unsafe fn from_x86<T: FromX86>(x86: &mut X86) -> T {
    T::from_x86(x86)
}
