//! File API for 16-bit Windows backward compat.

use std::io::SeekFrom;

use super::{HFILE, OpenFile};
use crate::{Machine, calling_convention::ArrayOut, winapi::kernel32::set_last_error};
use win32_winapi::ERROR;

#[win32_derive::dllexport]
pub fn _lopen(machine: &mut Machine, lpPathName: Option<&str>, iReadWrite: i32) -> HFILE {
    if iReadWrite != 0 {
        todo!();
    }
    OpenFile(machine, lpPathName, None, 0)
}

#[win32_derive::dllexport]
pub fn _lclose(machine: &mut Machine, hFile: HFILE) -> HFILE {
    if machine.state.kernel32.files.remove(hFile).is_none() {
        log::debug!("CloseHandle({hFile:?}): unknown handle");
        set_last_error(machine, ERROR::INVALID_HANDLE);
        // Docs don't mention any error handling, this is just a guess!
        return HFILE::null();
    }

    set_last_error(machine, ERROR::SUCCESS);
    hFile
}

#[win32_derive::dllexport]
pub fn _llseek(machine: &mut Machine, hFile: HFILE, lOffset: i32, iOrigin: i32) -> i32 {
    let Some(file) = machine.state.kernel32.files.get_mut(hFile) else {
        set_last_error(machine, ERROR::INVALID_HANDLE);
        return -1;
    };

    let seek = match iOrigin {
        0 => SeekFrom::Start(lOffset as u64),
        1 => SeekFrom::Current(lOffset as i64),
        2 => SeekFrom::End(lOffset as i64),
        _ => {
            set_last_error(machine, ERROR::INVALID_PARAMETER);
            return -1;
        }
    };
    match file.seek(seek) {
        Ok(new_pos) => new_pos as i32,
        Err(_) => {
            set_last_error(machine, ERROR::INVALID_HANDLE);
            -1
        }
    }
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
