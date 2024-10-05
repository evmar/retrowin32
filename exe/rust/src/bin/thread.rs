//! Run two threads printing in parallel.
//! Purpose: exercise CreateThread and threading in general.

#![windows_subsystem = "console"]

use windows_sys::Win32::{
    Storage::FileSystem::WriteFile,
    System::{
        Console::{GetStdHandle, STD_OUTPUT_HANDLE},
        Threading::{CreateThread, GetCurrentThreadId, Sleep, TlsAlloc, TlsGetValue, TlsSetValue},
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

struct ThreadParams {
    name: &'static str,
    steps: u32,
    tls_key: u32,
}

fn run_thread(params: &ThreadParams) {
    for i in 0..params.steps {
        let thread_id = unsafe { GetCurrentThreadId() };
        let tls = unsafe { TlsGetValue(params.tls_key) as u32 };
        print(format!(
            "thread_id={thread_id} name={name:?} tls={tls} i={i}\n",
            name = params.name
        ));
        unsafe { Sleep(1000 / params.steps) };
    }
    let thread_id = unsafe { GetCurrentThreadId() };
    let tls = unsafe { TlsGetValue(params.tls_key) as u32 };
    print(format!(
        "thread_id={thread_id} name={name:?} tls={tls} returning\n",
        name = params.name,
    ));
}

unsafe extern "system" fn thread_proc(param: *mut std::ffi::c_void) -> u32 {
    let params = &*(param as *const ThreadParams);
    TlsSetValue(params.tls_key, 2 as *const _);
    run_thread(params);
    0
}

unsafe extern "system" fn short_thread_proc(_param: *mut std::ffi::c_void) -> u32 {
    let thread_id = GetCurrentThreadId();
    print(format!("thread_id={thread_id} short_thread_proc exiting\n"));
    0
}

fn main() {
    unsafe {
        // Create a thread that starts and exits quickly, to exercise thread teardown.
        CreateThread(
            std::ptr::null(),
            0x1000,
            Some(short_thread_proc),
            std::ptr::null(),
            0,
            std::ptr::null_mut(),
        );

        let tls_key = TlsAlloc();
        TlsSetValue(tls_key, 1 as *const _);
        let thread_params = ThreadParams {
            name: "i_am_thread",
            steps: 5,
            tls_key,
        };

        CreateThread(
            std::ptr::null(),
            0x1000,
            Some(thread_proc),
            &thread_params as *const _ as *const _,
            0,
            std::ptr::null_mut(),
        );

        run_thread(&ThreadParams {
            name: "main",
            steps: 10,
            tls_key,
        });
    }
}
