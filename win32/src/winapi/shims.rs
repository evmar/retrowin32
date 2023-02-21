//! Functions to unsafely grab winapi function arguments from an X86.

use super::types::Str16;
use x86::{Memory, X86};

unsafe fn smuggle<T: ?Sized>(x: &T) -> &'static T {
    std::mem::transmute(x)
}
unsafe fn smuggle_mut<T: ?Sized>(x: &mut T) -> &'static mut T {
    std::mem::transmute(x)
}

pub trait FromX86: Sized {
    fn from_raw(_raw: u32) -> Self {
        unimplemented!()
    }
    unsafe fn from_x86(x86: &mut X86) -> Self {
        Self::from_raw(x86::ops::pop(x86))
    }
}
impl FromX86 for u32 {
    fn from_raw(raw: u32) -> Self {
        raw
    }
}
impl FromX86 for i32 {
    fn from_raw(raw: u32) -> Self {
        raw as i32
    }
}
impl FromX86 for bool {
    fn from_raw(raw: u32) -> Self {
        raw != 0
    }
}
impl<T: TryFrom<u32>> FromX86 for Result<T, T::Error> {
    fn from_raw(raw: u32) -> Self {
        T::try_from(raw)
    }
}
impl<T: x86::Pod> FromX86 for Option<&T> {
    unsafe fn from_x86(x86: &mut X86) -> Self {
        let addr = x86::ops::pop(x86);
        if addr == 0 {
            None
        } else {
            Some(smuggle(x86.mem.view::<T>(addr)))
        }
    }
}
impl<T: x86::Pod> FromX86 for Option<&mut T> {
    unsafe fn from_x86(x86: &mut X86) -> Self {
        let addr = x86::ops::pop(x86);
        if addr == 0 {
            None
        } else {
            Some(smuggle_mut(x86.mem.view_mut::<T>(addr)))
        }
    }
}
impl FromX86 for Option<&[u8]> {
    unsafe fn from_x86(x86: &mut X86) -> Self {
        let ofs = x86::ops::pop(x86) as usize;
        let len = x86::ops::pop(x86) as usize;
        if ofs == 0 {
            return None;
        }
        Some(smuggle(&x86.mem[ofs..ofs + len]))
    }
}
impl FromX86 for Option<&mut [u8]> {
    unsafe fn from_x86(x86: &mut X86) -> Self {
        let ofs = x86::ops::pop(x86) as usize;
        let len = x86::ops::pop(x86) as usize;
        if ofs == 0 {
            return None;
        }
        Some(smuggle_mut(&mut x86.mem[ofs..ofs + len]))
    }
}
impl FromX86 for Option<&[u16]> {
    unsafe fn from_x86(x86: &mut X86) -> Self {
        let ofs = x86::ops::pop(x86) as usize;
        let len = x86::ops::pop(x86) as usize;
        if ofs == 0 {
            return None;
        }
        std::mem::transmute(&x86.mem[ofs..ofs + len])
    }
}
impl FromX86 for Option<&mut [u16]> {
    unsafe fn from_x86(x86: &mut X86) -> Self {
        let ofs = x86::ops::pop(x86) as usize;
        let len = x86::ops::pop(x86) as usize;
        if ofs == 0 {
            return None;
        }
        std::mem::transmute(&mut x86.mem[ofs..ofs + len])
    }
}
impl FromX86 for Option<&str> {
    unsafe fn from_x86(x86: &mut X86) -> Self {
        let ofs = x86::ops::pop(x86) as usize;
        if ofs == 0 {
            return None;
        }
        let strz = x86.mem[ofs..].read_strz();
        Some(smuggle(strz))
    }
}
impl<'a> FromX86 for Option<Str16<'a>> {
    unsafe fn from_x86(x86: &mut X86) -> Self {
        let ofs = x86::ops::pop(x86) as usize;
        if ofs == 0 {
            return None;
        }
        let mem16: &[u16] = {
            let mem = &x86.mem[ofs as usize..];
            let ptr = mem.as_ptr() as *const u16;
            std::slice::from_raw_parts(ptr, mem.len() / 2)
        };
        Some(Str16::from_nul_term(mem16))
    }
}
pub unsafe fn from_x86<T: FromX86>(x86: &mut X86) -> T {
    T::from_x86(x86)
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
impl ToX86 for usize {
    fn to_raw(&self) -> u32 {
        *self as u32
    }
}
