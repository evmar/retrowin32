use crate::{
    winapi::{
        calling_convention::ArrayWithSize, handle::HANDLE, kernel32::WriteFile, Str16, DWORD,
        HFILE, WORD,
    },
    Machine,
};

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

#[win32_derive::dllexport]
pub fn WriteConsoleA(
    _machine: &mut Machine,
    hConsoleOutput: HANDLE<()>,
    lpBuffer: ArrayWithSize<u8>,
    lpNumberOfCharsWritten: Option<&mut u32>,
    lpReserved: u32,
) -> bool {
    let msg = std::str::from_utf8(lpBuffer.unwrap()).unwrap();
    log::debug!("WriteConsoleA: {:?}", msg);
    if let Some(w) = lpNumberOfCharsWritten {
        *w = msg.len() as u32;
    }
    true // success
}

#[win32_derive::dllexport]
pub fn WriteConsoleW(
    machine: &mut Machine,
    hConsoleOutput: HFILE,
    lpBuffer: ArrayWithSize<u16>,
    lpNumberOfCharsWritten: Option<&mut u32>,
    _lpReserved: u32,
) -> bool {
    let buf = Str16::from_buffer(lpBuffer.unwrap()).to_string();
    let mut bytes_written = 0;
    if !WriteFile(
        machine,
        hConsoleOutput,
        Some(buf.as_bytes()),
        Some(&mut bytes_written),
        0,
    ) {
        return false;
    }
    if let Some(chars_written) = lpNumberOfCharsWritten {
        *chars_written = bytes_written;
    }
    return bytes_written == buf.len() as u32;
}

pub type CONSOLE_READCONSOLE_CONTROL = u32; // TODO

#[win32_derive::dllexport]
pub fn ReadConsoleA(
    _machine: &mut Machine,
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
    _machine: &mut Machine,
    hConsoleHandle: HANDLE<()>,
    dwMode: u32, /* CONSOLE_MODE */
) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn ReadConsoleInputA(
    _machine: &mut Machine,
    hConsoleInput: HANDLE<()>,
    lpBuffer: u32, // [INPUT_RECORD]
    nLength: u32,
    lpNumberOfEventsRead: Option<&mut u32>,
) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn PeekConsoleInputA(
    _machine: &mut Machine,
    hConsoleInput: HANDLE<()>,
    lpBuffer: u32, // [INPUT_RECORD]
    nLength: u32,
    lpNumberOfEventsRead: Option<&mut u32>,
) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn GetNumberOfConsoleInputEvents(
    _machine: &mut Machine,
    hConsoleInput: HANDLE<()>,
    lpNumberOfEvents: Option<&mut u32>,
) -> bool {
    todo!()
}
