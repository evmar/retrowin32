//! File API for 16-bit Windows backward compat.

use super::{HFILE, get_state};
use std::io::SeekFrom;
use win32_system::System;
use win32_winapi::{ERROR, calling_convention::ArrayOut};

#[win32_derive::dllexport]
pub fn _lopen(sys: &dyn System, lpPathName: Option<&str>, iReadWrite: i32) -> HFILE {
    if iReadWrite != 0 {
        todo!();
    }
    super::file::OpenFile(sys, lpPathName, None, 0)
}

#[win32_derive::dllexport]
pub fn _lclose(sys: &dyn System, hFile: HFILE) -> HFILE {
    if get_state(sys).files.remove(hFile).is_none() {
        log::debug!("CloseHandle({hFile:?}): unknown handle");
        sys.set_last_error(ERROR::INVALID_HANDLE);
        // Docs don't mention any error handling, this is just a guess!
        return HFILE::null();
    }

    sys.set_last_error(ERROR::SUCCESS);
    hFile
}

#[win32_derive::dllexport]
pub fn _llseek(sys: &dyn System, hFile: HFILE, lOffset: i32, iOrigin: i32) -> i32 {
    let mut state = get_state(sys);
    let Some(file) = state.files.get_mut(hFile) else {
        sys.set_last_error(ERROR::INVALID_HANDLE);
        return -1;
    };

    let seek = match iOrigin {
        0 => SeekFrom::Start(lOffset as u64),
        1 => SeekFrom::Current(lOffset as i64),
        2 => SeekFrom::End(lOffset as i64),
        _ => {
            sys.set_last_error(ERROR::INVALID_PARAMETER);
            return -1;
        }
    };
    match file.seek(seek) {
        Ok(new_pos) => new_pos as i32,
        Err(_) => {
            sys.set_last_error(ERROR::INVALID_HANDLE);
            -1
        }
    }
}

#[win32_derive::dllexport]
pub fn _lread(sys: &dyn System, hFile: HFILE, mut lpBuffer: ArrayOut<u8>) -> i32 {
    let mut state = get_state(sys);
    let file = state.files.get_mut(hFile).unwrap();
    let Ok(len) = file.read(lpBuffer.as_mut_slice()) else {
        sys.set_last_error(ERROR::INVALID_HANDLE);
        return -1;
    };
    sys.set_last_error(ERROR::SUCCESS);
    len as i32
}

#[win32_derive::dllexport]
pub fn _hread(sys: &dyn System, hFile: HFILE, lpBuffer: ArrayOut<u8>) -> i32 {
    // This function isn't documented, but appears to be the same as _lread
    // with a slightly different buffer size type (UINT/long) that is 32 bits
    // either way.
    _lread(sys, hFile, lpBuffer)
}
