#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use crate::machine::Machine;

#[win32_derive::dllexport]
pub fn timeSetEvent(
    machine: &mut Machine,
    uDelay: u32,
    uResolution: u32,
    lpTimeProc: u32,
    dwUser: u32,
    fuEvent: u32,
) -> u32 {
    0
}
