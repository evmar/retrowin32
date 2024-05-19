#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use crate::{
    machine::Machine,
    winapi::{stack_args::ArrayWithSizeMut, types::HFILE},
};

const TRACE_CONTEXT: &'static str = "ntdll";

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
    Buffer: ArrayWithSizeMut<u8>,
    ByteOffset: Option<&mut u64>,
    Key: u32,
) -> u32 {
    let file = machine.state.kernel32.files.get_mut(&FileHandle).unwrap();
    if Event != 0 {
        todo!();
    }
    let status_block = IoStatusBlock.unwrap();
    let buf = Buffer.unwrap();
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
