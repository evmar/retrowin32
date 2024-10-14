#![allow(non_snake_case)]

mod builtin;

pub use builtin::DLL;

use crate::machine::Machine;
use memory::{Extensions, ExtensionsMut};

#[win32_derive::dllexport(cdecl)]
pub fn memcpy(machine: &mut Machine, dst: u32, src: u32, len: u32) -> u32 {
    // TODO: this probably violates Rust rules around aliasing, if callers expect memmove.
    machine
        .mem()
        .sub32_mut(dst, len)
        .copy_from_slice(machine.mem().sub32(src, len));
    dst
}

#[win32_derive::dllexport(cdecl)]
pub fn memset(machine: &mut Machine, dst: u32, val: u32, len: u32) -> u32 {
    machine.mem().sub32_mut(dst, len).fill(val as u8);
    dst
}

#[win32_derive::dllexport(cdecl)]
pub fn memcmp(machine: &mut Machine, lhs: u32, rhs: u32, len: u32) -> u32 {
    let left = machine.mem().sub32(lhs, len);
    let right = machine.mem().sub32(rhs, len);
    match left.cmp(right) {
        std::cmp::Ordering::Less => -1i32 as u32,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    }
}

#[win32_derive::dllexport(cdecl)]
pub fn _CxxThrowException(_machine: &mut Machine, pExceptionObject: u32, pThrowInfo: u32) -> u32 {
    panic!("exception");
}
