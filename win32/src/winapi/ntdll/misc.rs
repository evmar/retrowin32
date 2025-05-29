use crate::winapi::kernel32;
pub use kernel32::file::HFILE;
use win32_system::System;
use win32_winapi::calling_convention::ArrayOut;

const STATUS_SUCCESS: u32 = 0;

#[repr(C)]
#[derive(Debug)]
pub struct IO_STATUS_BLOCK {
    pub Status: u32,
    pub Information: u32,
}
unsafe impl memory::Pod for IO_STATUS_BLOCK {}

#[win32_derive::dllexport]
pub fn NtReadFile(
    sys: &dyn System,
    FileHandle: HFILE,
    Event: u32,
    ApcRoutine: u32,
    ApcContext: u32,
    IoStatusBlock: Option<&mut IO_STATUS_BLOCK>,
    mut Buffer: ArrayOut<u8>,
    ByteOffset: Option<&mut u64>,
    Key: u32,
) -> u32 {
    let mut state = kernel32::file::get_state(sys);
    let file = state.files.get_mut(FileHandle).unwrap();
    if Event != 0 {
        todo!();
    }
    let status_block = IoStatusBlock.unwrap();
    let buf = &mut Buffer;
    if ByteOffset.is_some() {
        todo!();
    }

    let len = file.read(buf).unwrap();
    *status_block = IO_STATUS_BLOCK {
        Status: STATUS_SUCCESS,
        Information: len as u32,
    };
    STATUS_SUCCESS
}

#[win32_derive::dllexport]
pub fn NtCurrentTeb(sys: &dyn System) -> u32 {
    sys.teb_addr()
}

#[win32_derive::dllexport]
pub fn RtlExitUserProcess(sys: &mut dyn System, exit_code: u32) {
    sys.exit(exit_code);
}
