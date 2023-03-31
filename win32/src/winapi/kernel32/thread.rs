use crate::machine::Machine;

use super::{peb_mut, teb_mut};

const TRACE: bool = true;

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
