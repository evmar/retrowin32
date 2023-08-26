//! Definition of a "retrowin32" builtin dll, used for testing retrowin32.
//! See win32/lib/README.md.

use crate::Machine;

const TRACE_CONTEXT: &'static str = "retrowin32";

#[win32_derive::dllexport]
pub async fn retrowin32_callback1(machine: &mut Machine, func: u32, data: u32) -> u32 {
    crate::shims::call_x86(machine, func, vec![data]).await;
    1
}
