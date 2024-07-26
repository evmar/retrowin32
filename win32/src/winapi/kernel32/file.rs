use crate::str16::String16;
use crate::winapi::kernel32::set_last_error;
use crate::winapi::stack_args::ToX86;
use crate::winapi::types::{
    io_error_to_win32, win32_error_str, DWORD, ERROR_FILE_NOT_FOUND, ERROR_INVALID_DATA,
    ERROR_INVALID_HANDLE, ERROR_SUCCESS, HFIND, MAX_PATH,
};
use crate::{
    machine::Machine,
    winapi::{
        stack_args::{ArrayWithSize, ArrayWithSizeMut},
        types::{Str16, HFILE},
    },
    FileOptions, FindFile, Stat, StatKind,
};
use bitflags::bitflags;
use typed_path::WindowsPath;

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

// https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilea
#[derive(Copy, Clone, Eq, PartialEq, Debug, win32_derive::TryFromEnum)]
pub enum CreationDisposition {
    CREATE_NEW = 1,
    CREATE_ALWAYS = 2,
    OPEN_EXISTING = 3,
    OPEN_ALWAYS = 4,
    TRUNCATE_EXISTING = 5,
}

// https://learn.microsoft.com/en-us/windows/win32/fileio/file-attribute-constants
bitflags! {
    pub struct FileAttribute: u32 {
        const INVALID = u32::MAX;
        const READONLY = 0x1;
        const HIDDEN = 0x2;
        const SYSTEM = 0x4;
        const DIRECTORY = 0x10;
        const ARCHIVE = 0x20;
        const DEVICE = 0x40;
        const NORMAL = 0x80;
        const TEMPORARY = 0x100;
        const SPARSE_FILE = 0x200;
        const REPARSE_POINT = 0x400;
        const COMPRESSED = 0x800;
        const OFFLINE = 0x1000;
        const NOT_CONTENT_INDEXED = 0x2000;
        const ENCRYPTED = 0x4000;
        const INTEGRITY_STREAM = 0x8000;
        const VIRTUAL = 0x10000;
        const NO_SCRUB_DATA = 0x20000;
        const EA = 0x40000;
        const PINNED = 0x80000;
        const UNPINNED = 0x100000;
        const RECALL_ON_OPEN = 0x40000;
        const RECALL_ON_DATA_ACCESS = 0x400000;
    }
}
impl From<&Stat> for FileAttribute {
    fn from(stat: &Stat) -> Self {
        let mut attr = FileAttribute::empty();
        match stat.kind {
            StatKind::File => {
                attr |= FileAttribute::NORMAL;
            }
            StatKind::Directory => {
                attr |= FileAttribute::DIRECTORY;
            }
            StatKind::Symlink => {
                attr |= FileAttribute::REPARSE_POINT;
            }
        }
        attr
    }
}
impl TryFrom<u32> for FileAttribute {
    type Error = u32;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        FileAttribute::from_bits(value).ok_or(value)
    }
}
impl ToX86 for FileAttribute {
    fn to_raw(&self) -> u32 {
        self.bits()
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
    let Some(file_name) = lpFileName else {
        log::debug!("CreateFileA failed: null lpFileName");
        set_last_error(machine, ERROR_INVALID_DATA);
        return HFILE::invalid();
    };

    // If this from_bits fails, it's due to more complex access bits; see ACCESS_MASK in MSDN docs.
    let generic_access = GENERIC::from_bits(dwDesiredAccess).unwrap();
    let creation_disposition = match dwCreationDisposition {
        Ok(value) => value,
        Err(value) => {
            log::debug!("CreateFileA({file_name:?}) failed: invalid dwCreationDisposition {value}");
            set_last_error(machine, ERROR_INVALID_DATA);
            return HFILE::invalid();
        }
    };

    // https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilea
    let file_options = FileOptions {
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

    let attr = dwFlagsAndAttributes.unwrap();
    if !(attr & !FileAttribute::NORMAL).is_empty() {
        unimplemented!("dwFlagsAndAttributes {attr:?}");
    }

    if !hTemplateFile.is_null() {
        unimplemented!("hTemplateFile {hTemplateFile:?}");
    }

    let path = WindowsPath::new(file_name);
    match machine.host.open(path, file_options) {
        Ok(file) => {
            set_last_error(machine, ERROR_SUCCESS);
            machine.state.kernel32.files.add(file)
        }
        Err(code) => {
            log::debug!(
                "CreateFileA({file_name:?}) failed: {}",
                win32_error_str(code)
            );
            set_last_error(machine, code);
            HFILE::invalid()
        }
    }
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

/// Contains a 64-bit value representing the number of 100-nanosecond intervals since
/// January 1, 1601 (UTC).
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct FILETIME {
    pub dwLowDateTime: u32,
    pub dwHighDateTime: u32,
}
unsafe impl memory::Pod for FILETIME {}

/// Number of 100ns intervals between 1601-01-01 and 1970-01-01.
/// Used to convert between Win32 FILETIME and Unix time.
const HNSEC_UNIX_OFFSET: i64 = 116_444_736_000_000_000;

impl FILETIME {
    #[inline]
    pub fn from_u64(value: u64) -> Self {
        FILETIME {
            dwLowDateTime: value as u32,
            dwHighDateTime: (value >> 32) as u32,
        }
    }

    #[inline]
    pub fn to_u64(self) -> u64 {
        (self.dwHighDateTime as u64) << 32 | self.dwLowDateTime as u64
    }

    pub fn from_unix_nanos(nanos: i64) -> Self {
        let hnsecs = nanos.div_euclid(100).saturating_add(HNSEC_UNIX_OFFSET);
        Self::from_u64(if hnsecs < 0 { 0 } else { hnsecs as u64 })
    }

    pub fn to_unix_nanos(self) -> i64 {
        let hnsecs = self.to_u64();
        if hnsecs > i64::MAX as u64 {
            return i64::MAX;
        }
        (hnsecs as i64).saturating_sub(HNSEC_UNIX_OFFSET) * 100
    }
}

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
impl From<&Stat> for BY_HANDLE_FILE_INFORMATION {
    fn from(stat: &Stat) -> Self {
        Self {
            dwFileAttributes: FileAttribute::from(stat).to_raw(),
            ftCreationTime: FILETIME::from_unix_nanos(stat.ctime),
            ftLastAccessTime: FILETIME::from_unix_nanos(stat.atime),
            ftLastWriteTime: FILETIME::from_unix_nanos(stat.mtime),
            dwVolumeSerialNumber: 0,
            nFileSizeHigh: (stat.size >> 32) as u32,
            nFileSizeLow: stat.size as u32,
            nNumberOfLinks: 1,
            nFileIndexHigh: 0,
            nFileIndexLow: 0,
        }
    }
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
        None => {
            log::debug!("GetFileInformationByHandle({hFile:?}) unknown handle");
            set_last_error(machine, ERROR_INVALID_DATA);
            return false;
        }
    };

    let stat = match file.stat() {
        Ok(stat) => stat,
        Err(code) => {
            log::debug!(
                "GetFileInformationByHandle({hFile:?}) failed: {}",
                win32_error_str(code)
            );
            set_last_error(machine, code);
            return false;
        }
    };

    if let Some(info) = lpFileInformation {
        *info = BY_HANDLE_FILE_INFORMATION::from(&stat);
    }

    set_last_error(machine, ERROR_SUCCESS);
    true
}

#[derive(Debug, Eq, PartialEq, win32_derive::TryFromEnum)]
pub enum FILE {
    BEGIN = 0,
    CURRENT = 1,
    END = 2,
}

#[win32_derive::dllexport]
pub fn SetFilePointer(
    machine: &mut Machine,
    hFile: HFILE,
    lDistanceToMove: i32,
    mut lpDistanceToMoveHigh: Option<&mut i32>,
    dwMoveMethod: Result<FILE, u32>,
) -> u32 {
    let mut lDistanceToMove = lDistanceToMove as i64;
    if let Some(high) = &mut lpDistanceToMoveHigh {
        lDistanceToMove |= (**high as i64) << 32;
    }
    let Some(file) = machine.state.kernel32.files.get_mut(hFile) else {
        log::debug!("SetFilePointer({hFile:?}) unknown handle");
        set_last_error(machine, ERROR_INVALID_HANDLE);
        return u32::MAX;
    };
    let seek = match dwMoveMethod.unwrap() {
        FILE::BEGIN => std::io::SeekFrom::Start(lDistanceToMove as u64),
        FILE::CURRENT => std::io::SeekFrom::Current(lDistanceToMove),
        FILE::END => std::io::SeekFrom::End(lDistanceToMove),
    };
    let pos = match file.seek(seek) {
        Ok(pos) => pos,
        Err(err) => {
            log::debug!("SetFilePointer({hFile:?}) failed: {:?}", err);
            set_last_error(machine, io_error_to_win32(&err));
            return u32::MAX;
        }
    };
    if let Some(high) = lpDistanceToMoveHigh {
        *high = (pos >> 32) as i32;
    } else if pos >> 32 != 0 {
        log::debug!("SetFilePointer({hFile:?}) overflow");
        set_last_error(machine, ERROR_INVALID_DATA);
        return u32::MAX;
    }
    set_last_error(machine, ERROR_SUCCESS);
    pos as u32
}

#[win32_derive::dllexport]
pub fn ReadFile(
    machine: &mut Machine,
    hFile: HFILE,
    lpBuffer: ArrayWithSizeMut<u8>,
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
        set_last_error(machine, ERROR_INVALID_HANDLE);
        return false;
    };
    if lpOverlapped != 0 {
        unimplemented!("ReadFile overlapped");
    }
    let Some(mut buf) = lpBuffer.to_option() else {
        log::debug!("ReadFile({hFile:?}) failed: null lpBuffer");
        set_last_error(machine, ERROR_INVALID_DATA);
        return false;
    };

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
                set_last_error(machine, io_error_to_win32(&err));
                return false;
            }
        }
    }

    set_last_error(machine, ERROR_SUCCESS);
    if let Some(bytes) = lpNumberOfBytesRead {
        *bytes = read as u32;
    }
    true
}

