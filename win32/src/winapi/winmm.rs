#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use crate::machine::Machine;

const TRACE: bool = true;

#[win32_derive::dllexport]
pub fn timeSetEvent(
    _machine: &mut Machine,
    _uDelay: u32,
    _uResolution: u32,
    _lpTimeProc: u32,
    _dwUser: u32,
    _fuEvent: u32,
) -> u32 {
    0
}
