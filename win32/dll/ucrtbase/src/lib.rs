//! The C runtime library.  This module is also the implementation of msvcrt.dll.

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

mod builtin;
mod init;
mod math;
mod memory;
mod misc;
mod rand;
mod time;

pub use builtin::DLL;
