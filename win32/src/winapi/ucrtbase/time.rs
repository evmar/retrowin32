use crate::Machine;

#[win32_derive::dllexport(cdecl)]
pub fn time(machine: &mut Machine, destTime: Option<&mut u32>) -> u32 {
    // Officially time.h conditionally provides time as 32-bit or 64-bit
    // based on the define _USE_32BIT_TIME_T, but the msvcrt DLL
    // contains the 32-bit one.
    let time = machine.host.system_time().timestamp() as u32;
    if let Some(destTime) = destTime {
        *destTime = time;
    }
    time
}

#[win32_derive::dllexport(cdecl)]
pub fn _time64(machine: &mut Machine, destTime: Option<&mut u64>) -> u64 {
    let time = machine.host.system_time().timestamp() as u64;
    if let Some(destTime) = destTime {
        *destTime = time;
    }
    time
}
