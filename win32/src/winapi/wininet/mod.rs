#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

mod builtin;

pub use builtin::DLL;

use crate::Machine;

#[win32_derive::dllexport]
pub fn InternetOpenA(
    _machine: &mut Machine,
    lpszAgent: Option<&str>,
    dwAccessType: u32,
    lpszProxy: Option<&str>,
    lpszProxyBypass: Option<&str>,
    dwFlags: u32,
) -> u32 {
    0
}
