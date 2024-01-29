#![no_std]
#![no_main]
#![windows_subsystem = "windows"]

use core::ptr;
use windows_sys::Win32::{
    Foundation::{HWND, LRESULT},
    Graphics::Gdi::{BeginPaint, EndPaint, FillRect, COLOR_WINDOW, HBRUSH, PAINTSTRUCT},
    Storage::FileSystem::WriteFile,
    System::Console::{GetStdHandle, STD_OUTPUT_HANDLE},
    UI::WindowsAndMessaging::{
        CreateWindowExA, DefWindowProcA, DispatchMessageA, GetMessageA, PostQuitMessage,
        RegisterClassA, ShowWindow, TranslateMessage, CW_USEDEFAULT, MSG, SW_NORMAL, WM_DESTROY,
        WM_PAINT, WNDCLASSA, WS_OVERLAPPED,
    },
};

fn print(buf: &[u8]) {
    unsafe {
        let stdout = GetStdHandle(STD_OUTPUT_HANDLE);
        WriteFile(
            stdout,
            buf.as_ptr(),
            buf.len() as u32,
            core::ptr::null_mut(),
            core::ptr::null_mut(),
        );
    }
}

// rust-analyzer gets confused about this function, so we hide it from rust-analyzer
// following https://github.com/phil-opp/blog_os/issues/1022
#[cfg(not(test))]
#[panic_handler]
unsafe fn handle_panic(_: &core::panic::PanicInfo) -> ! {
    print(b"panicked");
    windows_sys::Win32::System::Threading::ExitProcess(1);
}

unsafe extern "system" fn wndproc(hwnd: HWND, msg: u32, wparam: usize, lparam: isize) -> LRESULT {
    match msg {
        WM_DESTROY => {
            PostQuitMessage(0);
            0
        }
        WM_PAINT => {
            let mut ps: PAINTSTRUCT = unsafe { core::mem::zeroed() };
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
        lpszMenuName: ptr::null(),
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
        ptr::null(),
    );
    if hwnd == 0 {
        panic!("create failed");
    }
    ShowWindow(hwnd, SW_NORMAL);
    hwnd
}

#[no_mangle]
pub unsafe extern "C" fn mainCRTStartup() {
    create_window();
    let mut msg: MSG = unsafe { core::mem::zeroed() };
    while GetMessageA(&mut msg, 0, 0, 0) > 0 {
        TranslateMessage(&msg);
        DispatchMessageA(&msg);
    }
}