#[win32_derive::dllexport]
pub fn WriteFile(
    machine: &mut Machine,
    hFile: HFILE,
    lpBuffer: ArrayWithSize<u8>,
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
    let Some(mut buf) = lpBuffer else {
        log::debug!("WriteFile({hFile:?}) failed: null lpBuffer");
        set_last_error(machine, ERROR_INVALID_DATA);
        return false;
    };

    let n = match hFile {
        STDOUT_HFILE | STDERR_HFILE => {
            machine.host.log(buf);
            buf.len()
        }
        _ => {
            let Some(file) = machine.state.kernel32.files.get_mut(hFile) else {
                log::debug!("WriteFile({hFile:?}) unknown handle");
                set_last_error(machine, ERROR_INVALID_HANDLE);
                return false;
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
                        set_last_error(machine, io_error_to_win32(&err));
                        return false;
                    }
                }
            }
            written
        }
    };

    set_last_error(machine, ERROR_SUCCESS);
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
pub fn GetFullPathNameA(
    machine: &mut Machine,
    lpFileName: Option<&str>,
    nBufferLength: u32,
    lpBuffer: u32,
    lpFilePart: Option<&mut u32>,
) -> u32 {
    let Some(file_name) = lpFileName else {
        log::debug!("GetFullPathNameA failed: null lpFileName");
        set_last_error(machine, ERROR_INVALID_DATA);
        return 0;
    };

    let cwd = match machine.host.current_dir() {
        Ok(value) => value,
        Err(code) => {
            log::debug!(
                "GetFullPathNameA({file_name:?}) failed: {}",
                win32_error_str(code)
            );
            set_last_error(machine, code);
            return 0;
        }
    };
    let out_path = cwd.join(file_name).normalize();
    let out_bytes = out_path.as_bytes();

    set_last_error(machine, ERROR_SUCCESS);

    let buf = machine
        .mem()
        .sub(lpBuffer, nBufferLength)
        .as_mut_slice_todo();
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
        set_last_error(machine, ERROR_INVALID_DATA);
        return 0;
    };

    let file_name = file_name.to_string();
    let cwd = match machine.host.current_dir() {
        Ok(value) => value,
        Err(code) => {
            log::debug!(
                "GetFullPathNameW({file_name:?}) failed: {}",
                win32_error_str(code)
            );
            set_last_error(machine, code);
            return 0;
        }
    };
    let out_path = cwd.join(&file_name).normalize();
    let out_bytes = String16::from(out_path.to_string_lossy().as_ref()).0;

    set_last_error(machine, ERROR_SUCCESS);

    let buf = Str16::from_bytes_mut(
        machine
            .mem()
            .sub(lpBuffer, nBufferLength * 2)
            .as_mut_slice_todo(),
    );
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

    file_name.len() as u32
}

