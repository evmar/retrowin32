#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use crate::Machine;

mod builtin;

pub use builtin::DLL;
use memory::{Extensions, ExtensionsMut};

#[win32_derive::dllexport]
pub fn PathRemoveFileSpecA(machine: &mut Machine, pszPath: u32) -> bool {
    let path = machine.mem().slicez(pszPath);
    let path = machine.mem().sub32_mut(pszPath, path.len() as u32);
    for (i, c) in path.iter_mut().enumerate().rev() {
        if *c == b'\\' {
            *c = 0;
            return true;
        }
    }
    false
}
