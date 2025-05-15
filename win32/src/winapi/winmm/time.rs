use super::get_state;
use crate::{Machine, System, winapi::kernel32};
use memory::Pod;

pub struct TimeThread {
    timer_id: u32,
    delay: u32,
    callback: u32,
    user_data: u32,
}

#[derive(Debug, win32_derive::TryFromEnum)]
pub enum TIME {
    ONESHOT = 0,
    PERIODIC = 1,
}

#[win32_derive::dllexport]
pub async fn retrowin32_time_thread_main(sys: &mut dyn System) {
    let &TimeThread {
        timer_id,
        delay,
        callback,
        user_data,
    } = get_state(sys).time_thread.as_ref().unwrap();
    loop {
        kernel32::Sleep(sys, delay).await;
        sys.call_x86(callback, vec![timer_id, 0, user_data, 0, 0])
            .await;
    }
}

#[win32_derive::dllexport]
pub fn timeSetEvent(
    machine: &mut Machine,
    uDelay: u32,
    uResolution: u32,
    lpTimeProc: u32,
    dwUser: u32,
    fuEvent: Result<TIME, u32>,
) -> u32 {
    // TODO: fuEvent is a bitfield, but we only support ONESHOT and PERIODIC
    assert!(machine.state.winmm.time_thread.is_none());

    // TODO: only exactly one timer supported
    let timer_id = 1;
    machine.state.winmm.time_thread = Some(TimeThread {
        timer_id,
        delay: uDelay,
        callback: lpTimeProc,
        user_data: dwUser,
    });

    let retrowin32_time_thread_main =
        crate::loader::get_symbol(machine, "winmm.dll", "retrowin32_time_thread_main");

    let thread = kernel32::create_thread(machine, 0x1000);
    let cpu = machine.emu.x86.new_cpu();
    Machine::init_thread(cpu, &thread);
    cpu.regs.eip = retrowin32_time_thread_main;

    timer_id
}

#[win32_derive::dllexport]
pub fn timeKillEvent(sys: &dyn System, uTimerID: u32) -> u32 {
    0
}

#[win32_derive::dllexport]
pub fn timeGetTime(machine: &mut Machine) -> u32 {
    machine.host.ticks()
}

const TIMERR_NOERROR: u32 = 0;

#[win32_derive::dllexport]
pub fn timeBeginPeriod(sys: &dyn System, uPeriod: u32) -> u32 {
    // ignore
    TIMERR_NOERROR
}

#[win32_derive::dllexport]
pub fn timeEndPeriod(sys: &dyn System, uPeriod: u32) -> u32 {
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
pub fn timeGetDevCaps(sys: &dyn System, ptc: Option<&mut TIMECAPS>, cbtc: u32) -> u32 {
    assert!(cbtc >= std::mem::size_of::<TIMECAPS>() as u32);
    *ptc.unwrap() = TIMECAPS {
        uPeriodMin: 1,
        uPeriodMax: 1,
    };
    0
}
