#![allow(non_snake_case)]

mod message;
mod resource;
mod window;

pub use super::gdi32::HDC;
use super::handle::Handles;
use super::types::*;
use crate::machine::Machine;
pub use message::*;
pub use resource::*;
use std::collections::VecDeque;
use std::rc::Rc;
pub use window::*;

pub use super::kernel32::ResourceId;

const TRACE_CONTEXT: &'static str = "user32";

type HINSTANCE = u32;

pub struct State {
    wndclasses: Vec<Rc<WndClass>>,
    pub windows: Handles<HWND, Window>,
    messages: VecDeque<MSG>,
    /// TODO: we vend new timer ids but never fire the timers
    next_timer: u32,
}
impl Default for State {
    fn default() -> Self {
        Self {
            wndclasses: Default::default(),
            windows: Default::default(),
            messages: Default::default(),
            next_timer: 1,
        }
    }
}
impl State {
    pub fn get_window(&mut self, hwnd: HWND) -> Option<&mut Window> {
        if hwnd.is_null() || hwnd.is_invalid() {
            return None;
        }
        self.windows.get_mut(hwnd)
    }
}

/*
pub mod MessageBoxFlags {
    pub const ABORTRETRYIGNORE: u32 = 0x00000002;
    pub const CANCELTRYCONTINUE: u32 = 0x00000006;
    pub const HELP: u32 = 0x00004000;
    pub const OK: u32 = 0x00000000;
    pub const OKCANCEL: u32 = 0x00000001;
    pub const RETRYCANCEL: u32 = 0x00000005;
    pub const YESNO: u32 = 0x00000004;
    pub const YESNOCANCEL: u32 = 0x00000003;

    pub const ICONEXCLAMATION: u32 = 0x00000030;
    pub const ICONWARNING: u32 = 0x00000030;
    pub const ICONINFORMATION: u32 = 0x00000040;
    pub const ICONASTERISK: u32 = 0x00000040;
    pub const ICONQUESTION: u32 = 0x00000020;
    pub const ICONSTOP: u32 = 0x00000010;
    pub const ICONERROR: u32 = 0x00000010;
    pub const ICONHAND: u32 = 0x00000010;

    pub const DEFBUTTON1: u32 = 0x00000000;
    pub const DEFBUTTON2: u32 = 0x00000100;
    pub const DEFBUTTON3: u32 = 0x00000200;
    pub const DEFBUTTON4: u32 = 0x00000300;

    pub const APPLMODAL: u32 = 0x00000000;
    pub const SYSTEMMODAL: u32 = 0x00001000;
    pub const TASKMODAL: u32 = 0x00002000;
    pub const DEFAULT_DESKTOP_ONLY: u32 = 0x00020000;
    pub const RIGHT: u32 = 0x00080000;
    pub const RTLREADING: u32 = 0x00100000;
    pub const SETFOREGROUND: u32 = 0x00010000;
    pub const TOPMOST: u32 = 0x00040000;
    pub const SERVICE_NOTIFICATION: u32 = 0x00200000;
}
*/

#[win32_derive::dllexport]
pub fn MessageBoxA(
    machine: &mut Machine,
    hWnd: HWND,
    lpText: Option<&str>,
    lpCaption: Option<&str>,
    uType: u32,
) -> u32 {
    machine.host.write(
        format!(
            "MessageBox: {}\n{}",
            lpCaption.unwrap_or("Error"),
            lpText.unwrap_or("")
        )
        .as_bytes(),
    );
    1 // IDOK
}

#[win32_derive::dllexport]
pub fn MessageBoxW(
    machine: &mut Machine,
    hWnd: HWND,
    lpText: Option<&Str16>,
    lpCaption: Option<&Str16>,
    uType: u32,
) -> u32 {
    machine.host.write(
        format!(
            "MessageBox: {}\n{}",
            lpCaption.unwrap().to_string(),
            lpText.unwrap().to_string()
        )
        .as_bytes(),
    );
    1 // IDOK
}

#[win32_derive::dllexport]
pub fn DialogBoxParamA(
    _machine: &mut Machine,
    hInstance: u32,
    lpTemplateName: u32,
    hWndParent: HWND,
    lpDialogFunc: u32,
    dwInitParam: u32,
) -> u32 {
    log::warn!("TODO: DialogBoxParamA({hInstance:x}, {lpTemplateName:x}, {hWndParent:x}, {lpDialogFunc:x}, {dwInitParam:x})");
    // TODO: this should run a nested message loop that will call back into lpDialogFunc,
    // which then will call EndDialog to end the nested message loop and return the value
    // passed to EndDialog.
    // Unfortunately we don't know what value to return here otherwise; it's application specific.
    0
}

#[derive(Debug, win32_derive::TryFromEnum)]
pub enum SystemMetric {
    CXSCREEN = 0,
    CYSCREEN = 1,
    CYCAPTION = 4,
    CXBORDER = 5,
    CYBORDER = 6,
    CYMENU = 15,
    CXFRAME = 32,
    CYFRAME = 33,
    CXVIRTUALSCREEN = 78,
    CYVIRTUALSCREEN = 79,
}

