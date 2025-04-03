//! Create a DirectDraw window and surface.

#![allow(non_snake_case)]
#![no_std]
#![no_main]
#![windows_subsystem = "windows"]

use ddraw_sys::*;
use exe::{println, windows_message_to_str};
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
    let mut msg: MSG = core::mem::zeroed();
    while GetMessageA(&mut msg, 0, 0, 0) > 0 {
        TranslateMessage(&msg);
        DispatchMessageA(&msg);
    }
}

#[no_mangle]
pub unsafe extern "C" fn mainCRTStartup() {
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
        lpszMenuName: core::ptr::null(),
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
        core::ptr::null(),
    );
    if hwnd == 0 {
        panic!("create failed");
    }

    println!("DirectDrawCreate");
    let mut ddraw: *mut IDirectDraw = core::ptr::null_mut();
    DirectDrawCreate(core::ptr::null_mut(), &mut ddraw, core::ptr::null_mut());

    println!("SetCooperativeLevel");
    (*(*ddraw).lpVtbl).SetCooperativeLevel.unwrap()(ddraw, hwnd as *mut _, 0x11); // normal | exclusive
    println!("SetDisplayMode");
    (*(*ddraw).lpVtbl).SetDisplayMode.unwrap()(ddraw, 320, 200, 32);

    let mut desc: DDSURFACEDESC = core::mem::zeroed();
    desc.dwSize = core::mem::size_of::<DDSURFACEDESC>() as _;
    desc.dwFlags = 0x21; // backbuffer | caps
    desc.dwBackBufferCount = 2;
    desc.ddsCaps.dwCaps = 0x4218; // primarysurface | videomemory | primarysurface | complex

    let mut surf: *mut IDirectDrawSurface = core::ptr::null_mut();
    (*(*ddraw).lpVtbl).CreateSurface.unwrap()(ddraw, &mut desc, &mut surf, core::ptr::null_mut());

    mainloop();
}
