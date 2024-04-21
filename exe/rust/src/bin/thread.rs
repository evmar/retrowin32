//! Run two threads printing in parallel.
//! Purpose: exercise CreateThread and threading in general.

#![windows_subsystem = "console"]

use windows_sys::Win32::System::Threading::{CreateThread, GetCurrentThreadId, Sleep};

unsafe extern "system" fn thread_proc(param: *mut std::ffi::c_void) -> u32 {
    let param = param as u32;
    for i in 0..3 {
        let thread_id = GetCurrentThreadId();
        println!("thread={thread_id} param={param} i={i}");
        Sleep(100);
    }
    0
}

fn main() {
    unsafe {
        CreateThread(
            std::ptr::null(),
            0x1000,
            Some(thread_proc),
            std::ptr::null(),
            0,
            std::ptr::null_mut(),
        );
        thread_proc(std::ptr::null_mut());
    }
}
