use super::{FileAttribute, STDERR_HFILE, STDIN_HFILE, STDOUT_HFILE};
use crate::Machine;
use bitflags::bitflags;
use memory::Pod;
use win32_system::{System, host};
use win32_winapi::{
    ERROR, HANDLE, Str16, WindowsPath,
    calling_convention::{Array, ArrayOut},
};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct HFILET;
pub type HFILE = HANDLE<HFILET>;

// https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilea
#[derive(Copy, Clone, Eq, PartialEq, Debug, win32_derive::TryFromEnum)]
pub enum CreationDisposition {
    CREATE_NEW = 1,
    CREATE_ALWAYS = 2,
    OPEN_EXISTING = 3,
    OPEN_ALWAYS = 4,
    TRUNCATE_EXISTING = 5,
}

bitflags! {
    #[derive(Debug, win32_derive::TryFromBitflags)]
    pub struct GENERIC: u32 {
        const ALL = 0x10000000;
        const EXECUTE = 0x20000000;
        const WRITE = 0x40000000;
        const READ = 0x80000000;
    }
}

/// The dwFlagsAndAttributes param to CreateFile combines FileAttribute with other flags.
#[derive(Debug)]
pub struct FlagsAndAttributes {
    #[allow(unused)] // logged in debugging
    flags: u32,
    attr: FileAttribute,
}

impl TryFrom<u32> for FlagsAndAttributes {
    type Error = u32;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let attr = FileAttribute::from_bits(value & FileAttribute::all().bits()).unwrap();
        let flags = value & !FileAttribute::all().bits();
        Ok(FlagsAndAttributes { flags, attr })
    }
}

#[win32_derive::dllexport]
pub fn CreateFileA(
    machine: &mut Machine,
    lpFileName: Option<&str>,
    dwDesiredAccess: Result<GENERIC, u32>,
    dwShareMode: u32,
    lpSecurityAttributes: u32,
    dwCreationDisposition: Result<CreationDisposition, u32>,
    dwFlagsAndAttributes: Result<FlagsAndAttributes, u32>,
    hTemplateFile: HFILE,
) -> HFILE {
    let Some(file_name) = lpFileName else {
        log::debug!("CreateFileA failed: null lpFileName");
        machine.set_last_error(ERROR::INVALID_DATA);
        return HFILE::invalid();
    };

    // If this from_bits fails, it's due to more complex access bits; see ACCESS_MASK in MSDN docs.
    let mut generic_access = dwDesiredAccess.unwrap();
    let creation_disposition = match dwCreationDisposition {
        Ok(value) => value,
        Err(value) => {
            log::debug!("CreateFileA({file_name:?}) failed: invalid dwCreationDisposition {value}");
            machine.set_last_error(ERROR::INVALID_DATA);
            return HFILE::invalid();
        }
    };

    if generic_access.is_empty() {
        // Windows can create a file with no access rights, for querying metadata;
        // model that as read access for now.
        generic_access |= GENERIC::READ;
    }

    // https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilea
    let file_options = host::FileOptions {
        read: generic_access.contains(GENERIC::READ),
        write: generic_access.contains(GENERIC::WRITE),
        truncate: matches!(
            creation_disposition,
            CreationDisposition::CREATE_ALWAYS | CreationDisposition::TRUNCATE_EXISTING
        ),
        create: matches!(
            creation_disposition,
            CreationDisposition::CREATE_ALWAYS | CreationDisposition::OPEN_ALWAYS
        ),
        create_new: creation_disposition == CreationDisposition::CREATE_NEW,
    };

    let flags = dwFlagsAndAttributes.unwrap();
    if !(flags.attr & !FileAttribute::NORMAL).is_empty() {
        unimplemented!("dwFlagsAndAttributes {attr:?}", attr = flags.attr);
    }

    if !hTemplateFile.is_null() {
        unimplemented!("hTemplateFile {hTemplateFile:?}");
    }

    let path = WindowsPath::new(file_name);
    match machine.host.open(path, file_options) {
        Ok(file) => {
            machine.set_last_error(ERROR::SUCCESS);
            machine.state.kernel32.files.add(file)
        }
        Err(err) => {
            log::debug!("CreateFileA({file_name:?}) failed: {err:?}",);
            machine.set_last_error(err);
            HFILE::invalid()
        }
    }
}

