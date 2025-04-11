//! Create a window and paint on it.
//! Purpose: exercise some GDI basics like CreateWindow and message loop.

#![no_std]
#![no_main]
#![windows_subsystem = "windows"]

use exe as _; // panic handler
use windows_sys::Win32::{
    Foundation::{HWND, LRESULT},
    Graphics::Gdi::*,
    UI::WindowsAndMessaging::*,
};

const ID_QUITBUTTON: usize = 1;

unsafe extern "system" fn wndproc(hwnd: HWND, msg: u32, wparam: usize, lparam: isize) -> LRESULT {
    match msg {
        WM_DESTROY => {
            PostQuitMessage(0);
            return 0;
        }
        WM_PAINT => {
            let mut ps: PAINTSTRUCT = core::mem::zeroed();
            let hdc = BeginPaint(hwnd, &mut ps);
            FillRect(hdc, &ps.rcPaint, (COLOR_WINDOW + 1) as HBRUSH);
            EndPaint(hwnd, &ps);
            return 0;
        }
        WM_COMMAND => match wparam {
            ID_QUITBUTTON => {
                PostQuitMessage(0);
                return 0;
            }
            _ => {}
        },
        _ => {}
    }
    DefWindowProcA(hwnd, msg, wparam, lparam)
}

unsafe fn get_default_font() -> HFONT {
    let mut metrics: NONCLIENTMETRICSA = core::mem::zeroed();
    metrics.cbSize = core::mem::size_of::<NONCLIENTMETRICSA>() as u32;
    if SystemParametersInfoA(
        SPI_GETNONCLIENTMETRICS,
        metrics.cbSize,
        &mut metrics as *mut _ as *mut _,
        0,
    ) == 0
    {
        panic!("SystemParametersInfoA failed");
    }

    let hfont = CreateFontIndirectA(&metrics.lfMessageFont);
    if hfont == 0 {
        panic!("CreateFontIndirectA failed");
    }

    hfont
}

unsafe fn create_window() -> HWND {
    let hfont = get_default_font();

    let class_name = b"gdi\0";
    let wc = WNDCLASSA {
        style: 0,
        lpfnWndProc: Some(wndproc),
        cbClsExtra: 0,
        cbWndExtra: 0,
        hInstance: 0,
        hIcon: 0,
        hCursor: 0,
        hbrBackground: 0,
        lpszMenuName: core::ptr::null(),
        lpszClassName: class_name.as_ptr(),
    };
    RegisterClassA(&wc);

    let hwnd = CreateWindowExA(
        0,
        class_name.as_ptr(),
        b"title\0".as_ptr(),
        WS_OVERLAPPED,
        CW_USEDEFAULT,
        CW_USEDEFAULT,
        400,
        300,
        0,
        0,
        0,
        core::ptr::null(),
    );
    if hwnd == 0 {
        panic!("create failed");
    }

    let button = CreateWindowExA(
        0,
        b"BUTTON\0".as_ptr(),
        b"quit\0".as_ptr(),
        WS_TABSTOP | WS_VISIBLE | WS_CHILD | BS_DEFPUSHBUTTON as u32,
        10,
        10,
        100,
        30,
        hwnd,
        ID_QUITBUTTON as _,
        0,
        core::ptr::null(),
    );
    if button == 0 {
        panic!("button create failed");
    }

    SendMessageA(button, WM_SETFONT, hfont as usize, /* redraw */ 0);

    ShowWindow(hwnd, SW_NORMAL);
    hwnd
}

#[no_mangle]
pub unsafe extern "C" fn mainCRTStartup() {
    create_window();
    let mut msg: MSG = core::mem::zeroed();
    while GetMessageA(&mut msg, 0, 0, 0) > 0 {
        TranslateMessage(&msg);
        DispatchMessageA(&msg);
    }
}
