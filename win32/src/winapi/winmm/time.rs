use crate::machine::Machine;

const TRACE_CONTEXT: &'static str = "winmm/time";

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
    machine.host.ticks()
}

const TIMERR_NOERROR: u32 = 0;

#[win32_derive::dllexport]
pub fn timeBeginPeriod(_machine: &mut Machine, uPeriod: u32) -> u32 {
    // ignore
    TIMERR_NOERROR
}