#[win32_derive::dllexport]
pub fn CreateFileW(
    machine: &mut Machine,
    lpFileName: Option<&Str16>,
    dwDesiredAccess: Result<GENERIC, u32>,
    dwShareMode: u32,
    lpSecurityAttributes: u32,
    dwCreationDisposition: Result<CreationDisposition, u32>,
    dwFlagsAndAttributes: Result<FlagsAndAttributes, u32>,
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

#[repr(C)]
#[derive(Debug)]
pub struct OFSTRUCT {
    cBytes: u8,
    fFixedDisk: u8,
    nErrCode: u16,
    Reserved1: u16,
    Reserved2: u16,
    szPathName: [i8; 128],
}
unsafe impl Pod for OFSTRUCT {}

#[win32_derive::dllexport]
pub fn OpenFile(
    machine: &mut Machine,
    lpFileName: Option<&str>,
    lpReOpenBuff: Option<&mut OFSTRUCT>,
    uStyle: u32,
) -> HFILE {
    if uStyle != 0 {
        todo!();
    }

    let file_options = host::FileOptions {
        read: true,
        write: false,
        truncate: false,
        create: false,
        create_new: false,
    };

    let file_name = lpFileName.unwrap();
    let path = WindowsPath::new(file_name);
    let hfile = match machine.host.open(path, file_options) {
        Ok(file) => {
            machine.set_last_error(ERROR::SUCCESS);
            machine.state.kernel32.files.add(file)
        }
        Err(err) => {
            log::debug!("CreateFileA({file_name:?}) failed: {err:?}",);
            machine.set_last_error(err);
            return HFILE::invalid();
        }
    };

    if let Some(of) = lpReOpenBuff {
        *of = OFSTRUCT {
            cBytes: std::mem::size_of::<OFSTRUCT>() as u8,
            fFixedDisk: true as u8,
            nErrCode: 0,
            Reserved1: 0,
            Reserved2: 0,
            szPathName: [0; 128],
        };
    }

    hfile
}

#[win32_derive::dllexport]
pub fn ReadFile(
    machine: &mut Machine,
    hFile: HFILE,
    mut lpBuffer: ArrayOut<u8>,
    mut lpNumberOfBytesRead: Option<&mut u32>,
    lpOverlapped: u32,
) -> bool {
    // "ReadFile sets this value to zero before doing any work or error checking."
    if let Some(bytes) = lpNumberOfBytesRead.as_deref_mut() {
        *bytes = 0;
    }
    let Some(file) = (match hFile {
        STDIN_HFILE => unimplemented!("ReadFile(stdin)"),
        _ => machine.state.kernel32.files.get_mut(hFile),
    }) else {
        log::debug!("ReadFile({hFile:?}) unknown handle");
        machine.set_last_error(ERROR::INVALID_HANDLE);
        return false;
    };
    if lpOverlapped != 0 {
        unimplemented!("ReadFile overlapped");
    }
    let mut buf = lpBuffer.as_mut_slice();

    let mut read = 0;
    while !buf.is_empty() {
        match file.read(buf) {
            Ok(0) => break,
            Ok(n) => {
                read += n;
                buf = &mut buf[n..];
            }
            Err(err) => {
                log::debug!("ReadFile({hFile:?}) failed: {:?}", err);
                machine.set_last_error(ERROR::from(err));
                return false;
            }
        }
    }

    machine.set_last_error(ERROR::SUCCESS);
    if let Some(bytes) = lpNumberOfBytesRead {
        *bytes = read as u32;
    }
    true
}

pub fn write_file(machine: &mut Machine, hFile: HFILE, mut buf: &[u8]) -> Result<usize, ERROR> {
    match hFile {
        STDOUT_HFILE | STDERR_HFILE => {
            machine.host.stdout(buf);
            return Ok(buf.len());
        }
        _ => {}
    };

    let Some(file) = machine.state.kernel32.files.get_mut(hFile) else {
        log::debug!("WriteFile({hFile:?}) unknown handle");
        return Err(ERROR::INVALID_HANDLE);
    };
    let mut written = 0;
    while !buf.is_empty() {
        match file.write(buf) {
            Ok(0) => break,
            Ok(n) => {
                written += n;
                buf = &buf[n..];
            }
            Err(err) => {
                log::debug!("WriteFile({hFile:?}) failed: {:?}", err);
                return Err(ERROR::from(err));
            }
        }
    }
    Ok(written)
}

#[win32_derive::dllexport]
pub fn WriteFile(
    machine: &mut Machine,
    hFile: HFILE,
    lpBuffer: Array<u8>,
    mut lpNumberOfBytesWritten: Option<&mut u32>,
    lpOverlapped: u32,
) -> bool {
    // "WriteFile sets this value to zero before doing any work or error checking."
    if let Some(bytes) = lpNumberOfBytesWritten.as_deref_mut() {
        *bytes = 0;
    }
    if lpOverlapped != 0 {
        unimplemented!("ReadFile overlapped");
    }

    match write_file(machine, hFile, &lpBuffer) {
        Err(err) => {
            machine.set_last_error(err);
            false
        }
        Ok(n) => {
            machine.set_last_error(ERROR::SUCCESS);
            if let Some(written) = lpNumberOfBytesWritten {
                *written = n as u32;
            }
            true
        }
    }
}

#[win32_derive::dllexport]
pub fn SetEndOfFile(machine: &mut Machine, hFile: HFILE) -> bool {
    let file = match machine.state.kernel32.files.get_mut(hFile) {
        Some(f) => f,
        None => {
            log::debug!("SetEndOfFile({hFile:?}) unknown handle");
            machine.set_last_error(ERROR::INVALID_HANDLE);
            return false;
        }
    };

    let len = match file.seek(std::io::SeekFrom::Current(0)) {
        Ok(pos) => pos,
        Err(err) => {
            log::debug!("SetEndOfFile({hFile:?}) failed: {:?}", err);
            machine.set_last_error(ERROR::from(err));
            return false;
        }
    };
    match file.set_len(len) {
        Ok(()) => {
            machine.set_last_error(ERROR::SUCCESS);
            true
        }
        Err(err) => {
            log::debug!("SetEndOfFile({hFile:?}) failed: {err:?}",);
            machine.set_last_error(err);
            false
        }
    }
}

#[win32_derive::dllexport]
pub fn FlushFileBuffers(machine: &mut Machine, hFile: HFILE) -> bool {
    todo!();
}

#[win32_derive::dllexport]
pub fn LockFile(
    sys: &dyn System,
    hFile: HFILE,
    dwFileOffsetLow: u32,
    dwFileOffsetHigh: u32,
    nNumberOfBytesToLockLow: u32,
    nNumberOfBytesToLockHigh: u32,
) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn UnlockFile(
    sys: &dyn System,
    hFile: HFILE,
    dwFileOffsetLow: u32,
    dwFileOffsetHigh: u32,
    nNumberOfBytesToUnlockLow: u32,
    nNumberOfBytesToUnlockHigh: u32,
) -> bool {
    todo!()
}