#[win32_derive::dllexport]
pub fn DeleteFileA(_machine: &mut Machine, lpFileName: Option<&str>) -> bool {
    // TODO
    true
}

#[win32_derive::dllexport]
pub fn GetFileAttributesA(machine: &mut Machine, lpFileName: Option<&str>) -> FileAttribute {
    let Some(file_name) = lpFileName else {
        log::debug!("CreateFileA failed: null lpFileName");
        set_last_error(machine, ERROR_INVALID_DATA);
        return FileAttribute::INVALID;
    };

    let path = WindowsPath::new(file_name);
    let stat = match machine.host.stat(path) {
        Ok(stat) => stat,
        Err(code) => {
            log::debug!(
                "GetFileAttributesA({file_name:?}) failed: {}",
                win32_error_str(code)
            );
            set_last_error(machine, code);
            return FileAttribute::INVALID;
        }
    };

    set_last_error(machine, ERROR_SUCCESS);

    match stat.kind {
        StatKind::File => FileAttribute::NORMAL,
        StatKind::Directory => FileAttribute::DIRECTORY,
        StatKind::Symlink => FileAttribute::REPARSE_POINT,
    }
}

#[win32_derive::dllexport]
pub fn GetCurrentDirectoryA(machine: &mut Machine, nBufferLength: u32, lpBuffer: u32) -> u32 {
    let cwd = match machine.host.current_dir() {
        Ok(value) => value,
        Err(code) => {
            log::debug!("GetCurrentDirectoryA failed: {}", win32_error_str(code));
            set_last_error(machine, code);
            return 0;
        }
    };
    let out_bytes = cwd.as_bytes();

    let buf = machine
        .mem()
        .sub(lpBuffer, nBufferLength)
        .as_mut_slice_todo();

    if buf.len() < out_bytes.len() + 1 {
        // not enough space
        log::debug!("GetCurrentDirectoryA -> size {}", out_bytes.len() + 1);
        return out_bytes.len() as u32 + 1;
    }

    buf[..out_bytes.len()].copy_from_slice(out_bytes);
    buf[out_bytes.len()] = 0;

    set_last_error(machine, ERROR_SUCCESS);
    out_bytes.len() as u32
}

