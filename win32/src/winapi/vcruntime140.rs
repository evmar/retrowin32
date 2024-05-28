#![allow(non_snake_case)]

use crate::machine::Machine;
use memory::Extensions;

const TRACE_CONTEXT: &'static str = "vcruntime140";

#[win32_derive::dllexport(cdecl)]
pub fn memcpy(machine: &mut Machine, dst: u32, src: u32, len: u32) -> u32 {
    // TODO: this probably violates Rust rules around aliasing, if callers expect memmove.
    machine
        .mem()
        .sub(dst, len)
        .as_mut_slice_todo()
        .copy_from_slice(machine.mem().sub(src, len).as_slice_todo());
    dst
}

#[win32_derive::dllexport(cdecl)]
pub fn memset(machine: &mut Machine, dst: u32, val: u32, len: u32) -> u32 {
    machine
        .mem()
        .sub(dst, len)
        .as_mut_slice_todo()
        .fill(val as u8);
    dst
}

#[win32_derive::dllexport(cdecl)]
pub fn memcmp(machine: &mut Machine, lhs: u32, rhs: u32, len: u32) -> u32 {
    let left = machine.mem().sub(lhs, len).as_slice();
    let right = machine.mem().sub(rhs, len).as_slice();
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
