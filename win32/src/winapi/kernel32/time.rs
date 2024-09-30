use super::{set_last_error, FILETIME};
use crate::{winapi::ERROR, Machine};
use chrono::{Datelike, Timelike};
use memory::{ExtensionsMut, Pod};

#[win32_derive::dllexport]
pub fn GetTickCount(machine: &mut Machine) -> u32 {
    machine.host.ticks()
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
    let ms = machine.host.ticks();
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
        .put_pod::<u32>(lpFrequency, QUERY_PERFORMANCE_FREQ);
    machine.mem().put_pod::<u32>(lpFrequency + 4, 0);
    true
}

#[win32_derive::dllexport]
pub fn GetSystemTimeAsFileTime(
    machine: &mut Machine,
    lpSystemTimeAsFileTime: Option<&mut FILETIME>,
) -> u32 {
    let date_time = machine.host.system_time().to_utc();
    if let Some(time) = lpSystemTimeAsFileTime {
        let Some(nanos) = date_time.timestamp_nanos_opt() else {
            log::warn!("GetSystemTimeAsFileTime: timestamp_nanos_opt failed");
            *time = FILETIME::zeroed();
            return 0;
        };
        *time = FILETIME::from_unix_nanos(nanos);
    }
    0
}

#[win32_derive::dllexport]
pub fn GetSystemTime(machine: &mut Machine, lpSystemTime: Option<&mut SYSTEMTIME>) -> u32 {
    let date_time = machine.host.system_time().naive_utc();
    if let Some(time) = lpSystemTime {
        *time = SYSTEMTIME::from_chrono(&date_time);
    }
    0
}

#[win32_derive::dllexport]
pub fn GetLocalTime(machine: &mut Machine, lpSystemTime: Option<&mut SYSTEMTIME>) -> u32 {
    let date_time = machine.host.system_time();
    if let Some(time) = lpSystemTime {
        *time = SYSTEMTIME::from_chrono(&date_time);
    }
    0
}

#[win32_derive::dllexport]
pub fn SystemTimeToFileTime(
    machine: &mut Machine,
    lpSystemTime: Option<&SYSTEMTIME>,
    lpFileTime: Option<&mut FILETIME>,
) -> bool {
    let Some(lpSystemTime) = lpSystemTime else {
        log::warn!("SystemTimeToFileTime: lpSystemTime is null");
        set_last_error(machine, ERROR::INVALID_DATA);
        return false;
    };
    let date_time = match chrono::NaiveDate::from_ymd_opt(
        lpSystemTime.wYear as i32,
        lpSystemTime.wMonth as u32,
        lpSystemTime.wDay as u32,
    )
    .and_then(|dt| {
        dt.and_hms_milli_opt(
            lpSystemTime.wHour as u32,
            lpSystemTime.wMinute as u32,
            lpSystemTime.wSecond as u32,
            lpSystemTime.wMilliseconds as u32,
        )
    }) {
        Some(dt) => dt.and_utc(),
        None => {
            log::warn!("SystemTimeToFileTime: invalid SYSTEMTIME");
            set_last_error(machine, ERROR::INVALID_DATA);
            return false;
        }
    };
    if let Some(time) = lpFileTime {
        let Some(nanos) = date_time.timestamp_nanos_opt() else {
            log::warn!("SystemTimeToFileTime: timestamp_nanos_opt failed");
            *time = FILETIME::zeroed();
            return false;
        };
        *time = FILETIME::from_unix_nanos(nanos);
    }
    true
}

#[win32_derive::dllexport]
pub fn FileTimeToSystemTime(
    machine: &mut Machine,
    lpFileTime: Option<&FILETIME>,
    lpSystemTime: Option<&mut SYSTEMTIME>,
) -> bool {
    let Some(lpFileTime) = lpFileTime else {
        log::warn!("FileTimeToSystemTime: lpFileTime is null");
        set_last_error(machine, ERROR::INVALID_DATA);
        return false;
    };
    let nanos = lpFileTime.to_unix_nanos();
    let date_time = chrono::DateTime::from_timestamp_nanos(nanos);
    if let Some(time) = lpSystemTime {
        *time = SYSTEMTIME::from_chrono(&date_time);
    }
    true
}

#[win32_derive::dllexport]
pub async fn Sleep(machine: &mut Machine, dwMilliseconds: u32) -> u32 {
    if dwMilliseconds == 0 {
        return 0;
    }

    #[cfg(feature = "x86-emu")]
    {
        let until = machine.host.ticks() + dwMilliseconds;
        machine.emu.x86.cpu_mut().block(Some(until)).await;
    }

    #[cfg(not(feature = "x86-emu"))]
    {
        _ = machine;
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
impl SYSTEMTIME {
    pub fn from_chrono<T>(dt: &T) -> Self
    where
        T: Datelike + Timelike,
    {
        Self {
            wYear: dt.year() as u16,
            wMonth: dt.month() as u16,
            wDayOfWeek: dt.weekday().num_days_from_sunday() as u16,
            wDay: dt.day() as u16,
            wHour: dt.hour() as u16,
            wMinute: dt.minute() as u16,
            wSecond: dt.second() as u16,
            wMilliseconds: (dt.nanosecond() / 1_000_000) as u16,
        }
    }
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
    *lpTimeZoneInformation.unwrap() = TIME_ZONE_INFORMATION::zeroed();
    TIME_ZONE_ID_UNKNOWN
}
