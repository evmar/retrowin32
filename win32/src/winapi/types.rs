//! Types exposed by the Windows API.

pub use super::handle::HANDLE;
pub use crate::str16::{Str16, String16};
use memory::Extensions;

pub type WORD = u16;
pub type DWORD = u32;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct HFILET;
pub type HFILE = HANDLE<HFILET>;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct HWNDT;
pub type HWND = HANDLE<HWNDT>;

#[repr(C, packed)]
#[derive(Debug, Default)]
pub struct RECT {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}
unsafe impl memory::Pod for RECT {}

#[repr(C, packed)]
#[derive(Debug)]
pub struct POINT {
    pub x: DWORD,
    pub y: DWORD,
}
unsafe impl memory::Pod for POINT {}

impl<'a> super::stack_args::FromStack<'a> for POINT {
    unsafe fn from_stack(mem: memory::Mem<'a>, sp: u32) -> Self {
        let x = mem.get_pod::<u32>(sp + 4);
        let y = mem.get_pod::<u32>(sp);
        POINT { x, y }
    }
}

// https://learn.microsoft.com/en-us/windows/win32/debug/system-error-codes--0-499-
pub const ERROR_FILE_NOT_FOUND: u32 = 2;
pub const ERROR_ACCESS_DENIED: u32 = 5;
pub const ERROR_INVALID_ACCESS: u32 = 12;
pub const ERROR_INVALID_DATA: u32 = 13;
pub const ERROR_FILE_EXISTS: u32 = 80;
pub const ERROR_OPEN_FAILED: u32 = 110;
pub const ERROR_ALREADY_EXISTS: u32 = 183;

pub fn win32_error_str(code: u32) -> &'static str {
    match code {
        ERROR_FILE_NOT_FOUND => "ERROR_FILE_NOT_FOUND",
        ERROR_ACCESS_DENIED => "ERROR_ACCESS_DENIED",
        ERROR_INVALID_ACCESS => "ERROR_INVALID_ACCESS",
        ERROR_INVALID_DATA => "ERROR_INVALID_DATA",
        ERROR_FILE_EXISTS => "ERROR_FILE_EXISTS",
        ERROR_OPEN_FAILED => "ERROR_OPEN_FAILED",
        ERROR_ALREADY_EXISTS => "ERROR_ALREADY_EXISTS",
        _ => "ERROR_UNKNOWN",
    }
}
