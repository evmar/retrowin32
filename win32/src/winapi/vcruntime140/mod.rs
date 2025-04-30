#![allow(non_snake_case)]

mod builtin;

pub use builtin::DLL;

use crate::System;
use memory::{Extensions, ExtensionsMut};

#[win32_derive::dllexport(cdecl)]
pub fn memcpy(sys: &mut dyn System, dst: u32, src: u32, len: u32) -> u32 {
    // TODO: this probably violates Rust rules around aliasing, if callers expect memmove.
    sys.mem()
        .sub32_mut(dst, len)
        .copy_from_slice(sys.mem().sub32(src, len));
    dst
}

#[win32_derive::dllexport(cdecl)]
pub fn memset(sys: &mut dyn System, dst: u32, val: u32, len: u32) -> u32 {
    sys.mem().sub32_mut(dst, len).fill(val as u8);
    dst
}

#[win32_derive::dllexport(cdecl)]
pub fn memcmp(sys: &mut dyn System, lhs: u32, rhs: u32, len: u32) -> u32 {
    let left = sys.mem().sub32(lhs, len);
    let right = sys.mem().sub32(rhs, len);
    match left.cmp(right) {
        std::cmp::Ordering::Less => -1i32 as u32,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    }
}

#[win32_derive::dllexport(cdecl)]
pub fn _CxxThrowException(sys: &dyn System, pExceptionObject: u32, pThrowInfo: u32) -> u32 {
    panic!("exception");
}
