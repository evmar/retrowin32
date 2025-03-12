#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

mod builtin;

use crate::Machine;
pub use builtin::DLL;

#[win32_derive::dllexport(ordinal = 17)]
pub fn InitCommonControls(_machine: &mut Machine) {}
