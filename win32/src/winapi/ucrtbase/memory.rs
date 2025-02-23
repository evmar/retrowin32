use crate::Machine;

#[win32_derive::dllexport(cdecl)]
pub fn malloc(machine: &mut Machine, size: u32) -> u32 {
    machine
        .state
        .kernel32
        .process_heap
        .alloc(machine.memory.mem(), size)
}

#[win32_derive::dllexport(cdecl)]
pub fn free(machine: &mut Machine, ptr: u32) -> u32 {
    machine
        .state
        .kernel32
        .process_heap
        .free(machine.memory.mem(), ptr);
    0
}
