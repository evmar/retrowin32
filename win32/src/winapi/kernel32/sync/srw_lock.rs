use crate::Machine;
use memory::Pod;

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

#[win32_derive::dllexport]
pub fn ReleaseSRWLockShared(_machine: &mut Machine, SRWLock: Option<&mut SRWLOCK>) -> u32 {
    0
}

#[win32_derive::dllexport]
pub fn AcquireSRWLockExclusive(_machine: &mut Machine, SRWLock: Option<&mut SRWLOCK>) -> u32 {
    0
}

#[win32_derive::dllexport]
pub fn TryAcquireSRWLockExclusive(_machine: &mut Machine, SRWLock: Option<&mut SRWLOCK>) -> u32 {
    0
}

#[win32_derive::dllexport]
pub fn ReleaseSRWLockExclusive(_machine: &mut Machine, SRWLock: Option<&mut SRWLOCK>) -> u32 {
    0
}
