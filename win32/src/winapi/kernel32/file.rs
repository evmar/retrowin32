use crate::{
    machine::Machine,
    winapi::types::{Str16, HFILE},
};

const TRACE: bool = true;

// For now, a magic variable that makes it easier to spot.
pub const STDIN_HFILE: HFILE = HFILE::from_raw(0xF11E_0100);
pub const STDOUT_HFILE: HFILE = HFILE::from_raw(0xF11E_0101);
pub const STDERR_HFILE: HFILE = HFILE::from_raw(0xF11E_0102);

#[derive(Debug)]
pub enum CreationDisposition {
    OPEN_EXISTING = 3,
}
impl TryFrom<u32> for CreationDisposition {
    type Error = u32;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(match value {
            x if x == CreationDisposition::OPEN_EXISTING as u32 => {
                CreationDisposition::OPEN_EXISTING
            }
            _ => return Err(value),
        })
    }
}

pub const FILE_ATTRIBUTE_NORMAL: u32 = 0x80;

const GENERIC_READ: u32 = 0x8000_0000;

#[win32_derive::dllexport]
pub fn CreateFileA(
    machine: &mut Machine,
    lpFileName: Option<&str>,
    dwDesiredAccess: u32,
    dwShareMode: u32,
    lpSecurityAttributes: u32,
    dwCreationDisposition: Result<CreationDisposition, u32>,
    dwFlagsAndAttributes: u32,
    hTemplateFile: HFILE,
) -> HFILE {
    let file_name = lpFileName.unwrap();
    if dwDesiredAccess != GENERIC_READ {
        unimplemented!("CreateFile access {:x}", dwDesiredAccess);
    }
    let _dwCreationDisposition = dwCreationDisposition.unwrap();
    if dwFlagsAndAttributes != FILE_ATTRIBUTE_NORMAL {
        unimplemented!("dwFlagsAndAttributes {dwFlagsAndAttributes:x}");
    }
    if !hTemplateFile.is_null() {
        unimplemented!("hTemplateFile {hTemplateFile:?}");
    }

    let file = machine.host.open(file_name);
    let hfile = HFILE::from_raw(0xF11E_0001);
    machine.state.kernel32.files.insert(hfile, file);
    hfile
}

#[win32_derive::dllexport]
pub fn CreateFileW(
    machine: &mut Machine,
    lpFileName: Option<Str16>,
    dwDesiredAccess: u32,
    dwShareMode: u32,
    lpSecurityAttributes: u32,
    dwCreationDisposition: Result<CreationDisposition, u32>,
    dwFlagsAndAttributes: u32,
    hTemplateFile: HFILE,
) -> HFILE {
    CreateFileA(
        machine,
        lpFileName
            .map(|f| f.to_string())
            .as_ref()
            .map(|f| f.as_str()),
        dwDesiredAccess,
        dwShareMode,
        lpSecurityAttributes,
        dwCreationDisposition,
        dwFlagsAndAttributes,
        hTemplateFile,
    )
}

#[win32_derive::dllexport]
pub fn WriteFile(
    machine: &mut Machine,
    hFile: HFILE,
    lpBuffer: Option<&[u8]>,
    lpNumberOfBytesWritten: Option<&mut u32>,
    lpOverlapped: u32,
) -> bool {
    assert!(hFile == STDOUT_HFILE || hFile == STDERR_HFILE);
    assert!(lpOverlapped == 0);

    let n = machine.host.write(lpBuffer.unwrap());

    // The docs say this parameter may not be null, but a test program with the param as null
    // runs fine on real Windows...
    if let Some(written) = lpNumberOfBytesWritten {
        *written = n as u32;
    }
    true
}
