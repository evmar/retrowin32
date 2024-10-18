#![no_main]
#![no_std]
#![windows_subsystem = "console"]

use no_std::{fmt::Fmt, print::print};
use windows_sys::Win32::UI::WindowsAndMessaging::GetSystemMetrics;

#[no_mangle]
pub unsafe extern "C" fn mainCRTStartup() {
    print(b"GetSystemMetrics():\r\n");
    for i in 0..100 {
        let metric = GetSystemMetrics(i);
        print(
            Fmt::new()
                .dec(i as u32)
                .str(" => ")
                .dec(metric as u32)
                .str("\r\n")
                .buf(),
        );
    }
}
