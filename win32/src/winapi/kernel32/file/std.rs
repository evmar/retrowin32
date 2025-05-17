use super::HFILE;
use crate::System;

#[derive(Debug, win32_derive::TryFromEnum)]
pub enum STD {
    INPUT_HANDLE = -10,
    OUTPUT_HANDLE = -11,
    ERROR_HANDLE = -12,
}

// For now, a magic variable  that makes it easier to spot.
pub const STDIN_HFILE: HFILE = HFILE::from_raw(0xF11E_0100);
pub const STDOUT_HFILE: HFILE = HFILE::from_raw(0xF11E_0101);
pub const STDERR_HFILE: HFILE = HFILE::from_raw(0xF11E_0102);

#[win32_derive::dllexport]
pub fn GetStdHandle(sys: &dyn System, nStdHandle: Result<STD, i32>) -> HFILE {
    match nStdHandle {
        Ok(STD::INPUT_HANDLE) => STDIN_HFILE,
        Ok(STD::OUTPUT_HANDLE) => STDOUT_HFILE,
        Ok(STD::ERROR_HANDLE) => STDERR_HFILE,
        _ => HFILE::invalid(),
    }
}

#[win32_derive::dllexport]
pub fn SetStdHandle(sys: &dyn System, nStdHandle: Result<STD, i32>, hHandle: u32) -> bool {
    true // success
}
