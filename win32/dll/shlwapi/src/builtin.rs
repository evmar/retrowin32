#![doc = r" Generated code, do not edit.  See winapi/builtin.rs for an overview."]
#![allow(unused_imports)]
#![allow(unused_variables)]
use win32_system::dll::*;
mod wrappers {
    use crate as shlwapi;
    use crate::*;
    use ::memory::Extensions;
    use win32_system::{System, trace};
    use win32_winapi::{calling_convention::*, *};
    pub unsafe fn PathRemoveFileSpecA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let pszPath = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("src/lib") {
                trace::Record::new(
                    shlwapi::PathRemoveFileSpecA_pos,
                    "src/lib",
                    "PathRemoveFileSpecA",
                    &[("pszPath", &pszPath)],
                )
                .enter()
            } else {
                None
            };
            let result = shlwapi::PathRemoveFileSpecA(sys, pszPath);
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
    raw: std::include_bytes!("../shlwapi.dll"),
};
