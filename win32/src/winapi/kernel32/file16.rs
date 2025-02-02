use crate::{
    winapi::{calling_convention::ArrayWithSize, types::HFILE},
    Machine,
};

#[win32_derive::dllexport]
pub fn _lopen(_machine: &mut Machine, lpPathName: Option<&str>, iReadWrite: i32) -> HFILE {
    todo!();
}

#[win32_derive::dllexport]
pub fn _lclose(_machine: &mut Machine, hFile: HFILE) -> HFILE {
    todo!();
}

#[win32_derive::dllexport]
pub fn _llseek(_machine: &mut Machine, hFile: HFILE, lOffset: i32, iOrigin: i32) -> i32 {
    todo!();
}

#[win32_derive::dllexport]
pub fn _lread(_machine: &mut Machine, hFile: HFILE, lpBuffer: ArrayWithSize<u8>) -> u32 {
    todo!();
}
