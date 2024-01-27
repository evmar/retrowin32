#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use crate::machine::Machine;

const TRACE_CONTEXT: &'static str = "winmm";

#[win32_derive::dllexport]
pub fn timeSetEvent(
    _machine: &mut Machine,
    uDelay: u32,
    uResolution: u32,
    lpTimeProc: u32,
    dwUser: u32,
    fuEvent: u32,
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

const TIMERR_NOERROR: u32 = 0;

#[win32_derive::dllexport]
pub fn timeBeginPeriod(_machine: &mut Machine, uPeriod: u32) -> u32 {
    // ignore
    TIMERR_NOERROR
}
