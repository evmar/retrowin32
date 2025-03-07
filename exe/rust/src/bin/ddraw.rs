#![allow(non_snake_case)]

use ddraw_sys::*;
use exe::windows_message_to_str;
use windows_sys::Win32::{
    Foundation::{HWND, LRESULT},
    UI::WindowsAndMessaging::{
        CreateWindowExA, DefWindowProcA, DispatchMessageA, GetMessageA, RegisterClassA,
        TranslateMessage, CW_USEDEFAULT, MSG, WNDCLASSA, WS_OVERLAPPED,
    },
};

unsafe extern "system" fn wndproc(hwnd: HWND, msg: u32, wparam: usize, lparam: isize) -> LRESULT {
    println!(
        "msg: {:x} {:?} {{",
        msg,
        windows_message_to_str(msg).unwrap()
    );
    let ret = DefWindowProcA(hwnd, msg, wparam, lparam);
    println!(
        "}} msg: {:x} {:?}",
        msg,
        windows_message_to_str(msg).unwrap()
    );
    ret
}

unsafe fn mainloop() {
    let mut msg: MSG = std::mem::zeroed();
    while GetMessageA(&mut msg, 0, 0, 0) > 0 {
        TranslateMessage(&msg);
        DispatchMessageA(&msg);
    }
}

unsafe fn unsafe_main() {
    let class_name = b"ddraw\0";
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

    println!("CreateWindowExA");
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

    println!("DirectDrawCreate");
    let mut ddraw: *mut IDirectDraw = std::ptr::null_mut();
    DirectDrawCreate(std::ptr::null_mut(), &mut ddraw, std::ptr::null_mut());

    println!("SetCooperativeLevel");
    (*(*ddraw).lpVtbl).SetCooperativeLevel.unwrap()(ddraw, hwnd as *mut _, 0x11); // normal | exclusive
    println!("SetDisplayMode");
    (*(*ddraw).lpVtbl).SetDisplayMode.unwrap()(ddraw, 320, 200, 32);

    let mut desc: DDSURFACEDESC = std::mem::zeroed();
    desc.dwSize = std::mem::size_of::<DDSURFACEDESC>() as u32;
    desc.dwFlags = 0x21; // backbuffer | caps
    desc.dwBackBufferCount = 2;
    desc.ddsCaps.dwCaps = 0x4218; // primarysurface | videomemory | primarysurface | complex

    let mut surf: *mut IDirectDrawSurface = std::ptr::null_mut();
    (*(*ddraw).lpVtbl).CreateSurface.unwrap()(ddraw, &mut desc, &mut surf, std::ptr::null_mut());

    mainloop();
}

fn main() {
    unsafe {
        unsafe_main();
    }
}
