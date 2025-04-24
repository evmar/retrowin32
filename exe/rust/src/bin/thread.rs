//! Run two threads printing in parallel.
//! Purpose: exercise CreateThread and threading in general.

#![no_std]
#![no_main]
#![windows_subsystem = "console"]

use exe::println;
use windows_sys::Win32::System::Threading::{
    CreateThread, GetCurrentThreadId, Sleep, TlsAlloc, TlsGetValue, TlsSetValue,
};

struct ThreadParams {
    name: &'static str,
    steps: u32,
    tls_key: u32,
}

fn run_thread(params: &ThreadParams) {
    for i in 0..params.steps {
        let thread_id = unsafe { GetCurrentThreadId() };
        let tls = unsafe { TlsGetValue(params.tls_key) as u32 };
        println!(
            "thread_id={thread_id} name={name:?} tls={tls} i={i}",
            name = params.name
        );
        unsafe { Sleep(1000 / params.steps) };
    }
    let thread_id = unsafe { GetCurrentThreadId() };
    let tls = unsafe { TlsGetValue(params.tls_key) as u32 };
    println!(
        "thread_id={thread_id} name={name:?} tls={tls} returning",
        name = params.name,
    );
}

extern "system" fn thread_proc(param: *mut core::ffi::c_void) -> u32 {
    unsafe {
        let params = &*(param as *const ThreadParams);
        TlsSetValue(params.tls_key, 2 as *const _);
        run_thread(params);
        0
    }
}

extern "system" fn short_thread_proc(_param: *mut core::ffi::c_void) -> u32 {
    let thread_id = unsafe { GetCurrentThreadId() };
    println!("thread_id={thread_id} short_thread_proc exiting\n");
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn mainCRTStartup() {
    unsafe {
        // Create a thread that starts and exits quickly, to exercise thread teardown.
        CreateThread(
            core::ptr::null(),
            0x1000,
            Some(short_thread_proc),
            core::ptr::null(),
            0,
            core::ptr::null_mut(),
        );

        let tls_key = TlsAlloc();
        TlsSetValue(tls_key, 1 as *const _);
        let thread_params = ThreadParams {
            name: "i_am_thread",
            steps: 5,
            tls_key,
        };

        CreateThread(
            core::ptr::null(),
            0x1000,
            Some(thread_proc),
            &thread_params as *const _ as *const _,
            0,
            core::ptr::null_mut(),
        );

        run_thread(&ThreadParams {
            name: "main",
            steps: 10,
            tls_key,
        });
    }
}
