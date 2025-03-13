use crate::{
    machine::Machine,
    winapi::{kernel32::set_last_error, Str16, String16, ERROR},
};
use memory::ExtensionsMut;

pub const MAX_PATH: usize = 260;

#[win32_derive::dllexport]
pub fn GetFullPathNameA(
    machine: &mut Machine,
    lpFileName: Option<&str>,
    nBufferLength: u32,
    lpBuffer: u32,
    lpFilePart: Option<&mut u32>,
) -> u32 {
    let Some(file_name) = lpFileName else {
        log::debug!("GetFullPathNameA failed: null lpFileName");
        set_last_error(machine, ERROR::INVALID_DATA);
        return 0;
    };

    let cwd = match machine.host.current_dir() {
        Ok(value) => value,
        Err(err) => {
            log::debug!("GetFullPathNameA({file_name:?}) failed: {err:?}",);
            set_last_error(machine, err);
            return 0;
        }
    };
    let out_path = cwd.join(file_name).normalize();
    let out_bytes = out_path.as_bytes();

    set_last_error(machine, ERROR::SUCCESS);

    let buf = machine.mem().sub32_mut(lpBuffer, nBufferLength);
    if let Some(part) = lpFilePart {
        if let Some(i) = out_bytes.iter().rposition(|&b| b == b'\\') {
            if i == out_bytes.len() - 1 {
                *part = 0;
            } else {
                *part = lpBuffer + i as u32 + 1;
            }
        } else {
            *part = 0;
        }
    }

    if buf.len() < out_bytes.len() + 1 {
        // not enough space
        log::debug!(
            "GetFullPathNameA({file_name:?}) -> size {}",
            file_name.len() + 1
        );
        return out_bytes.len() as u32 + 1;
    }

    buf[..out_bytes.len()].copy_from_slice(out_bytes);
    buf[out_bytes.len()] = 0;

    out_bytes.len() as u32
}

#[win32_derive::dllexport]
pub fn GetFullPathNameW(
    machine: &mut Machine,
    lpFileName: Option<&Str16>,
    nBufferLength: u32,
    lpBuffer: u32,
    lpFilePart: Option<&mut u32>,
) -> u32 {
    let Some(file_name) = lpFileName else {
        log::debug!("GetFullPathNameW failed: null lpFileName");
        set_last_error(machine, ERROR::INVALID_DATA);
        return 0;
    };

    let file_name = file_name.to_string();
    let cwd = match machine.host.current_dir() {
        Ok(value) => value,
        Err(err) => {
            log::debug!("GetFullPathNameW({file_name:?}) failed: {err:?}",);
            set_last_error(machine, err);
            return 0;
        }
    };
    let out_path = cwd.join(&file_name).normalize();
    let out_bytes = String16::from(out_path.to_string_lossy().as_ref()).0;

    set_last_error(machine, ERROR::SUCCESS);

    let buf = Str16::from_bytes_mut(machine.mem().sub32_mut(lpBuffer, nBufferLength * 2));
    if let Some(part) = lpFilePart {
        if let Some(i) = out_bytes.iter().rposition(|&b| b == b'\\' as u16) {
            if i == out_bytes.len() - 1 {
                *part = 0;
            } else {
                *part = lpBuffer + (i as u32 + 1) * 2;
            }
        } else {
            *part = 0;
        }
    }

    if buf.len() < out_bytes.len() + 1 {
        // not enough space
        log::debug!(
            "GetFullPathNameW({file_name:?}) -> size {}",
            file_name.len() + 1
        );
        return out_bytes.len() as u32 + 1;
    }

    buf[..out_bytes.len()].copy_from_slice(&out_bytes);
    buf[out_bytes.len()] = 0;

    out_bytes.len() as u32
}
