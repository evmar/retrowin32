//! Exercise command line functions.

#![no_std]
#![no_main]
#![windows_subsystem = "console"]

use core::ffi::CStr;
use exe::println;
use windows_sys::Win32::System::{Environment::GetCommandLineA, LibraryLoader::GetModuleFileNameA};

#[no_mangle]
pub unsafe extern "C" fn mainCRTStartup() {
    let mut buf: [u8; 256] = [0; 256];

    let ret = GetModuleFileNameA(0, buf.as_mut_ptr(), buf.len() as u32);
    let filename = CStr::from_bytes_until_nul(&buf).unwrap();
    println!("GetModuleFileNameA: {ret} {:?}", filename.to_str().unwrap());

    let cmdline = CStr::from_ptr(GetCommandLineA() as *const _);
    println!("GetCommandLineA: {:?}", cmdline.to_str().unwrap());
}
