#![allow(non_snake_case)]

use crate::Machine;

const TRACE_CONTEXT: &'static str = "ucrtbase";

#[win32_derive::dllexport]
pub fn _initterm(_machine: &mut Machine, start: u32, end: u32) -> u32 {
    0
}

#[win32_derive::dllexport]
pub fn _initterm_e(_machine: &mut Machine, start: u32, end: u32) -> u32 {
    0
}

#[win32_derive::dllexport]
pub fn _get_initial_narrow_environment(_machine: &mut Machine) -> u32 {
    0
}

#[win32_derive::dllexport]
pub fn __p___argv(_machine: &mut Machine) -> u32 {
    0
}

#[win32_derive::dllexport]
pub fn __p___argc(_machine: &mut Machine) -> u32 {
    0
}
