use super::FileAttribute;
use crate::FILETIME;
use win32_system::{System, generic_get_state, host};
use win32_winapi::{DWORD, ERROR, HANDLE, Handles, Str16, WindowsPath};

use super::HFILE;
use super::path::{MAX_PATH, normalize_dos_path};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct HFINDT;
pub type HFIND = HANDLE<HFINDT>;

type State = Handles<HFIND, FindHandle>;

#[inline]
pub fn get_state(sys: &dyn System) -> std::cell::RefMut<State> {
    generic_get_state::<State>(sys)
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

impl From<&host::ReadDirEntry> for WIN32_FIND_DATAA {
    fn from(file: &host::ReadDirEntry) -> Self {
        let stat = &file.stat;
        let mut data = Self {
            dwFileAttributes: FileAttribute::from(stat).bits(),
            ftCreationTime: FILETIME::from_unix_nanos(stat.ctime),
            ftLastAccessTime: FILETIME::from_unix_nanos(stat.atime),
            ftLastWriteTime: FILETIME::from_unix_nanos(stat.mtime),
            nFileSizeHigh: (stat.size >> 32) as DWORD,
            nFileSizeLow: stat.size as DWORD,
            dwReserved0: if stat.kind == host::StatKind::Symlink {
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

pub struct FindHandle {
    pub pattern: String,
    pub read_dir: Box<dyn host::ReadDir>,
}

pub type WIN32_FIND_DATAW = WIN32_FIND_DATAA; // TODO

#[win32_derive::dllexport]
pub fn FindFirstFileW(
    sys: &dyn System,
    lpFileName: Option<&Str16>,
    lpFindFileData: Option<&mut WIN32_FIND_DATAW>,
) -> HFIND {
    todo!()
}

#[win32_derive::dllexport]
pub fn FindFirstFileA(
    sys: &dyn System,
    lpFileName: Option<&str>,
    lpFindFileData: Option<&mut WIN32_FIND_DATAA>,
) -> HFIND {
    let Some(file_name) = lpFileName else {
        log::debug!("FindFirstFileA failed: null lpFileName");
        sys.set_last_error(ERROR::INVALID_DATA);
        return HFIND::invalid();
    };
    let file_name = normalize_dos_path(file_name);

    let path = WindowsPath::new(file_name);
    let parent = match path.parent() {
        // Empty path means a filename with no parent directory, apparently(?).
        Some(parent) if parent.as_bytes().len() == 0 => WindowsPath::new("."),
        // None means a path with no parent, e.g. root directory.
        None => WindowsPath::new("."),
        Some(parent) => parent,
    };
    let Some(pattern) = path.file_name() else {
        log::debug!("FindFirstFileA({file_name:?}) no file name");
        sys.set_last_error(ERROR::INVALID_DATA);
        return HFIND::invalid();
    };
    let mut pattern = match String::from_utf8(pattern.to_vec()) {
        Ok(value) => value,
        Err(e) => {
            log::debug!("FindFirstFileA({file_name:?}) invalid file name: {:?}", e);
            sys.set_last_error(ERROR::INVALID_DATA);
            return HFIND::invalid();
        }
    };
    if pattern == "." {
        pattern = "*".to_string();
    }

    let mut read_dir = match sys.host().read_dir(parent) {
        Ok(handle) => handle,
        Err(err) => {
            log::debug!("FindFirstFileA({file_name:?}) failed: {err:?}",);
            sys.set_last_error(err);
            return HFIND::invalid();
        }
    };

    let next = loop {
        match read_dir.next() {
            Ok(Some(entry)) => {
                if glob_match(&entry.name, &pattern) {
                    break entry;
                }
            }
            Ok(None) => {
                log::debug!("FindFirstFileA({file_name:?}) not found");
                sys.set_last_error(ERROR::FILE_NOT_FOUND);
                return HFIND::invalid();
            }
            Err(err) => {
                log::debug!("FindFirstFileA({file_name:?}) failed: {err:?}",);
                sys.set_last_error(err);
                return HFIND::invalid();
            }
        }
    };

    if let Some(data) = lpFindFileData {
        *data = WIN32_FIND_DATAA::from(&next);
    }

    sys.set_last_error(ERROR::SUCCESS);
    get_state(sys).add(FindHandle { pattern, read_dir })
}

#[win32_derive::dllexport]
pub fn FindNextFileW(
    sys: &dyn System,
    hFindFile: HFILE,
    lpFindFileData: Option<&mut WIN32_FIND_DATAW>,
) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn FindNextFileA(
    sys: &dyn System,
    hFindFile: HFIND,
    lpFindFileData: Option<&mut WIN32_FIND_DATAA>,
) -> bool {
    let mut state = get_state(sys);
    let handle = match state.get_mut(hFindFile) {
        Some(handle) => handle,
        None => {
            log::debug!("FindNextFileA({hFindFile:?}) unknown handle");
            sys.set_last_error(ERROR::INVALID_HANDLE);
            return false;
        }
    };

    let next = loop {
        match handle.read_dir.next() {
            Ok(Some(entry)) => {
                if glob_match(&entry.name, &handle.pattern) {
                    break entry;
                }
            }
            Ok(None) => {
                sys.set_last_error(ERROR::FILE_NOT_FOUND);
                return false;
            }
            Err(err) => {
                log::debug!("FindNextFileA({hFindFile:?}) failed: {err:?}",);
                sys.set_last_error(err);
                return false;
            }
        };
    };

    if let Some(data) = lpFindFileData {
        *data = WIN32_FIND_DATAA::from(&next);
    }

    sys.set_last_error(ERROR::SUCCESS);
    true
}

#[win32_derive::dllexport]
pub fn FindClose(sys: &dyn System, hFindFile: HFIND) -> bool {
    if get_state(sys).remove(hFindFile).is_none() {
        log::debug!("FindClose({hFindFile:?}): unknown handle");
        sys.set_last_error(ERROR::INVALID_HANDLE);
        return false;
    }

    sys.set_last_error(ERROR::SUCCESS);
    true
}

/// Matches a string against a glob pattern with `*` and `?` wildcards.
/// The pattern is case-insensitive. Used by `FindFirstFileA` and `FindNextFileA`.
fn glob_match(input: &str, pattern: &str) -> bool {
    let mut input = input.chars();
    let mut pattern = pattern.chars();
    loop {
        match (input.next(), pattern.next()) {
            (Some(_), Some('*')) => {
                if let Some(p) = pattern.next() {
                    while let Some(c) = input.next() {
                        if c.eq_ignore_ascii_case(&p) {
                            break;
                        }
                    }
                } else {
                    return true;
                }
            }
            (Some(c), Some('?')) => {
                if c == '\\' {
                    return false;
                }
            }
            (Some(c), Some(p)) => {
                if !c.eq_ignore_ascii_case(&p) {
                    return false;
                }
            }
            (None, Some('*')) => {
                return pattern.all(|c| c == '*');
            }
            (None, None) => {
                return true;
            }
            _ => {
                return false;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_glob_match() {
        assert!(glob_match("foo.txt", "*"));
        assert!(glob_match("foo.txt", "foo.txt"));
        assert!(glob_match("foo.txt", "foo.*"));
        assert!(glob_match("Foo.", "foo.*"));
        assert!(glob_match("foo.txt", "*.txt"));
        assert!(glob_match("foo.txt", "foo.???"));
        assert!(!glob_match("foo.txt", "foo.??"));
        assert!(!glob_match("foo.txt", "foo"));
        assert!(glob_match("FOO.txt", "foo.txt"));
    }
}
