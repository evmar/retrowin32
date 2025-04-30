use crate::System;
use memory::Pod;

#[repr(C)]
#[derive(Debug)]
pub struct SRWLOCK {
    ptr: u32,
}
unsafe impl Pod for SRWLOCK {}

#[win32_derive::dllexport]
pub fn AcquireSRWLockShared(sys: &dyn System, SRWLock: Option<&mut SRWLOCK>) -> u32 {
    0
}

#[win32_derive::dllexport]
pub fn ReleaseSRWLockShared(sys: &dyn System, SRWLock: Option<&mut SRWLOCK>) -> u32 {
    0
}

#[win32_derive::dllexport]
pub fn AcquireSRWLockExclusive(sys: &dyn System, SRWLock: Option<&mut SRWLOCK>) -> u32 {
    0
}

#[win32_derive::dllexport]
pub fn TryAcquireSRWLockExclusive(sys: &dyn System, SRWLock: Option<&mut SRWLOCK>) -> u32 {
    0
}

#[win32_derive::dllexport]
pub fn ReleaseSRWLockExclusive(sys: &dyn System, SRWLock: Option<&mut SRWLOCK>) -> u32 {
    0
}
