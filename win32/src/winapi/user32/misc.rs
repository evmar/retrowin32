use crate::{
    str16::Str16,
    winapi::{
        stack_args::VarArgs,
        types::{HWND, POINT},
    },
    Machine,
};
use memory::{Extensions, ExtensionsMut};
use std::io::{Cursor, Write};

pub type HINSTANCE = u32;

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
    RESERVED1 = 24,
    RESERVED2 = 25,
    RESERVED3 = 26,
    RESERVED4 = 27,
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
    match nIndex.unwrap() {
        SM::CXSCREEN => 640,
        SM::CYSCREEN => 480,
        SM::CYCAPTION => 19,
        SM::CXBORDER => 1,
        SM::CYBORDER => 1,
        SM::CYMENU => 19,
        SM::CXFRAME => 4,
        SM::CYFRAME => 4,
        SM::CXVIRTUALSCREEN => 640,
        SM::CYVIRTUALSCREEN => 480,
        metric => todo!("GetSystemMetrics({metric:?})"),
    }
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
    todo!()
}

#[win32_derive::dllexport]
pub fn GetKeyboardType(_machine: &mut Machine, nTypeFlag: i32) -> i32 {
    0 // fail
}
