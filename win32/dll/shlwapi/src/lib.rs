#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

mod builtin;

pub use builtin::DLL;
use memory::{Extensions, ExtensionsMut};
use win32_system::System;

#[win32_derive::dllexport]
pub fn PathRemoveFileSpecA(sys: &mut dyn System, pszPath: u32) -> bool {
    let path = sys.mem().slicez(pszPath);
    let path = sys.mem().sub32_mut(pszPath, path.len() as u32);
    for (i, c) in path.iter_mut().enumerate().rev() {
        if *c == b'\\' {
            *c = 0;
            return true;
        }
    }
    false
}
