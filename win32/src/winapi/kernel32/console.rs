use super::file::HFILE;
use crate::winapi::kernel32; // TODO: until we are in our own crate
use win32_system::System;
use win32_winapi::{DWORD, HANDLE, WORD, calling_convention::Array};

#[win32_derive::dllexport]
pub fn SetConsoleCtrlHandler(sys: &dyn System, _handlerRoutine: DWORD, _add: u32) -> bool {
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
    sys: &dyn System,
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

#[win32_derive::dllexport]
pub fn WriteConsoleA(
    sys: &dyn System,
    hConsoleOutput: HANDLE<()>,
    lpBuffer: Array<u8>,
    lpNumberOfCharsWritten: Option<&mut u32>,
    lpReserved: u32,
) -> bool {
    let msg = std::str::from_utf8(&lpBuffer).unwrap();
    log::debug!("WriteConsoleA: {:?}", msg);
    if let Some(w) = lpNumberOfCharsWritten {
        *w = msg.len() as u32;
    }
    true // success
}

#[win32_derive::dllexport]
pub fn WriteConsoleW(
    sys: &dyn System,
    hConsoleOutput: HFILE,
    lpBuffer: Array<u16>,
    lpNumberOfCharsWritten: Option<&mut u32>,
    _lpReserved: u32,
) -> bool {
    match kernel32::file::write_file(sys, hConsoleOutput, &lpBuffer) {
        Err(err) => {
            log::debug!("WriteConsoleW({hConsoleOutput:?}) failed: {:?}", err);
            false
        }
        Ok(n) => {
            if let Some(chars_written) = lpNumberOfCharsWritten {
                *chars_written = n as u32 / 2;
            }
            true
        }
    }
}

pub type CONSOLE_READCONSOLE_CONTROL = u32; // TODO

#[win32_derive::dllexport]
pub fn ReadConsoleA(
    sys: &dyn System,
    hConsoleInput: HANDLE<()>,
    lpBuffer: Option<&mut u8>,
    nNumberOfCharsToRead: u32,
    lpNumberOfCharsRead: Option<&mut u32>,
    pInputControl: Option<&mut CONSOLE_READCONSOLE_CONTROL>,
) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn SetConsoleMode(
    sys: &dyn System,
    hConsoleHandle: HANDLE<()>,
    dwMode: u32, /* CONSOLE_MODE */
) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn ReadConsoleInputA(
    sys: &dyn System,
    hConsoleInput: HANDLE<()>,
    lpBuffer: u32, // [INPUT_RECORD]
    nLength: u32,
    lpNumberOfEventsRead: Option<&mut u32>,
) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn PeekConsoleInputA(
    sys: &dyn System,
    hConsoleInput: HANDLE<()>,
    lpBuffer: u32, // [INPUT_RECORD]
    nLength: u32,
    lpNumberOfEventsRead: Option<&mut u32>,
) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn GetNumberOfConsoleInputEvents(
    sys: &dyn System,
    hConsoleInput: HANDLE<()>,
    lpNumberOfEvents: Option<&mut u32>,
) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn GetConsoleMode(sys: &dyn System, hConsoleHandle: HFILE, lpMode: Option<&mut u32>) -> bool {
    *lpMode.unwrap() = 0;
    true
}
