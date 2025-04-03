//! Test our println! replacement.
//! Exercises core::fmt.

#![no_std]
#![no_main]
#![windows_subsystem = "console"]

use exe::println;

#[no_mangle]
pub unsafe extern "C" fn mainCRTStartup() {
    println!("hello, world");
}
