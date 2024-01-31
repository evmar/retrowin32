//! Definition of a "retrowin32_test" builtin dll, used for testing retrowin32.
//! See win32/lib/README.md.

use crate::Machine;

const TRACE_CONTEXT: &'static str = "retrowin32";

#[win32_derive::dllexport]
pub async fn retrowin32_test_callback1(machine: &mut Machine, func: u32, data: u32) -> u32 {
    machine.call_x86(func, vec![data]).await;
    1
}
