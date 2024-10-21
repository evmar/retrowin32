#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use crate::Machine;

mod builtin;

pub use builtin::DLL;

#[win32_derive::dllexport]
pub fn PathRemoveFileSpecA(_machine: &mut Machine, pszPath: Option<&mut str>) -> bool {
    todo!()
}
