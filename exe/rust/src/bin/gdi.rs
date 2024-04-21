//! Create a window and paint on it.
//! Purpose: exercise some GDI basics like CreateWindow and message loop.

#![windows_subsystem = "windows"]

use windows_sys::Win32::{
    Foundation::{HWND, LRESULT},
    Graphics::Gdi::{BeginPaint, EndPaint, FillRect, COLOR_WINDOW, HBRUSH, PAINTSTRUCT},
    UI::WindowsAndMessaging::{
        CreateWindowExA, DefWindowProcA, DispatchMessageA, GetMessageA, PostQuitMessage,
        RegisterClassA, ShowWindow, TranslateMessage, CW_USEDEFAULT, MSG, SW_NORMAL, WM_DESTROY,
        WM_PAINT, WNDCLASSA, WS_OVERLAPPED,
    },
};

unsafe extern "system" fn wndproc(hwnd: HWND, msg: u32, wparam: usize, lparam: isize) -> LRESULT {
    match msg {
        WM_DESTROY => {
            PostQuitMessage(0);
            0
        }
        WM_PAINT => {
            let mut ps: PAINTSTRUCT = std::mem::zeroed();
            let hdc = BeginPaint(hwnd, &mut ps);
            FillRect(hdc, &ps.rcPaint, (COLOR_WINDOW + 1) as HBRUSH);
            EndPaint(hwnd, &ps);
            0
        }
        _ => DefWindowProcA(hwnd, msg, wparam, lparam),
    }
}

unsafe fn create_window() -> HWND {
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
        lpszMenuName: std::ptr::null(),
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
        std::ptr::null(),
    );
    if hwnd == 0 {
        panic!("create failed");
    }
    ShowWindow(hwnd, SW_NORMAL);
    hwnd
}

fn main() {
    unsafe {
        create_window();
        let mut msg: MSG = std::mem::zeroed();
        while GetMessageA(&mut msg, 0, 0, 0) > 0 {
            TranslateMessage(&msg);
            DispatchMessageA(&msg);
        }
    }
}
