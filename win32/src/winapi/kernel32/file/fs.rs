use crate::winapi::kernel32::SECURITY_ATTRIBUTES;
use win32_system::System;
use win32_winapi::{ERROR, Str16, WindowsPath, encoding::*};

#[win32_derive::dllexport]
pub fn DeleteFileW(sys: &dyn System, lpFileName: Option<&Str16>) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn DeleteFileA(sys: &dyn System, lpFileName: Option<&str>) -> bool {
    let Some(file_name) = lpFileName else {
        log::debug!("DeleteFileA failed: null lpFileName");
        sys.set_last_error(ERROR::INVALID_DATA);
        return false;
    };

    let path = WindowsPath::new(file_name);
    match sys.host().remove_file(path) {
        Ok(()) => {
            sys.set_last_error(ERROR::SUCCESS);
            true
        }
        Err(err) => {
            log::debug!("DeleteFileA({file_name:?}) failed: {err:?}",);
            sys.set_last_error(err);
            false
        }
    }
}

#[win32_derive::dllexport]
pub fn RemoveDirectoryW(sys: &dyn System, lpPathName: Option<&Str16>) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn RemoveDirectoryA(sys: &dyn System, lpPathName: Option<&str>) -> bool {
    let Some(path_name) = lpPathName else {
        log::debug!("RemoveDirectoryA failed: null lpPathName");
        sys.set_last_error(ERROR::INVALID_DATA);
        return false;
    };

    let path = WindowsPath::new(path_name);
    match sys.host().remove_dir(path) {
        Ok(()) => {
            sys.set_last_error(ERROR::SUCCESS);
            true
        }
        Err(err) => {
            log::debug!("RemoveDirectoryA({path_name:?}) failed: {err:?}",);
            sys.set_last_error(err);
            false
        }
    }
}

fn get_current_directory(sys: &dyn System, buf: &mut dyn Encoder) -> u32 {
    let cwd = match sys.host().current_dir() {
        Ok(value) => value,
        Err(err) => {
            log::debug!("GetCurrentDirectory failed: {err:?}");
            sys.set_last_error(err);
            return 0;
        }
    };

    buf.write_nul(std::str::from_utf8(cwd.as_bytes()).unwrap());
    let len = match buf.status() {
        Ok(len) => len - 1, // exclude nul
        Err(len) => {
            log::debug!("GetCurrentDirectory -> size {}", len);
            return len;
        }
    };

    sys.set_last_error(ERROR::SUCCESS);
    len
}

#[win32_derive::dllexport]
pub fn GetCurrentDirectoryW(sys: &dyn System, nBufferLength: u32, lpBuffer: u32) -> u32 {
    let mut buf = EncoderWide::from_mem(unsafe { sys.mem().detach() }, lpBuffer, nBufferLength);
    get_current_directory(sys, &mut buf)
}

#[win32_derive::dllexport]
pub fn GetCurrentDirectoryA(sys: &dyn System, nBufferLength: u32, lpBuffer: u32) -> u32 {
    let mut buf = EncoderAnsi::from_mem(unsafe { sys.mem().detach() }, lpBuffer, nBufferLength);
    get_current_directory(sys, &mut buf)
}

#[win32_derive::dllexport]
pub fn SetCurrentDirectoryW(sys: &dyn System, lpPathName: Option<&Str16>) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn SetCurrentDirectoryA(sys: &dyn System, lpPathName: Option<&str>) -> bool {
    log::warn!("SetCurrentDirectoryA not implemented");
    true
}

#[win32_derive::dllexport]
pub fn CreateDirectoryW(
    sys: &dyn System,
    lpPathName: Option<&Str16>,
    lpSecurityAttributes: Option<&mut SECURITY_ATTRIBUTES>,
) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn CreateDirectoryA(
    sys: &dyn System,
    lpPathName: Option<&str>,
    lpSecurityAttributes: u32,
) -> bool {
    let Some(path_name) = lpPathName else {
        log::debug!("CreateDirectoryA failed: null lpPathName");
        sys.set_last_error(ERROR::INVALID_DATA);
        return false;
    };

    let path = WindowsPath::new(path_name);
    match sys.host().create_dir(path) {
        Ok(()) => {
            sys.set_last_error(ERROR::SUCCESS);
            true
        }
        Err(error) => {
            log::debug!("CreateDirectoryA({path_name:?}) failed: {error:?}",);
            sys.set_last_error(error);
            false
        }
    }
}

#[win32_derive::dllexport]
pub fn MoveFileW(
    sys: &dyn System,
    lpExistingFileName: Option<&Str16>,
    lpNewFileName: Option<&Str16>,
) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn MoveFileA(
    sys: &dyn System,
    lpExistingFileName: Option<&str>,
    lpNewFileName: Option<&str>,
) -> bool {
    todo!();
}
