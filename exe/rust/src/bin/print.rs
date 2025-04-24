//! Test our println! replacement.
//! Exercises core::fmt.

#![no_std]
#![no_main]
#![windows_subsystem = "console"]

use exe::println;

#[unsafe(no_mangle)]
pub extern "C" fn mainCRTStartup() {
    println!("hello, world");
}
