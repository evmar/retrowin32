// https://learn.microsoft.com/en-us/windows/win32/debug/system-error-codes--0-499-

/// Windows error codes.
#[allow(non_camel_case_types)]
#[derive(Debug, win32_derive::TryFromEnum)]
pub enum ERROR {
    SUCCESS = 0,
    FILE_NOT_FOUND = 2,
    ACCESS_DENIED = 5,
    INVALID_HANDLE = 6,
    INVALID_ACCESS = 12,
    INVALID_DATA = 13,
    FILE_EXISTS = 80,
    OPEN_FAILED = 110,
    MOD_NOT_FOUND = 126,
    ALREADY_EXISTS = 183,
}

impl ERROR {
    pub fn from_io_error(err: &std::io::Error) -> ERROR {
        match err.kind() {
            std::io::ErrorKind::NotFound => ERROR::FILE_NOT_FOUND,
            std::io::ErrorKind::PermissionDenied => ERROR::ACCESS_DENIED,
            std::io::ErrorKind::InvalidData => ERROR::INVALID_DATA,
            std::io::ErrorKind::AlreadyExists => ERROR::FILE_EXISTS,
            std::io::ErrorKind::InvalidInput => ERROR::INVALID_ACCESS,
            _ => ERROR::OPEN_FAILED,
        }
    }
}

impl From<ERROR> for u32 {
    fn from(err: ERROR) -> u32 {
        err as u32
    }
}
