use crate::Machine;

#[win32_derive::dllexport]
pub fn InitializeCriticalSection(_machine: &mut Machine, lpCriticalSection: u32) -> bool {
    true
}

#[win32_derive::dllexport]
pub fn InitializeCriticalSectionEx(
    _machine: &mut Machine,
    lpCriticalSection: u32,
    dwSpinCount: u32,
    flags: u32,
) -> bool {
    true
}

#[win32_derive::dllexport]
pub fn InitializeCriticalSectionAndSpinCount(
    _machine: &mut Machine,
    lpCriticalSection: u32,
    dwSpinCount: u32,
) -> bool {
    // "On single-processor systems, the spin count is ignored and the critical section spin count is set to 0 (zero)."
    // "This function always succeeds and returns a nonzero value."
    true
}

#[win32_derive::dllexport]
pub fn DeleteCriticalSection(_machine: &mut Machine, lpCriticalSection: u32) -> u32 {
    0
}

#[win32_derive::dllexport]
pub fn EnterCriticalSection(_machine: &mut Machine, lpCriticalSection: u32) -> u32 {
    0
}

#[win32_derive::dllexport]
pub fn LeaveCriticalSection(_machine: &mut Machine, lpCriticalSection: u32) -> u32 {
    0
}
