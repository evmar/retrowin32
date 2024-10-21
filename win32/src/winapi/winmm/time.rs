use memory::Pod;

use crate::machine::Machine;

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
pub fn timeKillEvent(_machine: &mut Machine, uTimerID: u32) -> u32 {
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

#[win32_derive::dllexport]
pub fn timeEndPeriod(_machine: &mut Machine, uPeriod: u32) -> u32 {
    // ignore
    TIMERR_NOERROR
}

#[repr(C)]
#[derive(Debug)]
pub struct TIMECAPS {
    uPeriodMin: u32,
    uPeriodMax: u32,
}
unsafe impl Pod for TIMECAPS {}

#[win32_derive::dllexport]
pub fn timeGetDevCaps(_machine: &mut Machine, ptc: Option<&mut TIMECAPS>, cbtc: u32) -> u32 {
    assert!(cbtc >= std::mem::size_of::<TIMECAPS>() as u32);
    *ptc.unwrap() = TIMECAPS {
        uPeriodMin: 1,
        uPeriodMax: 1,
    };
    0
}
