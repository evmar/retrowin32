use memory::ExtensionsMut;

use crate::Machine;

#[win32_derive::dllexport(cdecl)]
pub fn malloc(machine: &mut Machine, size: u32) -> u32 {
    machine
        .state
        .kernel32
        .process_heap
        .borrow_mut()
        .alloc(machine.memory.mem(), size)
}

#[win32_derive::dllexport(cdecl)]
pub fn calloc(machine: &mut Machine, count: u32, size: u32) -> u32 {
    machine
        .state
        .kernel32
        .process_heap
        .borrow_mut()
        .alloc(machine.memory.mem(), count * size)
}

#[win32_derive::dllexport(cdecl)]
pub fn free(machine: &mut Machine, ptr: u32) -> u32 {
    machine
        .state
        .kernel32
        .process_heap
        .borrow_mut()
        .free(machine.memory.mem(), ptr);
    0
}

#[win32_derive::dllexport(cdecl)]
pub fn memset(machine: &mut Machine, dst: u32, val: u32, len: u32) -> u32 {
    machine.mem().sub32_mut(dst, len).fill(val as u8);
    dst
}

#[win32_derive::dllexport(cdecl, symbol = "??2@YAPAXI@Z")]
pub fn operator_new(machine: &mut Machine, size: u32) -> u32 {
    machine
        .state
        .kernel32
        .process_heap
        .borrow_mut()
        .alloc(machine.memory.mem(), size)
}

#[win32_derive::dllexport(cdecl, symbol = "??3@YAXPAX@Z")]
pub fn operator_delete(machine: &mut Machine, size: u32) {
    todo!()
}

#[win32_derive::dllexport(cdecl)]
pub fn __CxxFrameHandler(
    machine: &mut Machine,
    pExcept: u32,
    pRN: u32,
    pContext: u32,
    pDC: u32,
) -> u32 {
    todo!()
}

#[win32_derive::dllexport(cdecl)]
pub fn _EH_prolog(machine: &mut Machine) {
    // TODO: exception handler setup
}
