use crate::machine::Machine;

const TRACE_CONTEXT: &'static str = "vcruntime140";

#[win32_derive::dllexport(cdecl)]
pub fn memcpy(machine: &mut Machine, dst: u32, src: u32, len: u32) -> u32 {
    // TODO: this probably violates Rust rules around aliasing, if callers expect memmove.
    machine
        .mem()
        .sub(dst, len)
        .as_mut_slice_todo()
        .copy_from_slice(machine.mem().sub(src, len).as_slice_todo());
    dst
}
