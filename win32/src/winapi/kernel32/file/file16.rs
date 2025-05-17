//! File API for 16-bit Windows backward compat.

use super::HFILE;
use crate::{Machine, System, calling_convention::ArrayOut, winapi::kernel32::set_last_error};
use win32_winapi::ERROR;

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
pub fn _lread(machine: &mut Machine, hFile: HFILE, mut lpBuffer: ArrayOut<u8>) -> i32 {
    let file = machine.state.kernel32.files.get_mut(hFile).unwrap();
    let Ok(len) = file.read(lpBuffer.as_mut_slice()) else {
        set_last_error(machine, ERROR::INVALID_HANDLE);
        return -1;
    };
    set_last_error(machine, ERROR::SUCCESS);
    len as i32
}

#[win32_derive::dllexport]
pub fn _hread(machine: &mut Machine, hFile: HFILE, lpBuffer: ArrayOut<u8>) -> i32 {
    // This function isn't documented, but appears to be the same as _lread
    // with a slightly different buffer size type (UINT/long) that is 32 bits
    // either way.
    _lread(machine, hFile, lpBuffer)
}