#[win32_derive::dllexport]
pub fn GetSystemMetrics(_machine: &mut Machine, nIndex: Result<SystemMetric, u32>) -> u32 {
    let metric = match nIndex {
        Ok(metric) => metric,
        Err(val) => {
            log::error!("GetSystemMetrics({val}) => 0");
            return 0;
        }
    };
    match metric {
        SystemMetric::CXSCREEN => 640,
        SystemMetric::CYSCREEN => 480,
        SystemMetric::CYCAPTION => 19,
        SystemMetric::CXBORDER => 1,
        SystemMetric::CYBORDER => 1,
        SystemMetric::CYMENU => 19,
        SystemMetric::CXFRAME => 4,
        SystemMetric::CYFRAME => 4,
        SystemMetric::CXVIRTUALSCREEN => 640,
        SystemMetric::CYVIRTUALSCREEN => 480,
    }
}

#[win32_derive::dllexport]
pub fn SetTimer(
    machine: &mut Machine,
    hWnd: HWND,
    nIDEvent: u32,
    uElapse: u32,
    lpTimerFunc: u32,
) -> u32 {
    const USER_TIMER_MINIMUM: u32 = 0x0000_000A;
    const USER_TIMER_MAXIMUM: u32 = 0x7FFF_FFFF;
    let _uElapse = num_traits::clamp(uElapse, USER_TIMER_MINIMUM, USER_TIMER_MAXIMUM);
    if lpTimerFunc != 0 {
        todo!("SetTimer with callback");
    }

    log::warn!("registering timer that will never fire");
    let timer = machine.state.user32.next_timer;
    machine.state.user32.next_timer += 1;
    timer
}

#[win32_derive::dllexport]
pub fn SetRect(
    _machine: &mut Machine,
    lprc: Option<&mut RECT>,
    xLeft: i32,
    yTop: i32,
    xRight: i32,
    yBottom: i32,
) -> bool {
    let rect = lprc.unwrap();
    *rect = RECT {
        left: xLeft,
        top: yTop,
        right: xRight,
        bottom: yBottom,
    };
    true
}

#[win32_derive::dllexport]
pub fn CheckMenuItem(_machine: &mut Machine, hMenu: HMENU, uIDCheckItem: u32, uCheck: u32) -> u32 {
    0 // previous state: unchecked
}

#[win32_derive::dllexport]
pub fn SetMenu(_machine: &mut Machine, hWnd: HWND, hMenu: HMENU) -> bool {
    true // success
}

#[derive(Debug)]
#[repr(C)]
pub struct PAINTSTRUCT {
    hdc: HDC,
    fErase: u32,
    rcPaint: RECT,
    fRestore: u32,
    fIncUpdate: u32,
    rgbReserved: [u8; 32],
}
unsafe impl memory::Pod for PAINTSTRUCT {}

#[win32_derive::dllexport]
pub fn BeginPaint(machine: &mut Machine, hWnd: HWND, lpPaint: Option<&mut PAINTSTRUCT>) -> HDC {
    let window = machine.state.user32.windows.get_mut(hWnd).unwrap();
    if let Some(hbrush) = window.wndclass.background.to_option() {
        if let super::gdi32::Object::Brush(brush) = machine.state.gdi32.objects.get(hbrush).unwrap()
        {
            window
                .pixels_mut(&mut *machine.host)
                .raw
                .fill(brush.color.to_pixel());
        }
    }
    let hdc = window.hdc;
    *lpPaint.unwrap() = PAINTSTRUCT {
        hdc: hdc,
        fErase: 1, // todo
        rcPaint: RECT {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        },
        fRestore: 0,          // reserved
        fIncUpdate: 0,        // reserved
        rgbReserved: [0; 32], // reserved
    };
    hdc
}

#[win32_derive::dllexport]
pub fn EndPaint(machine: &mut Machine, hWnd: HWND, lpPaint: Option<&PAINTSTRUCT>) -> bool {
    let window = machine.state.user32.windows.get_mut(hWnd).unwrap();
    window.flush_pixels();
    machine
        .state
        .user32
        .windows
        .get_mut(hWnd)
        .unwrap()
        .need_paint = false;
    true
}

#[win32_derive::dllexport]
pub fn PtInRect(_machine: &mut Machine, lprc: Option<&RECT>, pt: POINT) -> bool {
    let rect = lprc.unwrap();
    let (x, y) = (pt.x as i32, pt.y as i32);
    x >= rect.left && x < rect.right && y >= rect.top && y < rect.bottom
}

#[win32_derive::dllexport]
pub fn SetCapture(_machine: &mut Machine, hwnd: HWND) -> HWND {
    HWND::null()
}

#[win32_derive::dllexport]
pub fn ReleaseCapture(_machine: &mut Machine) -> bool {
    true
}
