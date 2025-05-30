use super::{SECURITY_ATTRIBUTES, file::HFILE};
use win32_system::System;

#[win32_derive::dllexport]
pub fn CreatePipe(
    sys: &dyn System,
    hReadPipe: Option<&mut HFILE>,
    hWritePipe: Option<&mut HFILE>,
    lpPipeAttributes: Option<&mut SECURITY_ATTRIBUTES>,
    nSize: u32,
) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn PeekNamedPipe(
    sys: &dyn System,
    hNamedPipe: HFILE,
    lpBuffer: Option<&mut u32>, // TODO
    nBufferSize: u32,
    lpBytesRead: Option<&mut u32>,
    lpTotalBytesAvail: Option<&mut u32>,
    lpBytesLeftThisMessage: Option<&mut u32>,
) -> bool {
    todo!()
}
