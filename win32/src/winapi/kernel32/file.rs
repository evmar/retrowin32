use crate::{
    machine::Machine,
    winapi::{
        stack_args::{ArrayWithSize, ArrayWithSizeMut},
        types::{Str16, HFILE},
    },
};

const TRACE_CONTEXT: &'static str = "kernel32/file";

#[derive(Debug)]
pub enum STD {
    INPUT_HANDLE = -10,
    OUTPUT_HANDLE = -11,
    ERROR_HANDLE = -12,
}
impl TryFrom<u32> for STD {
    type Error = u32;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(match value as i32 {
            -10 => STD::INPUT_HANDLE,
            -11 => STD::OUTPUT_HANDLE,
            -12 => STD::ERROR_HANDLE,
            _ => return Err(value),
        })
    }
}

// For now, a magic variable  that makes it easier to spot.
pub const STDIN_HFILE: HFILE = HFILE::from_raw(0xF11E_0100);
pub const STDOUT_HFILE: HFILE = HFILE::from_raw(0xF11E_0101);
pub const STDERR_HFILE: HFILE = HFILE::from_raw(0xF11E_0102);

#[win32_derive::dllexport]
pub fn GetStdHandle(_machine: &mut Machine, nStdHandle: Result<STD, u32>) -> HFILE {
    match nStdHandle {
        Ok(STD::INPUT_HANDLE) => STDIN_HFILE,
        Ok(STD::OUTPUT_HANDLE) => STDOUT_HFILE,
        Ok(STD::ERROR_HANDLE) => STDERR_HFILE,
        _ => HFILE::invalid(),
    }
}

#[derive(Debug, win32_derive::TryFromEnum)]
pub enum CreationDisposition {
    OPEN_EXISTING = 3,
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
    lpFileName: Option<&Str16>,
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
pub fn GetFileType(machine: &mut Machine, hFile: HFILE) -> u32 {
    let FILE_TYPE_CHAR = 0x2;
    let FILE_TYPE_UNKNOWN = 0x8;
    match hFile {
        STDIN_HFILE | STDOUT_HFILE | STDERR_HFILE => return FILE_TYPE_CHAR,
        _ => {}
    }
    if machine.state.kernel32.files.get(&hFile).is_some() {
        return FILE_TYPE_CHAR;
    }

    log::error!("GetFileType({hFile:?}) unknown handle");
    FILE_TYPE_UNKNOWN
}

#[win32_derive::dllexport]
pub fn SetFilePointer(
    machine: &mut Machine,
    hFile: HFILE,
    lDistanceToMove: u32,
    lpDistanceToMoveHigh: Option<&mut u32>,
    dwMoveMethod: u32,
) -> u32 {
    const FILE_BEGIN: u32 = 0;
    const INVALID_SET_FILE_POINTER: u32 = !0;

    if lpDistanceToMoveHigh.is_some() {
        unimplemented!();
    }
    if dwMoveMethod != FILE_BEGIN {
        unimplemented!();
    }
    let file = machine.state.kernel32.files.get_mut(&hFile).unwrap();
    if !file.seek(lDistanceToMove) {
        // TODO: SetLastError
        return INVALID_SET_FILE_POINTER;
    }
    lDistanceToMove
}

#[win32_derive::dllexport]
pub fn ReadFile(
    machine: &mut Machine,
    hFile: HFILE,
    lpBuffer: ArrayWithSizeMut<u8>,
    lpNumberOfBytesRead: Option<&mut u32>,
    lpOverlapped: u32,
) -> bool {
    let file = machine.state.kernel32.files.get_mut(&hFile).unwrap();
    // TODO: SetLastError
    file.read(lpBuffer.unwrap(), lpNumberOfBytesRead.unwrap())
}

#[win32_derive::dllexport]
pub fn WriteFile(
    machine: &mut Machine,
    hFile: HFILE,
    lpBuffer: ArrayWithSize<u8>,
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

#[win32_derive::dllexport]
pub fn GetConsoleMode(
    _machine: &mut Machine,
    hConsoleHandle: HFILE,
    lpMode: Option<&mut u32>,
) -> bool {
    *lpMode.unwrap() = 0;
    true
}

#[win32_derive::dllexport]
pub fn GetFullPathNameW(
    _machine: &mut Machine,
    lpFileName: Option<&Str16>,
    nBufferLength: u32,
    lpBuffer: u32,
    lpFilePart: u32,
) -> u32 {
    0 // fail
}
