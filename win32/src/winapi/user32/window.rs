use super::*;
use crate::{host, winapi::gdi32::HDC};
use bitflags::bitflags;
use std::rc::Rc;

pub struct Window {
    pub host: Box<dyn host::Window>,
    pub wndclass: Rc<WndClass>,
}

pub struct WndClass {
    pub name: String,
    pub wndproc: u32,
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct WNDCLASSA {
    style: u32,
    lpfnWndProc: u32,
    cbClsExtra: u32,
    cbWndExtra: u32,
    hInstance: HINSTANCE,
    hIcon: HICON,
    hCursor: HCURSOR,
    hbrBackground: HBRUSH,
    lpszMenuName: u32,
    lpszClassName: u32,
}
unsafe impl memory::Pod for WNDCLASSA {}

#[win32_derive::dllexport]
pub fn RegisterClassA(machine: &mut Machine, lpWndClass: Option<&WNDCLASSA>) -> u32 {
    let wndclass = lpWndClass.unwrap();
    let ex = WNDCLASSEXA {
        cbSize: std::mem::size_of::<WNDCLASSEXA>() as u32,
        style: wndclass.style,
        lpfnWndProc: wndclass.lpfnWndProc,
        cbClsExtra: wndclass.cbClsExtra,
        cbWndExtra: wndclass.cbWndExtra,
        hInstance: wndclass.hInstance,
        hIcon: wndclass.hIcon,
        hCursor: wndclass.hCursor,
        hbrBackground: wndclass.hbrBackground,
        lpszMenuName: wndclass.lpszMenuName,
        lpszClassName: wndclass.lpszClassName,
        hIconSm: 0,
    };
    RegisterClassExA(machine, Some(&ex))
}

#[repr(C, packed)]
#[derive(Clone, Debug)]
pub struct WNDCLASSEXA {
    cbSize: u32,
    style: u32,
    lpfnWndProc: u32,
    cbClsExtra: u32,
    cbWndExtra: u32,
    hInstance: HINSTANCE,
    hIcon: HICON,
    hCursor: HCURSOR,
    hbrBackground: HBRUSH,
    lpszMenuName: u32,
    lpszClassName: u32,
    hIconSm: HICON,
}
unsafe impl memory::Pod for WNDCLASSEXA {}

#[win32_derive::dllexport]
pub fn RegisterClassExA(machine: &mut Machine, lpWndClassEx: Option<&WNDCLASSEXA>) -> u32 {
    let lpWndClassEx = lpWndClassEx.unwrap();
    let atom = machine.state.user32.wndclasses.len() as u32 + 1;
    let name = machine
        .mem()
        .slicez(lpWndClassEx.lpszClassName)
        .unwrap()
        .to_ascii()
        .to_string();
    let wndclass = WndClass {
        // atom,
        name,
        wndproc: lpWndClassEx.lpfnWndProc,
    };
    machine.state.user32.wndclasses.push(Rc::new(wndclass));
    atom
}

bitflags! {
    pub struct WindowStyle: u32 {
        const WS_POPUP           = 0x80000000;
        const WS_CHILD           = 0x40000000;
        const WS_MINIMIZE        = 0x20000000;
        const WS_VISIBLE         = 0x10000000;
        const WS_DISABLED        = 0x08000000;
        const WS_CLIPSIBLINGS    = 0x04000000;
        const WS_CLIPCHILDREN    = 0x02000000;
        const WS_MAXIMIZE        = 0x01000000;
        const WS_BORDER          = 0x00800000;
        const WS_DLGFRAME        = 0x00400000;
        const WS_VSCROLL         = 0x00200000;
        const WS_HSCROLL         = 0x00100000;
        const WS_SYSMENU         = 0x00080000;
        const WS_THICKFRAME      = 0x00040000;
        const WS_GROUP           = 0x00020000;
        const WS_TABSTOP         = 0x00010000;
    }
}
impl TryFrom<u32> for WindowStyle {
    type Error = u32;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        WindowStyle::from_bits(value).ok_or(value)
    }
}

