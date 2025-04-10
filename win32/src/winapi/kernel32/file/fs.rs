use crate::{
    winapi::{
        encoding::{Encoder, EncoderAnsi, EncoderWide},
        kernel32::{set_last_error, SECURITY_ATTRIBUTES},
    },
    Machine, ERROR,
};
use memory::str16::Str16;
use typed_path::WindowsPath;

#[win32_derive::dllexport]
pub fn DeleteFileW(_machine: &mut Machine, lpFileName: Option<&Str16>) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn DeleteFileA(machine: &mut Machine, lpFileName: Option<&str>) -> bool {
    let Some(file_name) = lpFileName else {
        log::debug!("DeleteFileA failed: null lpFileName");
        set_last_error(machine, ERROR::INVALID_DATA);
        return false;
    };

    let path = WindowsPath::new(file_name);
    match machine.host.remove_file(path) {
        Ok(()) => {
            set_last_error(machine, ERROR::SUCCESS);
            true
        }
        Err(err) => {
            log::debug!("DeleteFileA({file_name:?}) failed: {err:?}",);
            set_last_error(machine, err);
            false
        }
    }
}

#[win32_derive::dllexport]
pub fn RemoveDirectoryW(_machine: &mut Machine, lpPathName: Option<&Str16>) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn RemoveDirectoryA(machine: &mut Machine, lpPathName: Option<&str>) -> bool {
    let Some(path_name) = lpPathName else {
        log::debug!("RemoveDirectoryA failed: null lpPathName");
        set_last_error(machine, ERROR::INVALID_DATA);
        return false;
    };

    let path = WindowsPath::new(path_name);
    match machine.host.remove_dir(path) {
        Ok(()) => {
            set_last_error(machine, ERROR::SUCCESS);
            true
        }
        Err(err) => {
            log::debug!("RemoveDirectoryA({path_name:?}) failed: {err:?}",);
            set_last_error(machine, err);
            false
        }
    }
}

fn get_current_directory(machine: &mut Machine, buf: &mut dyn Encoder) -> u32 {
    let cwd = match machine.host.current_dir() {
        Ok(value) => value,
        Err(err) => {
            log::debug!("GetCurrentDirectory failed: {err:?}");
            set_last_error(machine, err);
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

    set_last_error(machine, ERROR::SUCCESS);
    len
}

#[win32_derive::dllexport]
pub fn GetCurrentDirectoryW(machine: &mut Machine, nBufferLength: u32, lpBuffer: u32) -> u32 {
    let mut buf = EncoderWide::from_mem(unsafe { machine.mem().detach() }, lpBuffer, nBufferLength);
    get_current_directory(machine, &mut buf)
}

#[win32_derive::dllexport]
pub fn GetCurrentDirectoryA(machine: &mut Machine, nBufferLength: u32, lpBuffer: u32) -> u32 {
    let mut buf = EncoderAnsi::from_mem(unsafe { machine.mem().detach() }, lpBuffer, nBufferLength);
    get_current_directory(machine, &mut buf)
}

#[win32_derive::dllexport]
pub fn SetCurrentDirectoryW(_machine: &mut Machine, lpPathName: Option<&Str16>) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn SetCurrentDirectoryA(_machine: &mut Machine, lpPathName: Option<&str>) -> bool {
    log::warn!("SetCurrentDirectoryA not implemented");
    true
}

#[win32_derive::dllexport]
pub fn CreateDirectoryW(
    _machine: &mut Machine,
    lpPathName: Option<&Str16>,
    lpSecurityAttributes: Option<&mut SECURITY_ATTRIBUTES>,
) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn CreateDirectoryA(
    machine: &mut Machine,
    lpPathName: Option<&str>,
    lpSecurityAttributes: u32,
) -> bool {
    let Some(path_name) = lpPathName else {
        log::debug!("CreateDirectoryA failed: null lpPathName");
        set_last_error(machine, ERROR::INVALID_DATA);
        return false;
    };

    let path = WindowsPath::new(path_name);
    match machine.host.create_dir(path) {
        Ok(()) => {
            set_last_error(machine, ERROR::SUCCESS);
            true
        }
        Err(error) => {
            log::debug!("CreateDirectoryA({path_name:?}) failed: {error:?}",);
            set_last_error(machine, error);
            false
        }
    }
}

#[win32_derive::dllexport]
pub fn MoveFileW(
    _machine: &mut Machine,
    lpExistingFileName: Option<&Str16>,
    lpNewFileName: Option<&Str16>,
) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn MoveFileA(
    _machine: &mut Machine,
    lpExistingFileName: Option<&str>,
    lpNewFileName: Option<&str>,
) -> bool {
    todo!();
}
