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
    pub unsafe fn GetFileVersionInfoSizeA(machine: &mut Machine, stack_args: u32) -> u64 {
        let mem = machine.mem().detach();
        let lptstrFilename = <Option<&str>>::from_stack(mem, stack_args + 0u32);
        let lpdwHandle = <Option<&mut u32>>::from_stack(mem, stack_args + 4u32);
        let __trace_record = if crate::trace::enabled("version") {
            crate::trace::Record::new(
                winapi::version::GetFileVersionInfoSizeA_pos,
                "version",
                "GetFileVersionInfoSizeA",
                &[
                    ("lptstrFilename", &lptstrFilename),
                    ("lpdwHandle", &lpdwHandle),
                ],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::version::GetFileVersionInfoSizeA(machine, lptstrFilename, lpdwHandle);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.to_abireturn()
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
