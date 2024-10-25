//! Exercise command line functions.

#![windows_subsystem = "console"]

use std::ffi::CStr;
use windows_sys::Win32::System::{Environment::GetCommandLineA, LibraryLoader::GetModuleFileNameA};

fn main() {
    unsafe {
        let mut buf: [u8; 256] = [0; 256];

        let ret = GetModuleFileNameA(0, buf.as_mut_ptr(), buf.len() as u32);
        let filename = CStr::from_bytes_until_nul(&buf).unwrap();
        println!("GetModuleFileNameA: {ret} {:?}", filename.to_string_lossy());

        let cmdline = CStr::from_ptr(GetCommandLineA() as *const _);
        println!("GetCommandLineA: {:?}", cmdline.to_string_lossy());
    }
}
