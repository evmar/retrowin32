use crate::winapi::handle::HANDLE;
use crate::winapi::types::{DWORD, WORD};
use crate::Machine;

const TRACE_CONTEXT: &'static str = "kernel32/console";

#[win32_derive::dllexport]
pub fn SetConsoleCtrlHandler(_machine: &mut Machine, _handlerRoutine: DWORD, _add: u32) -> bool {
    true // succeed
}

#[repr(C)]
#[derive(Debug)]
pub struct COORD {
    X: i16,
    Y: i16,
}
unsafe impl ::memory::Pod for COORD {}

#[repr(C)]
#[derive(Debug)]
pub struct SMALL_RECT {
    Left: i16,
    Top: i16,
    Right: i16,
    Bottom: i16,
}
unsafe impl ::memory::Pod for SMALL_RECT {}

#[repr(C)]
#[derive(Debug)]
pub struct CONSOLE_SCREEN_BUFFER_INFO {
    dwSize: COORD,
    dwCursorPosition: COORD,
    wAttributes: WORD,
    srWindow: SMALL_RECT,
    dwMaximumWindowSize: COORD,
}
unsafe impl ::memory::Pod for CONSOLE_SCREEN_BUFFER_INFO {}

#[win32_derive::dllexport]
pub fn GetConsoleScreenBufferInfo(
    _machine: &mut Machine,
    _hConsoleOutput: HANDLE<()>,
    lpConsoleScreenBufferInfo: Option<&mut CONSOLE_SCREEN_BUFFER_INFO>,
) -> bool {
    if let Some(info) = lpConsoleScreenBufferInfo {
        info.dwSize = COORD { X: 80, Y: 25 };
        info.dwCursorPosition = COORD { X: 0, Y: 0 };
        info.wAttributes = 0;
        info.srWindow = SMALL_RECT {
            Left: 0,
            Top: 0,
            Right: 79,
            Bottom: 24,
        };
        info.dwMaximumWindowSize = COORD { X: 80, Y: 25 };
    }
    true // success
}
