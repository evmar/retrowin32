#![allow(non_snake_case)]

use crate::machine::Machine;

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
    0
}

#[win32_derive::dllexport(cdecl)]
pub fn _CxxThrowException(_machine: &mut Machine, pExceptionObject: u32, pThrowInfo: u32) -> u32 {
    panic!("exception");
}
