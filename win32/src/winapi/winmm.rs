#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use crate::machine::Machine;

const TRACE_CONTEXT: &'static str = "winmm";

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

#[win32_derive::dllexport]
pub fn timeGetTime(machine: &mut Machine) -> u32 {
    machine.host.time()
}

#[win32_derive::dllexport]
pub fn waveOutGetNumDevs(_machine: &mut Machine) -> u32 {
    0 // no sound yet
}
