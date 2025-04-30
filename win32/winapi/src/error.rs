// https://learn.microsoft.com/en-us/windows/win32/debug/system-error-codes--0-499-

use crate::calling_convention;

/// Windows error codes.
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, win32_derive::TryFromEnum)]
pub enum ERROR {
    SUCCESS = 0,
    FILE_NOT_FOUND = 2,
    ACCESS_DENIED = 5,
    INVALID_HANDLE = 6,
    INVALID_ACCESS = 12,
    INVALID_DATA = 13,
    OUT_OF_PAPER = 28,
    FILE_EXISTS = 80,
    OPEN_FAILED = 110,
    INSUFFICIENT_BUFFER = 122,
    MOD_NOT_FOUND = 126,
    ALREADY_EXISTS = 183,
}

impl From<std::io::Error> for ERROR {
    fn from(err: std::io::Error) -> Self {
        match err.kind() {
            std::io::ErrorKind::NotFound => ERROR::FILE_NOT_FOUND,
            std::io::ErrorKind::PermissionDenied => ERROR::ACCESS_DENIED,
            std::io::ErrorKind::InvalidData => ERROR::INVALID_DATA,
            std::io::ErrorKind::AlreadyExists => ERROR::FILE_EXISTS,
            std::io::ErrorKind::InvalidInput => ERROR::INVALID_ACCESS,
            _ => unimplemented!(),
        }
    }
}

impl From<ERROR> for u32 {
    fn from(err: ERROR) -> u32 {
        err as u32
    }
}

impl Into<calling_convention::ABIReturn> for ERROR {
    fn into(self) -> calling_convention::ABIReturn {
        (self as u32).into()
    }
}

impl Into<calling_convention::ABIReturn> for Result<ERROR, u32> {
    fn into(self) -> calling_convention::ABIReturn {
        match self {
            Ok(err) => err.into(),
            Err(err) => err.into(),
        }
    }
}
