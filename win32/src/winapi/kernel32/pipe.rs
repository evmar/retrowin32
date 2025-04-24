use super::SECURITY_ATTRIBUTES;
use crate::{Machine, winapi::HFILE};

#[win32_derive::dllexport]
pub fn CreatePipe(
    _machine: &mut Machine,
    hReadPipe: Option<&mut HFILE>,
    hWritePipe: Option<&mut HFILE>,
    lpPipeAttributes: Option<&mut SECURITY_ATTRIBUTES>,
    nSize: u32,
) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn PeekNamedPipe(
    _machine: &mut Machine,
    hNamedPipe: HFILE,
    lpBuffer: Option<&mut u32>, // TODO
    nBufferSize: u32,
    lpBytesRead: Option<&mut u32>,
    lpTotalBytesAvail: Option<&mut u32>,
    lpBytesLeftThisMessage: Option<&mut u32>,
) -> bool {
    todo!()
}
