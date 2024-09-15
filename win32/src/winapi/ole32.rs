#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use crate::Machine;

const TRACE_CONTEXT: &'static str = "ole32";

#[win32_derive::dllexport]
pub fn OleInitialize(_machine: &mut Machine, _pvReserved: u32) -> u32 {
    0
}
