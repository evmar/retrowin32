#![no_main]
#![no_std]
#![windows_subsystem = "console"]

use no_std::{print::print, println};
use windows_sys::Win32::UI::WindowsAndMessaging::GetSystemMetrics;

#[no_mangle]
pub unsafe extern "C" fn mainCRTStartup() {
    print(b"GetSystemMetrics():\r\n");
    for i in 0..100 {
        let metric = GetSystemMetrics(i);
        println!("{} => {}", i, metric);
    }
}
