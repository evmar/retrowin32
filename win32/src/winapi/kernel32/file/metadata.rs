use crate::{
    Machine,
    winapi::{
        ERROR,
        kernel32::{FILETIME, STDERR_HFILE, STDIN_HFILE, STDOUT_HFILE, set_last_error},
    },
};
use bitflags::bitflags;
use win32_system::{System, host};
use win32_winapi::{Str16, WindowsPath, calling_convention::ABIReturn};

use super::HFILE;

// https://learn.microsoft.com/en-us/windows/win32/fileio/file-attribute-constants
bitflags! {
    #[derive(Copy, Clone, Debug, win32_derive::TryFromBitflags)]
    pub struct FileAttribute: u32 {
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
impl FileAttribute {
    pub fn invalid() -> FileAttribute {
        // INVALID_FILE_ATTRIBUTES is not part of the enum
        FileAttribute::from_bits_retain(u32::MAX)
    }
}

impl From<&host::Stat> for FileAttribute {
    fn from(stat: &host::Stat) -> Self {
        let mut attr = FileAttribute::empty();
        match stat.kind {
            host::StatKind::File => {
                attr |= FileAttribute::NORMAL;
            }
            host::StatKind::Directory => {
                attr |= FileAttribute::DIRECTORY;
            }
            host::StatKind::Symlink => {
                attr |= FileAttribute::REPARSE_POINT;
            }
        }
        attr
    }
}

impl Into<ABIReturn> for FileAttribute {
    fn into(self) -> ABIReturn {
        (self.bits() as u32).into()
    }
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
impl From<&host::Stat> for BY_HANDLE_FILE_INFORMATION {
    fn from(stat: &host::Stat) -> Self {
        Self {
            dwFileAttributes: FileAttribute::from(stat).bits(),
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
            set_last_error(machine, ERROR::INVALID_DATA);
            return false;
        }
    };

    let stat = match file.stat() {
        Ok(stat) => stat,
        Err(err) => {
            log::debug!("GetFileInformationByHandle({hFile:?}) failed: {err:?}",);
            set_last_error(machine, err);
            return false;
        }
    };

    if let Some(info) = lpFileInformation {
        *info = BY_HANDLE_FILE_INFORMATION::from(&stat);
    }

    set_last_error(machine, ERROR::SUCCESS);
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
        set_last_error(machine, ERROR::INVALID_HANDLE);
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
            set_last_error(machine, ERROR::from(err));
            return u32::MAX;
        }
    };
    if let Some(high) = lpDistanceToMoveHigh {
        *high = (pos >> 32) as i32;
    } else if pos >> 32 != 0 {
        log::debug!("SetFilePointer({hFile:?}) overflow");
        set_last_error(machine, ERROR::INVALID_DATA);
        return u32::MAX;
    }
    set_last_error(machine, ERROR::SUCCESS);
    pos as u32
}

#[win32_derive::dllexport]
pub fn GetFileAttributesW(sys: &dyn System, lpFileName: Option<&Str16>) -> u32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn GetFileAttributesA(machine: &mut Machine, lpFileName: Option<&str>) -> FileAttribute {
    let Some(file_name) = lpFileName else {
        log::debug!("CreateFileA failed: null lpFileName");
        set_last_error(machine, ERROR::INVALID_DATA);
        return FileAttribute::invalid();
    };

    let path = WindowsPath::new(file_name);
    let stat = match machine.host.stat(path) {
        Ok(stat) => stat,
        Err(err) => {
            log::debug!("GetFileAttributesA({file_name:?}) failed: {err:?}",);
            set_last_error(machine, err);
            return FileAttribute::invalid();
        }
    };

    set_last_error(machine, ERROR::SUCCESS);

    match stat.kind {
        host::StatKind::File => FileAttribute::NORMAL,
        host::StatKind::Directory => FileAttribute::DIRECTORY,
        host::StatKind::Symlink => FileAttribute::REPARSE_POINT,
    }
}

#[win32_derive::dllexport]
pub fn GetFileSize(machine: &mut Machine, hFile: HFILE, lpFileSizeHigh: Option<&mut u32>) -> u32 {
    let file = match machine.state.kernel32.files.get(hFile) {
        Some(f) => f,
        None => {
            log::debug!("GetFileSize({hFile:?}) unknown handle");
            set_last_error(machine, ERROR::INVALID_HANDLE);
            return u32::MAX;
        }
    };

    let stat = match file.stat() {
        Ok(stat) => stat,
        Err(err) => {
            log::debug!("GetFileSize({hFile:?}) failed: {err:?}");
            set_last_error(machine, err);
            return u32::MAX;
        }
    };

    set_last_error(machine, ERROR::SUCCESS);

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
            set_last_error(machine, ERROR::INVALID_HANDLE);
            return false;
        }
    };

    let stat = match file.stat() {
        Ok(stat) => stat,
        Err(error) => {
            log::debug!("GetFileTime({hFile:?}) failed: {error:?}");
            set_last_error(machine, error);
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

    set_last_error(machine, ERROR::SUCCESS);
    true
}

#[win32_derive::dllexport]
pub fn SetFileAttributesW(
    sys: &dyn System,
    lpFileName: Option<&Str16>,
    dwFileAttributes: Result<FileAttribute, u32>,
) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn SetFileAttributesA(
    machine: &mut Machine,
    lpFileName: Option<&str>,
    dwFileAttributes: Result<FileAttribute, u32>,
) -> bool {
    let Some(file_name) = lpFileName else {
        log::debug!("SetFileAttributesA failed: null lpFileName");
        set_last_error(machine, ERROR::INVALID_DATA);
        return false;
    };
    dwFileAttributes.unwrap();

    let _ = file_name;
    log::debug!("SetFileAttributesA stub");
    true
}

#[win32_derive::dllexport]
pub fn SetFileTime(
    machine: &mut Machine,
    hFile: HFILE,
    lpCreationTime: Option<&FILETIME>,
    lpLastAccessTime: Option<&FILETIME>,
    lpLastWriteTime: Option<&FILETIME>,
) -> bool {
    let file = match machine.state.kernel32.files.get_mut(hFile) {
        Some(f) => f,
        None => {
            log::debug!("SetFileTime({hFile:?}) unknown handle");
            set_last_error(machine, ERROR::INVALID_HANDLE);
            return false;
        }
    };

    let mut stat = match file.stat() {
        Ok(stat) => stat,
        Err(error) => {
            log::debug!("SetFileTime({hFile:?}) failed: {error:?}");
            set_last_error(machine, error);
            return false;
        }
    };

    if let Some(time) = lpCreationTime {
        stat.ctime = time.to_unix_nanos();
    }
    if let Some(time) = lpLastAccessTime {
        stat.atime = time.to_unix_nanos();
    }
    if let Some(time) = lpLastWriteTime {
        stat.mtime = time.to_unix_nanos();
    }

    let _ = stat;
    log::debug!("SetFileTime stub");
    true
}
