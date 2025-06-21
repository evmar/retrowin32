#![no_std]
#![no_main]
#![windows_subsystem = "console"]

extern crate alloc;

use alloc::{boxed::Box, ffi::CString, vec::Vec};
use core::ffi::CStr;
use exe::println;
use miniz_oxide::{deflate::compress_to_vec, inflate::decompress_to_vec};
use windows_sys::Win32::{
    Foundation::{CloseHandle, GENERIC_READ},
    Storage::FileSystem::{CreateFileA, FILE_ATTRIBUTE_NORMAL, OPEN_EXISTING, ReadFile},
    System::Environment::GetCommandLineA,
};

fn roundtrip(data: &[u8]) {
    let compressed = compress_to_vec(data, 6);
    let decompressed = decompress_to_vec(compressed.as_slice()).unwrap();
    if data != decompressed {
        if data.len() != decompressed.len() {
            panic!("length mismatch: {} != {}", data.len(), decompressed.len());
        }
        for (i, (a, b)) in data.iter().zip(decompressed.iter()).enumerate() {
            if a != b {
                panic!("data mismatch at {i}: {a} != {b}");
            }
        }
    }
}

fn read_file(path: &str) -> Box<[u8]> {
    let mut contents = Vec::new();
    unsafe {
        let f = CreateFileA(
            CString::new(path).unwrap().as_ptr() as *const _,
            GENERIC_READ,
            /* dwShareMode */ 0,
            core::ptr::null_mut(),
            OPEN_EXISTING,
            FILE_ATTRIBUTE_NORMAL,
            /* hTemplateFile */ 0,
        );
        if f == -1 {
            panic!("failed to open file");
        }
        let mut buf = [0u8; 16 << 10];
        loop {
            let mut bytes_read = 0;
            let ok = ReadFile(
                f,
                buf.as_mut_ptr() as *mut _,
                buf.len() as u32,
                &mut bytes_read,
                core::ptr::null_mut(),
            ) != 0;
            if !ok {
                panic!("ReadFile failed");
            }
            if bytes_read == 0 {
                break;
            }
            contents.extend_from_slice(&buf[..bytes_read as usize]);
        }
        CloseHandle(f);
    }
    contents.into_boxed_slice()
}

#[unsafe(no_mangle)]
pub extern "C" fn mainCRTStartup() {
    let cmdline = unsafe { CStr::from_ptr(GetCommandLineA() as *const _) }
        .to_str()
        .unwrap();
    let args = cmdline.split(' ').collect::<Vec<_>>();

    if args.len() != 3 {
        println!("usage: {} reps filename", args[0]);
        return;
    }
    let reps = args[1].parse::<usize>().unwrap();
    let filename = &args[2];

    println!("reading {filename}");
    let buf = read_file(filename);
    println!("read {} bytes", buf.len());

    for i in 0..reps {
        println!("roundtripping {i}", i = i + 1);
        roundtrip(&*buf);
    }
    println!(
        "roundtripped {bytes} bytes {reps} times",
        bytes = buf.len(),
        reps = reps
    );
}
