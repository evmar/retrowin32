//! Functions to unsafely grab winapi function arguments from an x86 stack.

use super::types::Str16;
use x86::{Mem, Memory};

unsafe fn extend_lifetime<'a, T: ?Sized>(x: &T) -> &'a T {
    std::mem::transmute(x)
}
unsafe fn extend_lifetime_mut<'a, T: ?Sized>(x: &mut T) -> &'a mut T {
    std::mem::transmute(x)
}

pub trait FromX86: Sized {
    fn from_raw(_raw: u32) -> Self {
        unimplemented!()
    }
    unsafe fn from_stack(mem: &mut Mem, sp: u32) -> Self {
        Self::from_raw(*mem.view::<u32>(sp))
    }
    fn stack_consumed() -> u32 {
        4
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
    unsafe fn from_stack(mem: &mut Mem, sp: u32) -> Self {
        let addr = mem.read_u32(sp);
        if addr == 0 {
            None
        } else {
            Some(extend_lifetime(mem.view::<T>(addr)))
        }
    }
}

impl<T: x86::Pod> FromX86 for Option<&mut T> {
    unsafe fn from_stack(mem: &mut Mem, sp: u32) -> Self {
        let addr = mem.read_u32(sp);
        if addr == 0 {
            None
        } else {
            Some(extend_lifetime_mut(mem.view_mut::<T>(addr)))
        }
    }
}

impl FromX86 for Option<&[u8]> {
    unsafe fn from_stack(mem: &mut Mem, sp: u32) -> Self {
        let addr = mem.read_u32(sp) as usize;
        let len = mem.read_u32(sp + 4) as usize;
        if addr == 0 {
            return None;
        }
        Some(extend_lifetime(&mem[addr..addr + len].as_slice_todo()))
    }
    fn stack_consumed() -> u32 {
        8
    }
}

impl FromX86 for Option<&mut [u8]> {
    unsafe fn from_stack(mem: &mut Mem, sp: u32) -> Self {
        let addr = mem.read_u32(sp) as usize;
        let len = mem.read_u32(sp + 4) as usize;
        if addr == 0 {
            return None;
        }
        Some(extend_lifetime_mut(
            mem[addr..addr + len].as_mut_slice_todo(),
        ))
    }
    fn stack_consumed() -> u32 {
        8
    }
}

impl FromX86 for Option<&[u16]> {
    unsafe fn from_stack(mem: &mut Mem, sp: u32) -> Self {
        let addr = mem.read_u32(sp) as usize;
        let len = mem.read_u32(sp + 4) as usize;
        if addr == 0 {
            return None;
        }
        std::mem::transmute(&mem[addr..addr + len])
    }
    fn stack_consumed() -> u32 {
        8
    }
}

impl FromX86 for Option<&mut [u16]> {
    unsafe fn from_stack(mem: &mut Mem, sp: u32) -> Self {
        let addr = mem.read_u32(sp) as usize;
        let len = mem.read_u32(sp + 4) as usize;
        if addr == 0 {
            return None;
        }
        std::mem::transmute(&mut mem[addr..addr + len])
    }
    fn stack_consumed() -> u32 {
        8
    }
}

impl FromX86 for Option<&str> {
    unsafe fn from_stack(mem: &mut Mem, sp: u32) -> Self {
        let addr = mem.read_u32(sp) as usize;
        if addr == 0 {
            return None;
        }
        let strz = mem[addr as usize..].read_strz();
        Some(extend_lifetime(strz))
    }
}

impl<'a> FromX86 for Option<Str16<'a>> {
    unsafe fn from_stack(mem: &mut Mem, sp: u32) -> Self {
        let addr = mem.read_u32(sp) as usize;
        if addr == 0 {
            return None;
        }
        let mem16: &[u16] = {
            let mem = &mem[addr as usize..];
            let ptr = mem.as_slice_todo().as_ptr() as *const u16;
            std::slice::from_raw_parts(ptr, mem.len() / 2)
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
impl ToX86 for usize {
    fn to_raw(&self) -> u32 {
        *self as u32
    }
}
