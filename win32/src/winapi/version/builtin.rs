#![doc = r" Generated code, do not edit.  See winapi/builtin.rs for an overview."]
#![allow(unused_imports)]
#![allow(unused_variables)]
use crate::{
    shims::{Handler, Shim},
    winapi::builtin::BuiltinDLL,
};
mod wrappers {
    use crate::{
        machine::Machine,
        winapi::{self, stack_args::*, types::*},
    };
    use ::memory::Extensions;
    use winapi::version::*;
    pub unsafe fn GetFileVersionInfoSizeA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lptstrFilename = <Option<&str>>::from_stack(mem, stack_args + 0u32);
        let lpdwHandle = <Option<&mut u32>>::from_stack(mem, stack_args + 4u32);
        let __trace_context = if crate::trace::enabled("version") {
            Some(crate::trace::trace_begin(
                "version",
                "GetFileVersionInfoSizeA",
                &[
                    ("lptstrFilename", &lptstrFilename),
                    ("lpdwHandle", &lpdwHandle),
                ],
            ))
        } else {
            None
        };
        let result = winapi::version::GetFileVersionInfoSizeA(machine, lptstrFilename, lpdwHandle);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::version::GetFileVersionInfoSizeA_pos.0,
                winapi::version::GetFileVersionInfoSizeA_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
}
const SHIMS: [Shim; 1usize] = [Shim {
    name: "GetFileVersionInfoSizeA",
    func: Handler::Sync(wrappers::GetFileVersionInfoSizeA),
}];
pub const DLL: BuiltinDLL = BuiltinDLL {
    file_name: "version.dll",
    shims: &SHIMS,
    raw: std::include_bytes!("../../../dll/version.dll"),
};
