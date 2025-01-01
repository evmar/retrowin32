use crate::{
    str16::Str16,
    winapi::{
        stack_args::{FromArg, VarArgs},
        types::{HWND, POINT, RECT},
    },
    Machine,
};
use memory::{Extensions, ExtensionsMut};
use std::io::{Cursor, Write};

pub type HINSTANCE = u32;
pub type HKL = u32;
pub type HMONITOR = u32;

#[win32_derive::dllexport]
pub fn CharLowerA<'a>(machine: &mut Machine, lpsz: u32) -> u32 {
    if lpsz & 0xFFu32 == lpsz {
        let mut byte: u8 = lpsz as u8;
        byte.make_ascii_lowercase();
        byte as u32
    } else {
        todo!()
    }
}

#[win32_derive::dllexport]
pub fn CharLowerBuffA(machine: &mut Machine, lpsz: u32, cchLength: u32) -> u32 {
    machine
        .mem()
        .sub32_mut(lpsz, cchLength)
        .make_ascii_lowercase();
    cchLength
}

/// System metrics.
#[derive(Debug, win32_derive::TryFromEnum)]
pub enum SM {
    CXSCREEN = 0,
    CYSCREEN = 1,
    CXVSCROLL = 2,
    CYHSCROLL = 3,
    CYCAPTION = 4,
    CXBORDER = 5,
    CYBORDER = 6,
    CXDLGFRAME = 7,
    CYDLGFRAME = 8,
    CYVTHUMB = 9,
    CXHTHUMB = 10,
    CXICON = 11,
    CYICON = 12,
    CXCURSOR = 13,
    CYCURSOR = 14,
    CYMENU = 15,
    CXFULLSCREEN = 16,
    CYFULLSCREEN = 17,
    CYKANJIWINDOW = 18,
    MOUSEPRESENT = 19,
    CYVSCROLL = 20,
    CXHSCROLL = 21,
    DEBUG = 22,
    SWAPBUTTON = 23,
    // RESERVED1 = 24,
    // RESERVED2 = 25,
    // RESERVED3 = 26,
    // RESERVED4 = 27,
    CXMIN = 28,
    CYMIN = 29,
    CXSIZE = 30,
    CYSIZE = 31,
    CXFRAME = 32,
    CYFRAME = 33,
    CXMINTRACK = 34,
    CYMINTRACK = 35,
    CXDOUBLECLK = 36,
    CYDOUBLECLK = 37,
    CXICONSPACING = 38,
    CYICONSPACING = 39,
    MENUDROPALIGNMENT = 40,
    PENWINDOWS = 41,
    DBCSENABLED = 42,
    CMOUSEBUTTONS = 43,
    SECURE = 44,
    CXEDGE = 45,
    CYEDGE = 46,
    CXMINSPACING = 47,
    CYMINSPACING = 48,
    CXSMICON = 49,
    CYSMICON = 50,
    CYSMCAPTION = 51,
    CXSMSIZE = 52,
    CYSMSIZE = 53,
    CXMENUSIZE = 54,
    CYMENUSIZE = 55,
    ARRANGE = 56,
    CXMINIMIZED = 57,
    CYMINIMIZED = 58,
    CXMAXTRACK = 59,
    CYMAXTRACK = 60,
    CXMAXIMIZED = 61,
    CYMAXIMIZED = 62,
    NETWORK = 63,
    CLEANBOOT = 67,
    CXDRAG = 68,
    CYDRAG = 69,
    SHOWSOUNDS = 70,
    CXMENUCHECK = 71,
    CYMENUCHECK = 72,
    SLOWMACHINE = 73,
    MIDEASTENABLED = 74,
    MOUSEWHEELPRESENT = 75,
    XVIRTUALSCREEN = 76,
    YVIRTUALSCREEN = 77,
    CXVIRTUALSCREEN = 78,
    CYVIRTUALSCREEN = 79,
    CMONITORS = 80,
    SAMEDISPLAYFORMAT = 81,
    IMMENABLED = 82,
    CXFOCUSBORDER = 83,
    CYFOCUSBORDER = 84,
    TABLETPC = 86,
    MEDIACENTER = 87,
    STARTER = 88,
    SERVERR2 = 89,
    MOUSEHORIZONTALWHEELPRESENT = 91,
    CXPADDEDBORDER = 92,
    DIGITIZER = 94,
    MAXIMUMTOUCHES = 95,
}

