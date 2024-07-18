#![allow(non_snake_case)]

use super::kernel32::ExitProcess;
use crate::Machine;

const TRACE_CONTEXT: &'static str = "ucrtbase";

#[win32_derive::dllexport(cdecl)]
pub fn _initterm(_machine: &mut Machine, start: u32, end: u32) -> u32 {
    0
}

#[win32_derive::dllexport(cdecl)]
pub fn _initterm_e(_machine: &mut Machine, start: u32, end: u32) -> u32 {
    0
}

#[win32_derive::dllexport(cdecl)]
pub fn _get_initial_narrow_environment(_machine: &mut Machine) -> u32 {
    0
}

#[win32_derive::dllexport(cdecl)]
pub fn __p___argv(_machine: &mut Machine) -> u32 {
    0
}

#[win32_derive::dllexport(cdecl)]
pub fn __p___argc(_machine: &mut Machine) -> u32 {
    0
}

#[win32_derive::dllexport(cdecl)]
pub fn exit(machine: &mut Machine, status: u32) -> u32 {
    ExitProcess(machine, status)
}

#[win32_derive::dllexport(cdecl)]
pub fn _lock(_machine: &mut Machine, locknum: u32) -> u32 {
    0
}

#[win32_derive::dllexport(cdecl)]
pub fn _unlock(_machine: &mut Machine, locknum: u32) -> u32 {
    0
}

#[win32_derive::dllexport(cdecl)]
pub fn __dllonexit(_machine: &mut Machine, func: u32, d: u32, f: u32) -> u32 {
    0
}
