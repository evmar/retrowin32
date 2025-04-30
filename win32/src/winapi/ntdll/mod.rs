#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

mod builtin;

pub use builtin::DLL;

use crate::{Machine, calling_convention::ArrayOut, winapi::HFILE};

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
    machine: &mut Machine,
    FileHandle: HFILE,
    Event: u32,
    ApcRoutine: u32,
    ApcContext: u32,
    IoStatusBlock: Option<&mut IO_STATUS_BLOCK>,
    mut Buffer: ArrayOut<u8>,
    ByteOffset: Option<&mut u64>,
    Key: u32,
) -> u32 {
    let file = machine.state.kernel32.files.get_mut(FileHandle).unwrap();
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
pub fn NtCurrentTeb(machine: &mut Machine) -> u32 {
    machine.teb_addr()
}

#[win32_derive::dllexport]
pub fn RtlExitUserProcess(machine: &mut Machine, exit_code: u32) {
    machine.exit(exit_code);
}
