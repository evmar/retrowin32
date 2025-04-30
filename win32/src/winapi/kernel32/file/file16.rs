use super::HFILE;
use crate::{System, calling_convention::ArrayOut};

#[win32_derive::dllexport]
pub fn _lopen(sys: &dyn System, lpPathName: Option<&str>, iReadWrite: i32) -> HFILE {
    todo!();
}

#[win32_derive::dllexport]
pub fn _lclose(sys: &dyn System, hFile: HFILE) -> HFILE {
    todo!();
}

#[win32_derive::dllexport]
pub fn _llseek(sys: &dyn System, hFile: HFILE, lOffset: i32, iOrigin: i32) -> i32 {
    todo!();
}

#[win32_derive::dllexport]
pub fn _lread(sys: &dyn System, hFile: HFILE, lpBuffer: ArrayOut<u8>) -> u32 {
    todo!();
}
