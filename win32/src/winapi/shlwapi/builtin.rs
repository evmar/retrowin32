#![doc = r" Generated code, do not edit.  See winapi/builtin.rs for an overview."]
#![allow(unused_imports)]
#![allow(unused_variables)]
use win32_system::dll::*;
mod wrappers {
    use crate::{
        System,
        calling_convention::*,
        machine::Machine,
        winapi::{self, *},
    };
    use ::memory::Extensions;
    use win32_system::trace;
    use winapi::shlwapi::*;
    pub unsafe fn PathRemoveFileSpecA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let pszPath = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("shlwapi") {
                trace::Record::new(
                    winapi::shlwapi::PathRemoveFileSpecA_pos,
                    "shlwapi",
                    "PathRemoveFileSpecA",
                    &[("pszPath", &pszPath)],
                )
                .enter()
            } else {
                None
            };
            let result = winapi::shlwapi::PathRemoveFileSpecA(sys, pszPath);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
}
const SHIMS: [Shim; 1usize] = [Shim {
    name: "PathRemoveFileSpecA",
    func: Handler::Sync(wrappers::PathRemoveFileSpecA),
}];
pub const DLL: BuiltinDLL = BuiltinDLL {
    file_name: "shlwapi.dll",
    shims: &SHIMS,
    raw: std::include_bytes!("../../../dll/shlwapi.dll"),
};
