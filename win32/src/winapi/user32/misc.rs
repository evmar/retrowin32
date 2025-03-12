use crate::{
    calling_convention::VarArgs,
    winapi::{Str16, HWND, POINT, RECT},
    Machine,
};
use memory::ExtensionsMut;
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
pub fn wsprintfA(machine: &mut Machine, buf: u32, fmt: Option<&str>, args: VarArgs) -> u32 {
    // The output buffer size is unspecified (eek) but is maximum 1024.
    const BUF_LEN: u32 = 1024;
    let mem = machine.mem();
    let buf = mem.sub32_mut(buf, BUF_LEN);

    let mut out = Cursor::new(buf);
    crate::winapi::printf::printf(&mut out, fmt.unwrap(), args, mem).unwrap();
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

#[derive(Debug, win32_derive::TryFromEnum)]
pub enum SPI {
    GETBEEP = 0x0001,
    SETBEEP = 0x0002,
    GETMOUSE = 0x0003,
    SETMOUSE = 0x0004,
    GETBORDER = 0x0005,
    SETBORDER = 0x0006,
    GETKEYBOARDSPEED = 0x000A,
    SETKEYBOARDSPEED = 0x000B,
    LANGDRIVER = 0x000C,
    ICONHORIZONTALSPACING = 0x000D,
    GETSCREENSAVETIMEOUT = 0x000E,
    SETSCREENSAVETIMEOUT = 0x000F,
    GETSCREENSAVEACTIVE = 0x0010,
    SETSCREENSAVEACTIVE = 0x0011,
    GETGRIDGRANULARITY = 0x0012,
    SETGRIDGRANULARITY = 0x0013,
    SETDESKWALLPAPER = 0x0014,
    SETDESKPATTERN = 0x0015,
    GETKEYBOARDDELAY = 0x0016,
    SETKEYBOARDDELAY = 0x0017,
    ICONVERTICALSPACING = 0x0018,
    GETICONTITLEWRAP = 0x0019,
    SETICONTITLEWRAP = 0x001A,
    GETMENUDROPALIGNMENT = 0x001B,
    SETMENUDROPALIGNMENT = 0x001C,
    SETDOUBLECLKWIDTH = 0x001D,
    SETDOUBLECLKHEIGHT = 0x001E,
    GETICONTITLELOGFONT = 0x001F,
    SETDOUBLECLICKTIME = 0x0020,
    SETMOUSEBUTTONSWAP = 0x0021,
    SETICONTITLELOGFONT = 0x0022,
    GETFASTTASKSWITCH = 0x0023,
    SETFASTTASKSWITCH = 0x0024,
    SETDRAGFULLWINDOWS = 0x0025,
    GETDRAGFULLWINDOWS = 0x0026,
    GETNONCLIENTMETRICS = 0x0029,
    SETNONCLIENTMETRICS = 0x002A,
    GETMINIMIZEDMETRICS = 0x002B,
    SETMINIMIZEDMETRICS = 0x002C,
    GETICONMETRICS = 0x002D,
    SETICONMETRICS = 0x002E,
    SETWORKAREA = 0x002F,
    GETWORKAREA = 0x0030,
    SETPENWINDOWS = 0x0031,
    GETFILTERKEYS = 0x0032,
    SETFILTERKEYS = 0x0033,
    GETTOGGLEKEYS = 0x0034,
    SETTOGGLEKEYS = 0x0035,
    GETMOUSEKEYS = 0x0036,
    SETMOUSEKEYS = 0x0037,
    GETSHOWSOUNDS = 0x0038,
    SETSHOWSOUNDS = 0x0039,
    GETSTICKYKEYS = 0x003A,
    SETSTICKYKEYS = 0x003B,
    GETACCESSTIMEOUT = 0x003C,
    SETACCESSTIMEOUT = 0x003D,
    GETSERIALKEYS = 0x003E,
    SETSERIALKEYS = 0x003F,
    GETSOUNDSENTRY = 0x0040,
    SETSOUNDSENTRY = 0x0041,
    GETHIGHCONTRAST = 0x0042,
    SETHIGHCONTRAST = 0x0043,
    GETKEYBOARDPREF = 0x0044,
    SETKEYBOARDPREF = 0x0045,
    GETSCREENREADER = 0x0046,
    SETSCREENREADER = 0x0047,
    GETANIMATION = 0x0048,
    SETANIMATION = 0x0049,
    GETFONTSMOOTHING = 0x004A,
    SETFONTSMOOTHING = 0x004B,
    SETDRAGWIDTH = 0x004C,
    SETDRAGHEIGHT = 0x004D,
    SETHANDHELD = 0x004E,
    GETLOWPOWERTIMEOUT = 0x004F,
    GETPOWEROFFTIMEOUT = 0x0050,
    SETLOWPOWERTIMEOUT = 0x0051,
    SETPOWEROFFTIMEOUT = 0x0052,
    GETLOWPOWERACTIVE = 0x0053,
    GETPOWEROFFACTIVE = 0x0054,
    SETLOWPOWERACTIVE = 0x0055,
    SETPOWEROFFACTIVE = 0x0056,
    SETCURSORS = 0x0057,
    SETICONS = 0x0058,
    GETDEFAULTINPUTLANG = 0x0059,
    SETDEFAULTINPUTLANG = 0x005A,
    SETLANGTOGGLE = 0x005B,
    GETWINDOWSEXTENSION = 0x005C,
    SETMOUSETRAILS = 0x005D,
    GETMOUSETRAILS = 0x005E,
    GETSNAPTODEFBUTTON = 0x005F,
    SETSNAPTODEFBUTTON = 0x0060,
    SETSCREENSAVERRUNNING = 0x0061,
    GETMOUSEHOVERWIDTH = 0x0062,
    SETMOUSEHOVERWIDTH = 0x0063,
    GETMOUSEHOVERHEIGHT = 0x0064,
    SETMOUSEHOVERHEIGHT = 0x0065,
    GETMOUSEHOVERTIME = 0x0066,
    SETMOUSEHOVERTIME = 0x0067,
    GETWHEELSCROLLLINES = 0x0068,
    SETWHEELSCROLLLINES = 0x0069,
    GETMENUSHOWDELAY = 0x006A,
    SETMENUSHOWDELAY = 0x006B,
}

#[win32_derive::dllexport]
pub fn SystemParametersInfoA(
    _machine: &mut Machine,
    uiAction: Result<SPI, u32>,
    uiParam: u32,
    pvParam: u32,
    fWinIni: u32, /* SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS */
) -> bool {
    let action = uiAction.unwrap();
    match action {
        SPI::SETSCREENSAVEACTIVE => {}
        action => {
            // Almost all of these don't matter, but crash here because
            // in case of a GET* the caller will expect a value back.
            todo!("{:?}", action);
        }
    }
    true
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

#[win32_derive::dllexport]
pub fn RegisterClipboardFormatA(_machine: &mut Machine, lpszFormat: Option<&str>) -> bool {
    log::warn!("RegisterClipboardFormatA: stub");
    false
}

#[derive(Debug)]
#[allow(unused)]
pub struct TRACKMOUSEEVENT {
    cbSize: u32,
    dwFlags: u32, /* TRACKMOUSEEVENT_FLAGS */
    hwndTrack: HWND,
    dwHoverTime: u32,
}
unsafe impl ::memory::Pod for TRACKMOUSEEVENT {}

#[win32_derive::dllexport]
pub fn TrackMouseEvent(_machine: &mut Machine, lpEventTrack: Option<&mut TRACKMOUSEEVENT>) -> bool {
    false // fail
}
