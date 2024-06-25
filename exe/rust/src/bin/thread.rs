//! Run two threads printing in parallel.
//! Purpose: exercise CreateThread and threading in general.

#![windows_subsystem = "console"]

use windows_sys::Win32::{
    Storage::FileSystem::WriteFile,
    System::{
        Console::{GetStdHandle, STD_OUTPUT_HANDLE},
        Threading::{CreateThread, GetCurrentThreadId, Sleep},
    },
};

// TODO: can't use println!() yet because Rust wants to grab locks on stdout,
// and I haven't implemented lock APIs yet.
fn print(msg: String) {
    unsafe {
        let handle = GetStdHandle(STD_OUTPUT_HANDLE);
        WriteFile(
            handle,
            msg.as_ptr(),
            msg.len() as u32,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        );
    }
}

unsafe extern "system" fn thread_proc(param: *mut std::ffi::c_void) -> u32 {
    let param = if param.is_null() {
        None
    } else {
        Some(*(param as *const &str))
    };
    let fast = param.is_some();
    for i in 0..(if fast { 10 } else { 5 }) {
        let thread_id = GetCurrentThreadId();
        print(format!("thread={thread_id} param={param:?} i={i}\n"));
        Sleep(if fast { 100 } else { 200 });
    }
    let thread_id = GetCurrentThreadId();
    print(format!("thread={thread_id} param={param:?} returning\n"));
    0
}

fn main() {
    unsafe {
        CreateThread(
            std::ptr::null(),
            0x1000,
            Some(thread_proc),
            &"i_am_thread" as *const _ as *const _,
            0,
            std::ptr::null_mut(),
        );
        thread_proc(std::ptr::null_mut());
    }
}