#[win32_derive::dllexport]
pub fn GetSystemMetrics(_machine: &mut Machine, nIndex: Result<SM, u32>) -> u32 {
    // These were dumped from a win2k VM running at 640x480.
    // See exe/no_std/bin/metrics.rs.
    const METRICS: [u32; 100] = [
        640, 480, 16, 16, 19, 1, 1, 3, 3, 16, 16, 32, 32, 32, 32, 19, 640, 433, 0, 1, 16, 16, 0, 0,
        0, 0, 0, 0, 112, 27, 18, 18, 4, 4, 112, 27, 4, 4, 75, 75, 0, 0, 0, 5, 0, 2, 2, 160, 24, 16,
        16, 16, 12, 15, 18, 18, 8, 160, 24, 652, 492, 648, 460, 3, 0, 0, 0, 0, 4, 4, 0, 13, 13, 0,
        0, 1, 0, 0, 640, 480, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    METRICS[nIndex.unwrap() as usize]
}

#[win32_derive::dllexport]
pub fn GetSysColor(_machine: &mut Machine, nIndex: i32) -> u32 {
    todo!();
}

#[win32_derive::dllexport(cdecl)]
pub fn wsprintfA(machine: &mut Machine, buf: u32, fmt: Option<&str>, mut args: VarArgs) -> u32 {
    const BUF_LEN: u32 = 1024;
    let mem = machine.mem();
    let buf = mem.sub32_mut(buf, BUF_LEN);
    let mut out = Cursor::new(buf);

    fn read_number(c: u8) -> usize {
        // TODO: multiple digits, error handling, etc.
        assert!(c >= b'0' && c <= b'9');
        (c - b'0') as usize
    }

    let mut i = fmt.unwrap().bytes();
    while let Some(c) = i.next() {
        if c == b'%' {
            let mut c = i.next().unwrap();
            let mut width = 0;
            if c >= b'0' && c <= b'9' {
                width = read_number(c);
                c = i.next().unwrap();
            }
            let mut precision = 0;
            if c == b'.' {
                c = i.next().unwrap();
                precision = read_number(c);
                c = i.next().unwrap();
            }

            let mut long = false;
            if c == b'l' {
                long = true;
                c = i.next().unwrap();
            }
            _ = long; // currently ignored

            match c {
                b'u' => write!(
                    out,
                    "{:width$.precision$}",
                    args.pop::<u32>(mem),
                    width = width,
                    precision = precision
                )
                .unwrap(),
                b'd' => write!(
                    out,
                    "{:width$.precision$}",
                    args.pop::<i32>(mem),
                    width = width,
                    precision = precision
                )
                .unwrap(),
                b's' => {
                    let addr = args.pop::<u32>(mem);
                    let str = mem.slicez(addr);
                    write!(out, "{}", std::str::from_utf8(str).unwrap()).unwrap();
                }
                _ => todo!("format string character {:?}", c as char),
            }
        } else {
            out.write(&[c]).unwrap();
        }
    }
    out.write(&[0]).unwrap();
    // let len = out.position() as usize;
    // let buf = &out.into_inner()[..len];
    // log::info!("=> {}", std::str::from_utf8(buf).unwrap());
    // len as u32 - 1
    out.position() as u32 - 1
}

#[win32_derive::dllexport(cdecl)]
pub fn wsprintfW(machine: &mut Machine, buf: u32, fmt: Option<&Str16>, args: VarArgs) -> u32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn GetKeyState(_machine: &mut Machine, nVirtKey: u32) -> u32 {
    0
}

#[win32_derive::dllexport]
pub fn IsIconic(_machine: &mut Machine, hwnd: HWND) -> bool {
    false
}

#[win32_derive::dllexport]
pub fn WinHelpW(
    _machine: &mut Machine,
    hWndMain: HWND,
    lpszHelp: Option<&Str16>,
    uCommand: u32,
    dwData: u32,
) -> bool {
    todo!();
}

#[win32_derive::dllexport]
pub fn GetCursorPos(_machine: &mut Machine, lpPoint: Option<&mut POINT>) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn SetCursorPos(_machine: &mut Machine, x: i32, y: i32) -> bool {
    todo!();
}

#[win32_derive::dllexport]
pub fn GetKeyboardState(_machine: &mut Machine, lpKeyState: Option<&mut u8>) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn keybd_event(
    _machine: &mut Machine,
    bVk: u8,
    bScan: u8,
    dwFlags: u32, /* KEYBD_EVENT_FLAGS */
    dwExtraInfo: u32,
) {
    todo!()
}

#[win32_derive::dllexport]
pub fn SystemParametersInfoA(
    _machine: &mut Machine,
    uiAction: u32, /* SYSTEM_PARAMETERS_INFO_ACTION */
    uiParam: u32,
    pvParam: Option<&mut u8>,
    fWinIni: u32, /* SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS */
) -> bool {
    false // fail
}

#[win32_derive::dllexport]
pub fn GetKeyboardType(_machine: &mut Machine, nTypeFlag: i32) -> i32 {
    0 // fail
}

#[win32_derive::dllexport]
pub fn GetKeyboardLayout(_machine: &mut Machine, idThread: u32) -> u32 {
    log::warn!("GetKeyboardLayout: stub");
    0 // garbage value, unclear if callers care
}

#[win32_derive::dllexport]
pub fn GetKeyboardLayoutList(_machine: &mut Machine, nBuff: i32, lpList: Option<&mut HKL>) -> i32 {
    log::warn!("GetKeyboardLayoutList: stub");
    0 // no layouts
}

#[win32_derive::dllexport]
pub fn SetWindowsHookExA(
    _machine: &mut Machine,
    idHook: u32, /* WINDOWS_HOOK_ID */
    lpfn: u32,   /* HOOKPROC */
    hmod: HINSTANCE,
    dwThreadId: u32,
) -> u32 {
    0
}

#[repr(C)]
#[derive(Debug)]
pub struct MONITORINFO {
    pub cbSize: u32,
    pub rcMonitor: RECT,
    pub rcWork: RECT,
    pub dwFlags: u32,
}
unsafe impl memory::Pod for MONITORINFO {}

#[win32_derive::dllexport]
pub fn GetMonitorInfoA(
    _machine: &mut Machine,
    hMonitor: HMONITOR,
    lpmi: Option<&mut MONITORINFO>,
) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn OemToCharA(_machine: &mut Machine, pSrc: Option<&str>, pDst: Option<&str>) -> bool {
    true
}
