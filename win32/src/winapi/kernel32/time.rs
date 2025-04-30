use crate::{Machine, System};
use chrono::{Datelike, Timelike, Utc};
use memory::{ExtensionsMut, Pod};

#[win32_derive::dllexport]
pub fn GetTickCount(machine: &mut Machine) -> u32 {
    machine.host.ticks()
}

// The number of "counts" per second, where counts are the units returned by
// QueryPerformanceCounter.  On my Windows machine this value was 10m, which
// is to say a count is 0.1us.
const QUERY_PERFORMANCE_FREQ: u32 = 10_000_000;

// This is effectively a 64-bit integer but defined as two u32s for alignment reasons.
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

/// Contains a 64-bit value representing the number of 100-nanosecond intervals since
/// January 1, 1601 (UTC).
/// However, functions like FileTimeToLocalFileTime stuff local time into it anyway, ugh.
// This is effectively a 64-bit integer but defined as two u32s for alignment reasons.
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct FILETIME {
    pub dwLowDateTime: u32,
    pub dwHighDateTime: u32,
}
unsafe impl memory::Pod for FILETIME {}

/// Number of 100ns intervals between 1601-01-01 and 1970-01-01.
/// Used to convert between Win32 FILETIME and Unix time.
const HNSEC_UNIX_OFFSET: i64 = 116_444_736_000_000_000;

impl FILETIME {
    pub fn from_chrono(time: chrono::DateTime<Utc>) -> Self {
        let nanos = time.timestamp_nanos_opt().unwrap();
        Self::from_unix_nanos(nanos)
    }

    pub fn to_chrono(self) -> chrono::DateTime<Utc> {
        let nanos = self.to_unix_nanos();
        chrono::DateTime::from_timestamp_nanos(nanos)
    }

    pub fn from_u64(value: u64) -> Self {
        FILETIME {
            dwLowDateTime: value as u32,
            dwHighDateTime: (value >> 32) as u32,
        }
    }

    pub fn to_u64(self) -> u64 {
        (self.dwHighDateTime as u64) << 32 | self.dwLowDateTime as u64
    }

    pub fn from_unix_nanos(nanos: i64) -> Self {
        let hnsecs = nanos.div_euclid(100).saturating_add(HNSEC_UNIX_OFFSET);
        Self::from_u64(if hnsecs < 0 { 0 } else { hnsecs as u64 })
    }

    pub fn to_unix_nanos(self) -> i64 {
        let hnsecs = self.to_u64();
        if hnsecs > i64::MAX as u64 {
            return i64::MAX;
        }
        (hnsecs as i64).saturating_sub(HNSEC_UNIX_OFFSET) * 100
    }
}

#[win32_derive::dllexport]
pub fn GetSystemTimeAsFileTime(
    machine: &mut Machine,
    lpSystemTimeAsFileTime: Option<&mut FILETIME>,
) {
    let date_time = machine.host.system_time().to_utc();
    *lpSystemTimeAsFileTime.unwrap() = FILETIME::from_chrono(date_time);
}

#[win32_derive::dllexport]
pub fn GetSystemTime(machine: &mut Machine, lpSystemTime: Option<&mut SYSTEMTIME>) {
    let date_time = machine.host.system_time().to_utc();
    *lpSystemTime.unwrap() = SYSTEMTIME::from_chrono(&date_time);
}

#[win32_derive::dllexport]
pub fn GetLocalTime(machine: &mut Machine, lpSystemTime: Option<&mut SYSTEMTIME>) {
    let date_time = machine.host.system_time();
    *lpSystemTime.unwrap() = SYSTEMTIME::from_chrono(&date_time);
}

#[win32_derive::dllexport]
pub fn SetLocalTime(sys: &dyn System, lpSystemTime: Option<&mut SYSTEMTIME>) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn SystemTimeToFileTime(
    machine: &mut Machine,
    lpSystemTime: Option<&SYSTEMTIME>,
    lpFileTime: Option<&mut FILETIME>,
) -> bool {
    let date_time = lpSystemTime.unwrap().to_chrono().unwrap();
    *lpFileTime.unwrap() = FILETIME::from_chrono(date_time.and_utc());
    true
}