#[win32_derive::dllexport]
pub async fn CreateWindowExA(
    machine: &mut Machine,
    dwExStyle: u32,
    lpClassName: u32,
    lpWindowName: Option<&str>,
    dwStyle: Result<WindowStyle, u32>,
    X: u32,
    Y: u32,
    nWidth: u32,
    nHeight: u32,
    hWndParent: HWND,
    hMenu: u32,
    hInstance: u32,
    lpParam: u32,
) -> HWND {
    if lpClassName < 1 << 16 {
        todo!("numeric wndclass reference");
    }
    let mem = machine.mem();
    let class_name = mem.slicez(lpClassName).unwrap().to_ascii();
    let wndclass = machine
        .state
        .user32
        .wndclasses
        .iter()
        .find(|c| c.name == class_name)
        .unwrap()
        .clone();

    let _style = dwStyle.unwrap();
    const CW_USEDEFAULT: u32 = 0x8000_0000;

    // hInstance is only relevant when multiple DLLs register classes:
    //   https://devblogs.microsoft.com/oldnewthing/20050418-59/?p=35873

    let mut host_win = machine.host.create_window();
    host_win.set_title(lpWindowName.unwrap());
    if nWidth > 0 && nHeight > 0 {
        let width = if nWidth == CW_USEDEFAULT { 640 } else { nWidth };
        let height = if nHeight == CW_USEDEFAULT {
            480
        } else {
            nHeight
        };
        host_win.set_size(width, height);
    }

    let window = Window {
        host: host_win,
        wndclass,
    };
    machine.state.user32.windows.push(window);
    let hwnd = HWND::from_raw(machine.state.user32.windows.len() as u32);

    // Synchronously dispatch WM_CREATE.
    let msg = MSG {
        hwnd,
        message: WM::CREATE,
        wParam: 0,
        lParam: 0, // TODO: CREATESTRUCT
        time: 0,
        pt_x: 0,
        pt_y: 0,
        lPrivate: 0,
    };
    DispatchMessageA(machine, Some(&msg)).await;

    // Enqueue WM_ACTIVATEAPP message.
    machine.state.user32.messages.push_back(MSG {
        hwnd,
        message: WM::ACTIVATEAPP,
        wParam: true as u32, // activating
        lParam: 0,           // TODO: thread id
        time: 0,             // TODO
        pt_x: 0,             // TODO
        pt_y: 0,             // TODO
        lPrivate: 0,
    });

    hwnd
}

#[win32_derive::dllexport]
pub fn DestroyWindow(_machine: &mut Machine, hWnd: HWND) -> bool {
    true // success
}

#[win32_derive::dllexport]
pub fn GetForegroundWindow(machine: &mut Machine) -> HWND {
    HWND::from_raw(machine.state.user32.windows.len() as u32)
}

#[win32_derive::dllexport]
pub fn SetForegroundWindow(_machine: &mut Machine, hWnd: HWND) -> bool {
    true // success
}

#[win32_derive::dllexport]
pub fn GetActiveWindow(machine: &mut Machine) -> HWND {
    HWND::from_raw(machine.state.user32.windows.len() as u32)
}

#[win32_derive::dllexport]
pub fn GetLastActivePopup(machine: &mut Machine) -> HWND {
    HWND::from_raw(machine.state.user32.windows.len() as u32)
}

#[win32_derive::dllexport]
pub fn FindWindowA(
    machine: &mut Machine,
    lpClassName: Option<&str>,
    lpWindowName: Option<&str>,
) -> HWND {
    HWND::from_raw(machine.state.user32.windows.len() as u32)
}

#[win32_derive::dllexport]
pub fn UpdateWindow(_machine: &mut Machine, hWnd: HWND) -> bool {
    // TODO: this should cause a synchronous WM_PAINT.
    true // success
}

/// nCmdShow passed to ShowWindow().
#[derive(Debug)]
pub enum SW {
    HIDE = 0,
    NORMAL = 1,
    SHOWMINIMIZED = 2,
    SHOWMAXIMIZED = 3,
    SHOWNOACTIVATE = 4,
    SHOW = 5,
    MINIMIZE = 6,
    SHOWMINNOACTIVE = 7,
    SHOWNA = 8,
    RESTORE = 9,
    SHOWDEFAULT = 10,
    FORCEMINIMIZE = 11,
}
impl TryFrom<u32> for SW {
    type Error = u32;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => SW::HIDE,
            1 => SW::NORMAL,
            2 => SW::SHOWMINIMIZED,
            3 => SW::SHOWMAXIMIZED,
            4 => SW::SHOWNOACTIVATE,
            5 => SW::SHOW,
            6 => SW::MINIMIZE,
            7 => SW::SHOWMINNOACTIVE,
            8 => SW::SHOWNA,
            9 => SW::RESTORE,
            10 => SW::SHOWDEFAULT,
            11 => SW::FORCEMINIMIZE,
            _ => return Err(value),
        })
    }
}

#[win32_derive::dllexport]
pub fn ShowWindow(_machine: &mut Machine, hWnd: HWND, nCmdShow: Result<SW, u32>) -> bool {
    let previously_visible = true;
    previously_visible
}

#[win32_derive::dllexport]
pub fn SetFocus(_machine: &mut Machine, hWnd: HWND) -> HWND {
    let prev_focused = HWND::null();
    prev_focused
}

#[win32_derive::dllexport]
pub fn DefWindowProcA(
    _machine: &mut Machine,
    hWnd: HWND,
    msg: Result<WM, u32>,
    wParam: u32,
    lParam: u32,
) -> u32 {
    0
}

#[win32_derive::dllexport]
pub fn GetClientRect(_machine: &mut Machine, hWnd: HWND, lpRect: Option<&mut RECT>) -> bool {
    let rect = lpRect.unwrap();
    *rect = RECT {
        left: 0,
        top: 0,
        right: 640, // TODO: use actual window size
        bottom: 480,
    };
    true
}

#[win32_derive::dllexport]
pub fn GetWindowDC(_machine: &mut Machine, hWnd: HWND) -> HDC {
    HDC::null()
}

#[win32_derive::dllexport]
pub fn ReleaseDC(_machine: &mut Machine, hdc: HDC) -> bool {
    // Note: there is also DeleteDC; this one is specific for some specific DC types...
    log::warn!("todo: ReleaseDC({hdc:x})");
    false // fail
}
