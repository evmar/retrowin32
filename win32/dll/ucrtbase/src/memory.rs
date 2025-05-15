use memory::ExtensionsMut;
use win32_system::System;
use win32_winapi::CStr;

#[win32_derive::dllexport(cdecl)]
pub fn malloc(sys: &mut dyn System, size: u32) -> u32 {
    let memory = sys.memory();
    memory.process_heap.alloc(memory.mem(), size)
}

#[win32_derive::dllexport(cdecl)]
pub fn calloc(sys: &mut dyn System, count: u32, size: u32) -> u32 {
    let memory = sys.memory();
    memory.process_heap.alloc(memory.mem(), count * size)
}

#[win32_derive::dllexport(cdecl)]
pub fn free(sys: &mut dyn System, ptr: u32) -> u32 {
    let memory = sys.memory();
    memory.process_heap.free(memory.mem(), ptr);
    0
}

#[win32_derive::dllexport(cdecl)]
pub fn memset(sys: &mut dyn System, dst: u32, val: u32, len: u32) -> u32 {
    sys.mem().sub32_mut(dst, len).fill(val as u8);
    dst
}

#[win32_derive::dllexport(cdecl)]
pub fn strlen(sys: &mut dyn System, lpString: Option<&CStr>) -> u32 {
    // The mapping to str already computes the string length.
    lpString.unwrap().count_bytes() as u32
}

#[win32_derive::dllexport(cdecl)]
pub fn memcpy(sys: &mut dyn System, dest: u32, src: u32, count: u32) -> u32 {
    sys.mem().copy(src, dest, count);
    dest
}

#[win32_derive::dllexport(cdecl, symbol = "??2@YAPAXI@Z")]
pub fn operator_new(sys: &mut dyn System, size: u32) -> u32 {
    let memory = sys.memory();
    memory.process_heap.alloc(memory.mem(), size)
}

#[win32_derive::dllexport(cdecl, symbol = "??3@YAXPAX@Z")]
pub fn operator_delete(sys: &mut dyn System, size: u32) {
    todo!()
}

#[win32_derive::dllexport(cdecl)]
pub fn __CxxFrameHandler(
    sys: &mut dyn System,
    pExcept: u32,
    pRN: u32,
    pContext: u32,
    pDC: u32,
) -> u32 {
    todo!()
}

#[win32_derive::dllexport(cdecl)]
pub fn _EH_prolog(sys: &mut dyn System) {
    // TODO: exception handler setup
}