#[repr(C)]
#[derive(Debug)]
pub struct WIN32_FIND_DATAA {
    pub dwFileAttributes: DWORD,
    pub ftCreationTime: FILETIME,
    pub ftLastAccessTime: FILETIME,
    pub ftLastWriteTime: FILETIME,
    pub nFileSizeHigh: DWORD,
    pub nFileSizeLow: DWORD,
    pub dwReserved0: DWORD,
    pub dwReserved1: DWORD,
    pub cFileName: [u8; MAX_PATH],
    pub cAlternateFileName: [u8; 14],
}
impl From<&FindFile> for WIN32_FIND_DATAA {
    fn from(file: &FindFile) -> Self {
        let stat = &file.stat;
        let mut data = Self {
            dwFileAttributes: FileAttribute::from(stat).to_raw(),
            ftCreationTime: FILETIME::from_unix_nanos(stat.ctime),
            ftLastAccessTime: FILETIME::from_unix_nanos(stat.atime),
            ftLastWriteTime: FILETIME::from_unix_nanos(stat.mtime),
            nFileSizeHigh: (stat.size >> 32) as DWORD,
            nFileSizeLow: stat.size as DWORD,
            dwReserved0: if stat.kind == StatKind::Symlink {
                0xA000000C // IO_REPARSE_TAG_SYMLINK
            } else {
                0
            },
            dwReserved1: 0,
            cFileName: [0; MAX_PATH],
            cAlternateFileName: [0; 14],
        };
        if file.name.len() >= MAX_PATH {
            unimplemented!("long file name");
        }
        data.cFileName[..file.name.len()].copy_from_slice(file.name.as_bytes());
        data.cFileName[file.name.len()] = 0;
        const FAKE_DOS_NAME: &[u8] = b"FAKEDOSNAME\0";
        data.cAlternateFileName[..FAKE_DOS_NAME.len()].copy_from_slice(FAKE_DOS_NAME);
        data
    }
}
unsafe impl memory::Pod for WIN32_FIND_DATAA {}

#[win32_derive::dllexport]
pub fn FindFirstFileA(
    machine: &mut Machine,
    lpFileName: Option<&str>,
    lpFindFileData: Option<&mut WIN32_FIND_DATAA>,
) -> HFIND {
    let Some(file_name) = lpFileName else {
        log::debug!("FindFirstFileA failed: null lpFileName");
        set_last_error(machine, ERROR_INVALID_DATA);
        return HFIND::invalid();
    };

    if file_name
        .chars()
        // Skip \\?\ prefix
        .skip_while(|&c| c == '\\' || c == '?')
        .any(|c| c == '*' || c == '?')
    {
        todo!("FindFirstFileA({file_name:?}) wildcards");
    }

    let path = WindowsPath::new(file_name);
    let mut handle = match machine.host.find(path) {
        Ok(handle) => handle,
        Err(code) => {
            log::debug!(
                "FindFirstFileA({file_name:?}) failed: {}",
                win32_error_str(code)
            );
            set_last_error(machine, code);
            return HFIND::invalid();
        }
    };

    let next = match handle.next() {
        Ok(Some(stat)) => stat,
        Ok(None) => {
            log::debug!("FindFirstFileA({file_name:?}) not found");
            set_last_error(machine, ERROR_FILE_NOT_FOUND);
            return HFIND::invalid();
        }
        Err(code) => {
            log::warn!(
                "FindFirstFileA({file_name:?}) failed: {}",
                win32_error_str(code)
            );
            set_last_error(machine, code);
            return HFIND::invalid();
        }
    };

    if let Some(data) = lpFindFileData {
        *data = WIN32_FIND_DATAA::from(&next);
    }

    set_last_error(machine, ERROR_SUCCESS);
    machine.state.kernel32.find_handles.add(handle)
}

