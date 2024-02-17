use memory::Pod;

use crate::machine::Machine;

use super::{peb_mut, teb_mut};

const TRACE_CONTEXT: &'static str = "kernel32/thread";

#[win32_derive::dllexport]
pub fn GetCurrentThreadId(_machine: &mut Machine) -> u32 {
    1
}

#[win32_derive::dllexport]
pub fn TlsAlloc(machine: &mut Machine) -> u32 {
    let peb = peb_mut(machine);
    let slot = peb.TlsCount;
    peb.TlsCount = slot + 1;
    slot
}

#[win32_derive::dllexport]
pub fn TlsFree(machine: &mut Machine, dwTlsIndex: u32) -> bool {
    let peb = peb_mut(machine);
    if dwTlsIndex >= peb.TlsCount {
        log::warn!("TlsFree of unknown slot {dwTlsIndex}");
        return false;
    }
    // TODO
    true
}

#[win32_derive::dllexport]
pub fn TlsSetValue(machine: &mut Machine, dwTlsIndex: u32, lpTlsValue: u32) -> bool {
    let teb = teb_mut(machine);
    teb.TlsSlots[dwTlsIndex as usize] = lpTlsValue;
    true
}

#[win32_derive::dllexport]
pub fn TlsGetValue(machine: &mut Machine, dwTlsIndex: u32) -> u32 {
    let teb = teb_mut(machine);
    teb.TlsSlots[dwTlsIndex as usize]
}

#[win32_derive::dllexport]
pub fn CreateThread(
    _machine: &mut Machine,
    lpThreadAttributes: u32,
    dwStackSize: u32,
    lpStartAddress: u32,
    lpParameter: u32,
    dwCreationFlags: u32,
    lpThreadId: u32,
) -> u32 {
    log::warn!("CreateThread {lpThreadAttributes:x} {dwStackSize:x} {lpStartAddress:x} {lpParameter:x} {dwCreationFlags:x} {lpThreadId:x}");
    0
}

#[win32_derive::dllexport]
pub fn SetThreadPriority(_machine: &mut Machine, _hThread: u32, _nPriority: u32) -> bool {
    true // success
}

#[win32_derive::dllexport]
pub fn SetThreadStackGuarantee(_machine: &mut Machine, StackSizeInBytes: Option<&mut u32>) -> bool {
    // ignore
    true // success
}

#[win32_derive::dllexport]
pub fn InterlockedIncrement(_machine: &mut Machine, addend: Option<&mut u32>) -> u32 {
    let addend = addend.unwrap();
    *addend += 1;
    *addend
}

#[repr(C)]
#[derive(Debug)]
pub struct INIT_ONCE {
    ptr: u32,
}
unsafe impl Pod for INIT_ONCE {}

#[win32_derive::dllexport]
pub fn InitOnceBeginInitialize(
    _machine: &mut Machine,
    lpInitOnce: Option<&mut INIT_ONCE>,
    dwFlags: u32,
    fPending: Option<&mut u32>,
    lpContext: u32,
) -> bool {
    if dwFlags != 0 {
        todo!();
    }
    *fPending.unwrap() = 1;
    true
}

#[win32_derive::dllexport]
pub fn InitOnceComplete(
    _machine: &mut Machine,
    lpInitOnce: Option<&mut INIT_ONCE>,
    dwFlags: u32,
    lpContext: u32,
) -> bool {
    if dwFlags != 0 {
        todo!();
    }
    lpInitOnce.unwrap().ptr = lpContext;
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

#[repr(C)]
#[derive(Debug)]
pub struct SRWLOCK {
    ptr: u32,
}
unsafe impl Pod for SRWLOCK {}

#[win32_derive::dllexport]
pub fn AcquireSRWLockShared(_machine: &mut Machine, SRWLock: Option<&mut SRWLOCK>) -> u32 {
    0
}
