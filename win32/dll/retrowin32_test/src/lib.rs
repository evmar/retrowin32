//! Definition of a "retrowin32_test" builtin dll, used for testing retrowin32.
//! See win32/lib/README.md.

mod builtin;

pub use builtin::DLL;

use win32_system::System;

#[win32_derive::dllexport]
pub async fn retrowin32_test_callback1(sys: &mut dyn System, func: u32, data: u32) -> u32 {
    sys.call_x86(func, vec![data]).await;
    1
}
