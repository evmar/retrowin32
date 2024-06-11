use crate::{
    host,
    machine::Machine,
    winapi::{
        stack_args::{ArrayWithSize, ArrayWithSizeMut},
        types::{Str16, HFILE},
    },
};
use bitflags::bitflags;
use memory::Pod;

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

#[win32_derive::dllexport]
pub fn SetStdHandle(_machine: &mut Machine, nStdHandle: Result<STD, u32>, hHandle: u32) -> bool {
    true // succees
}

#[derive(Debug, win32_derive::TryFromEnum)]
pub enum CreationDisposition {
    CREATE_ALWAYS = 2,
    OPEN_EXISTING = 3,
}

bitflags! {
    pub struct FileAttribute: u32 {
        const NORMAL = 0x80;
    }
}
impl TryFrom<u32> for FileAttribute {
    type Error = u32;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        FileAttribute::from_bits(value).ok_or(value)
    }
}

bitflags! {
    pub struct GENERIC: u32 {
        const ALL = 0x10000000;
        const EXECUTE = 0x20000000;
        const WRITE = 0x40000000;
        const READ = 0x80000000;
    }
}
impl TryFrom<u32> for GENERIC {
    type Error = u32;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        GENERIC::from_bits(value).ok_or(value)
    }
}

#[win32_derive::dllexport]
pub fn CreateFileA(
    machine: &mut Machine,
    lpFileName: Option<&str>,
    dwDesiredAccess: u32,
    dwShareMode: u32,
    lpSecurityAttributes: u32,
    dwCreationDisposition: Result<CreationDisposition, u32>,
    dwFlagsAndAttributes: Result<FileAttribute, u32>,
    hTemplateFile: HFILE,
) -> HFILE {
    let file_name = lpFileName.unwrap();

    // If this from_bits fails, it's due to more complex access bits; see ACCESS_MASK in MSDN docs.
    let generic_access = GENERIC::from_bits(dwDesiredAccess).unwrap();
    let access = match generic_access {
        GENERIC::READ => host::FileAccess::READ,
        GENERIC::WRITE => host::FileAccess::WRITE,
        _ => unimplemented!("access {generic_access:?}"),
    };

    let _dwCreationDisposition = dwCreationDisposition.unwrap();
    // TODO: pass creation dispositions for create new vs existing etc.

    let attr = dwFlagsAndAttributes.unwrap();
    if attr - FileAttribute::NORMAL != FileAttribute::empty() {
        todo!();
    }

    if !hTemplateFile.is_null() {
        unimplemented!("hTemplateFile {hTemplateFile:?}");
    }

    let file = machine.host.open(file_name, access);
    machine.state.kernel32.files.add(file)
}

#[win32_derive::dllexport]
pub fn CreateFileW(
    machine: &mut Machine,
    lpFileName: Option<&Str16>,
    dwDesiredAccess: u32,
    dwShareMode: u32,
    lpSecurityAttributes: u32,
    dwCreationDisposition: Result<CreationDisposition, u32>,
    dwFlagsAndAttributes: Result<FileAttribute, u32>,
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
    if machine.state.kernel32.files.get(hFile).is_some() {
        return FILE_TYPE_CHAR;
    }

    log::error!("GetFileType({hFile:?}) unknown handle");
    FILE_TYPE_UNKNOWN
}

#[repr(C)]
#[derive(Debug)]
pub struct FILETIME {
    dwLowDateTime: u32,
    dwHighDateTime: u32,
}
unsafe impl memory::Pod for FILETIME {}

#[repr(C)]
#[derive(Debug)]
pub struct BY_HANDLE_FILE_INFORMATION {
    pub dwFileAttributes: u32,
    pub ftCreationTime: FILETIME,
    pub ftLastAccessTime: FILETIME,
    pub ftLastWriteTime: FILETIME,
    pub dwVolumeSerialNumber: u32,
    pub nFileSizeHigh: u32,
    pub nFileSizeLow: u32,
    pub nNumberOfLinks: u32,
    pub nFileIndexHigh: u32,
    pub nFileIndexLow: u32,
}
unsafe impl memory::Pod for BY_HANDLE_FILE_INFORMATION {}

#[win32_derive::dllexport]
pub fn GetFileInformationByHandle(
    machine: &mut Machine,
    hFile: HFILE,
    lpFileInformation: Option<&mut BY_HANDLE_FILE_INFORMATION>,
) -> bool {
    let file = match machine.state.kernel32.files.get(hFile) {
        Some(f) => f,
        None => todo!(),
    };

    let info = lpFileInformation.unwrap();
    *info = BY_HANDLE_FILE_INFORMATION::zeroed();
    info.nFileSizeLow = file.info();

    true
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
    let file = machine.state.kernel32.files.get_mut(hFile).unwrap();
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
    let file = match hFile {
        STDIN_HFILE => unimplemented!("ReadFile(stdin)"),
        _ => machine.state.kernel32.files.get_mut(hFile).unwrap(),
    };
    // TODO: SetLastError
    let n = file.read(lpBuffer.unwrap()).unwrap();
    if let Some(bytes) = lpNumberOfBytesRead {
        *bytes = n as u32;
    }
    true
}

#[win32_derive::dllexport]
pub fn WriteFile(
    machine: &mut Machine,
    hFile: HFILE,
    lpBuffer: ArrayWithSize<u8>,
    lpNumberOfBytesWritten: Option<&mut u32>,
    lpOverlapped: u32,
) -> bool {
    assert!(lpOverlapped == 0);

    let buf = lpBuffer.unwrap();
    let n = match hFile {
        STDOUT_HFILE | STDERR_HFILE => {
            machine.host.log(buf);
            buf.len()
        }
        _ => {
            let file = machine.state.kernel32.files.get_mut(hFile).unwrap();
            file.write(buf).unwrap()
        }
    };

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
    machine: &mut Machine,
    lpFileName: Option<&Str16>,
    nBufferLength: u32,
    lpBuffer: u32,
    lpFilePart: Option<&mut u32>,
) -> u32 {
    let file_name = lpFileName.unwrap();
    let buf = Str16::from_bytes_mut(
        machine
            .mem()
            .sub(lpBuffer, nBufferLength * 2)
            .as_mut_slice_todo(),
    );
    if let Some(_part) = lpFilePart {
        todo!();
    }

    if buf.len() < file_name.len() + 1 {
        // not enough space
        return file_name.len() as u32 + 1;
    }

    buf[..file_name.len()].copy_from_slice(file_name);
    buf[file_name.len()] = 0;

    file_name.len() as u32
}

#[win32_derive::dllexport]
pub fn DeleteFileA(_machine: &mut Machine, lpFileName: Option<&str>) -> bool {
    true
}