#[win32_derive::dllexport]
pub fn FileTimeToSystemTime(
    machine: &mut Machine,
    lpFileTime: Option<&FILETIME>,
    lpSystemTime: Option<&mut SYSTEMTIME>,
) -> bool {
    let date_time = lpFileTime.unwrap().to_chrono();
    *lpSystemTime.unwrap() = SYSTEMTIME::from_chrono(&date_time);
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

#[win32_derive::dllexport]
pub async fn SleepEx(machine: &mut Machine, dwMilliseconds: u32, bAlertable: bool) -> u32 {
    Sleep(machine, dwMilliseconds).await
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

    pub fn to_chrono(&self) -> Option<chrono::NaiveDateTime> {
        let date = chrono::NaiveDate::from_ymd_opt(
            self.wYear as i32,
            self.wMonth as u32,
            self.wDay as u32,
        )?;
        date.and_hms_milli_opt(
            self.wHour as u32,
            self.wMinute as u32,
            self.wSecond as u32,
            self.wMilliseconds as u32,
        )
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
    sys: &dyn System,
    lpTimeZoneInformation: Option<&mut TIME_ZONE_INFORMATION>,
) -> u32 {
    const TIME_ZONE_ID_UNKNOWN: u32 = 0;
    // It appears to be legal for all struct members to be zero, especially
    // while TIME_ZONE_ID_UNKNOWN says there's no daylight savings time.
    *lpTimeZoneInformation.unwrap() = TIME_ZONE_INFORMATION::zeroed();
    TIME_ZONE_ID_UNKNOWN
}

#[win32_derive::dllexport]
pub fn FileTimeToLocalFileTime(
    sys: &dyn System,
    lpFileTime: Option<&FILETIME>,
    lpLocalFileTime: Option<&mut FILETIME>,
) -> bool {
    // FILETIME is officially UTC, but this function wants it converted to local time and
    // stuffed back in to a FILETIME.
    // TODO: I might have this backwards.
    let time = lpFileTime
        .unwrap()
        .to_chrono()
        .with_timezone(&chrono::Local);
    *lpLocalFileTime.unwrap() = FILETIME::from_chrono(
        time.naive_local() // strip timezone
            .and_utc(),
    );
    true
}

#[win32_derive::dllexport]
pub fn LocalFileTimeToFileTime(
    sys: &dyn System,
    lpLocalFileTime: Option<&mut FILETIME>,
    lpFileTime: Option<&mut FILETIME>,
) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn FileTimeToDosDateTime(
    sys: &dyn System,
    lpFileTime: Option<&FILETIME>,
    lpFatDate: Option<&mut u16>,
    lpFatTime: Option<&mut u16>,
) -> bool {
    let time = lpFileTime.unwrap().to_chrono();
    *lpFatDate.unwrap() =
        (time.year() as u16 - 1980) << 9 | (time.month() as u16) << 5 | time.day() as u16;
    *lpFatTime.unwrap() =
        (time.hour() as u16) << 11 | (time.minute() as u16) << 5 | (time.second() as u16 / 2);
    true
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    // This test is tz-dependent:
    #[test]
    fn FileTimeToLocalFileTime() {
        use chrono::TimeZone;

        let utc = chrono::Utc.ymd(2021, 1, 1).and_hms(0, 0, 0);
        let file_time = FILETIME::from_chrono(utc);
        let mut out_file_time = FILETIME::from_chrono(utc);
        assert!(super::FileTimeToLocalFileTime(
            Machine::null(),
            Some(&file_time),
            Some(&mut out_file_time),
        ));
        eprintln!("file_time: {:?}", file_time.to_chrono());
        eprintln!("file_time: {:?}", out_file_time.to_chrono());
        assert!(file_time != out_file_time);
    }
}
*/
