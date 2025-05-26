use win32_system::System;

#[win32_derive::dllexport]
pub fn InitializeCriticalSection(sys: &dyn System, lpCriticalSection: u32) {}

#[win32_derive::dllexport]
pub fn InitializeCriticalSectionEx(
    sys: &dyn System,
    lpCriticalSection: u32,
    dwSpinCount: u32,
    flags: u32,
) -> bool {
    true
}

#[win32_derive::dllexport]
pub fn InitializeCriticalSectionAndSpinCount(
    sys: &dyn System,
    lpCriticalSection: u32,
    dwSpinCount: u32,
) -> bool {
    // "On single-processor systems, the spin count is ignored and the critical section spin count is set to 0 (zero)."
    // "This function always succeeds and returns a nonzero value."
    true
}

#[win32_derive::dllexport]
pub fn DeleteCriticalSection(sys: &dyn System, lpCriticalSection: u32) {}

#[win32_derive::dllexport]
pub fn EnterCriticalSection(sys: &dyn System, lpCriticalSection: u32) {}

#[win32_derive::dllexport]
pub fn LeaveCriticalSection(sys: &dyn System, lpCriticalSection: u32) {}
