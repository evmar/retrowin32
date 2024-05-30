use super::FILETIME;
use crate::Machine;
use memory::Pod;

const TRACE_CONTEXT: &'static str = "kernel32/time";

#[win32_derive::dllexport]
pub fn GetTickCount(machine: &mut Machine) -> u32 {
    machine.host.time()
}

// The number of "counts" per second, where counts are the units returned by
// QueryPerformanceCounter.  On my Windows machine this value was 10m, which
// is to say a count is 0.1us.
const QUERY_PERFORMANCE_FREQ: u32 = 10_000_000;

// In principle we could just use an i64 here, but when Windows passes one of these via
// the stack it may align it on a 4-byte address when Rust requires 8-byte alignment for
// 64-bit addresses.  So instead we more closely match the Windows behavior.
#[repr(C)]
#[derive(Debug)]
pub struct LARGE_INTEGER {
    LowPart: u32,
    HighPart: i32,
}
unsafe impl memory::Pod for LARGE_INTEGER {}

#[win32_derive::dllexport]
pub fn QueryPerformanceCounter(
    machine: &mut Machine,
    lpPerformanceCount: Option<&mut LARGE_INTEGER>,
) -> bool {
    let counter = lpPerformanceCount.unwrap();
    let ms = machine.host.time();
    let counts = ms as u64 * (QUERY_PERFORMANCE_FREQ as u64 / 1000);
    counter.LowPart = counts as u32;
    counter.HighPart = (counts >> 32) as u32 as i32;
    true // success
}

#[win32_derive::dllexport]
pub fn QueryPerformanceFrequency(machine: &mut Machine, lpFrequency: u32) -> bool {
    // 64-bit write
    machine
        .mem()
        .put::<u32>(lpFrequency, QUERY_PERFORMANCE_FREQ);
    machine.mem().put::<u32>(lpFrequency + 4, 0);
    true
}

#[win32_derive::dllexport]
pub fn GetSystemTimeAsFileTime(_machine: &mut Machine, _time: Option<&mut FILETIME>) -> u32 {
    0
}

#[win32_derive::dllexport]
pub async fn Sleep(machine: &mut Machine, dwMilliseconds: u32) -> u32 {
    #[cfg(feature = "x86-emu")]
    {
        let until = machine.host.time() + dwMilliseconds;
        machine.emu.x86.cpu_mut().block(Some(until)).await;
    }

    #[cfg(not(feature = "x86-emu"))]
    {
        log::warn!("TODO: sleep");
    }
    0
}

#[repr(C)]
#[derive(Debug)]
pub struct SYSTEMTIME {
    wYear: u16,
    wMonth: u16,
    wDayOfWeek: u16,
    wDay: u16,
    wHour: u16,
    wMinute: u16,
    wSecond: u16,
    wMilliseconds: u16,
}
unsafe impl memory::Pod for SYSTEMTIME {}

#[repr(C)]
#[derive(Debug)]
pub struct TIME_ZONE_INFORMATION {
    Bias: u32,
    StandardName: [u16; 32],
    StandardDate: SYSTEMTIME,
    StandardBias: u32,
    DaylightName: [u16; 32],
    DaylightDate: SYSTEMTIME,
    DaylightBias: u32,
}
unsafe impl memory::Pod for TIME_ZONE_INFORMATION {}

#[win32_derive::dllexport]
pub fn GetTimeZoneInformation(
    _machine: &mut Machine,
    lpTimeZoneInformation: Option<&mut TIME_ZONE_INFORMATION>,
) -> u32 {
    const TIME_ZONE_ID_UNKNOWN: u32 = 0;
    // It appears to be legal for all struct members to be zero, especially
    // while TIME_ZONE_ID_UNKNOWN says there's no daylight savings time.
    lpTimeZoneInformation.unwrap().clear_struct();
    TIME_ZONE_ID_UNKNOWN
}