#[win32_derive::dllexport]
pub fn FindNextFileA(
    machine: &mut Machine,
    hFindFile: HFIND,
    lpFindFileData: Option<&mut WIN32_FIND_DATAA>,
) -> bool {
    let handle = match machine.state.kernel32.find_handles.get_mut(hFindFile) {
        Some(handle) => handle,
        None => {
            log::debug!("FindNextFileA({hFindFile:?}) unknown handle");
            set_last_error(machine, ERROR_INVALID_HANDLE);
            return false;
        }
    };

    let next = match handle.next() {
        Ok(Some(stat)) => stat,
        Ok(None) => {
            set_last_error(machine, ERROR_FILE_NOT_FOUND);
            return false;
        }
        Err(code) => {
            log::warn!(
                "FindNextFileA({hFindFile:?}) failed: {}",
                win32_error_str(code)
            );
            set_last_error(machine, code);
            return false;
        }
    };

    if let Some(data) = lpFindFileData {
        *data = WIN32_FIND_DATAA::from(&next);
    }

    set_last_error(machine, ERROR_SUCCESS);
    true
}

#[win32_derive::dllexport]
pub fn FindClose(machine: &mut Machine, hFindFile: HFIND) -> bool {
    if machine
        .state
        .kernel32
        .find_handles
        .remove(hFindFile)
        .is_none()
    {
        log::debug!("FindClose({hFindFile:?}): unknown handle");
        set_last_error(machine, ERROR_INVALID_HANDLE);
        return false;
    }

    set_last_error(machine, ERROR_SUCCESS);
    true
}

#[win32_derive::dllexport]
pub fn GetFileSize(machine: &mut Machine, hFile: HFILE, lpFileSizeHigh: Option<&mut u32>) -> u32 {
    let file = match machine.state.kernel32.files.get(hFile) {
        Some(f) => f,
        None => {
            log::debug!("GetFileSize({hFile:?}) unknown handle");
            set_last_error(machine, ERROR_INVALID_HANDLE);
            return u32::MAX;
        }
    };

    let stat = match file.stat() {
        Ok(stat) => stat,
        Err(code) => {
            log::debug!("GetFileSize({hFile:?}) failed: {}", win32_error_str(code));
            set_last_error(machine, code);
            return u32::MAX;
        }
    };

    set_last_error(machine, ERROR_SUCCESS);

    if let Some(high) = lpFileSizeHigh {
        *high = (stat.size >> 32) as u32;
    } else if stat.size >> 32 != 0 {
        log::debug!("GetFileSize({hFile:?}) overflow");
        return u32::MAX;
    }
    stat.size as u32
}

#[win32_derive::dllexport]
pub fn GetFileTime(
    machine: &mut Machine,
    hFile: HFILE,
    lpCreationTime: Option<&mut FILETIME>,
    lpLastAccessTime: Option<&mut FILETIME>,
    lpLastWriteTime: Option<&mut FILETIME>,
) -> bool {
    let file = match machine.state.kernel32.files.get(hFile) {
        Some(f) => f,
        None => {
            log::debug!("GetFileTime({hFile:?}) unknown handle");
            set_last_error(machine, ERROR_INVALID_HANDLE);
            return false;
        }
    };

    let stat = match file.stat() {
        Ok(stat) => stat,
        Err(code) => {
            log::debug!("GetFileTime({hFile:?}) failed: {}", win32_error_str(code));
            set_last_error(machine, code);
            return false;
        }
    };

    if let Some(time) = lpCreationTime {
        *time = FILETIME::from_unix_nanos(stat.ctime);
    }
    if let Some(time) = lpLastAccessTime {
        *time = FILETIME::from_unix_nanos(stat.atime);
    }
    if let Some(time) = lpLastWriteTime {
        *time = FILETIME::from_unix_nanos(stat.mtime);
    }

    set_last_error(machine, ERROR_SUCCESS);
    true
}
