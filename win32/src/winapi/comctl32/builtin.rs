#![doc = r" Generated code, do not edit.  See winapi/builtin.rs for an overview."]
#![allow(unused_imports)]
#![allow(unused_variables)]
use crate::{
    shims::{Handler, Shim},
    winapi::builtin::BuiltinDLL,
};
mod wrappers {
    use crate::{
        calling_convention::*,
        machine::Machine,
        winapi::{self, *},
    };
    use ::memory::Extensions;
    use winapi::comctl32::*;
    pub unsafe fn InitCommonControls(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let __trace_record = if crate::winapi::trace::enabled("comctl32") {
            crate::winapi::trace::Record::new(
                winapi::comctl32::InitCommonControls_pos,
                "comctl32",
                "InitCommonControls",
                &[],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::comctl32::InitCommonControls(machine);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
}
const SHIMS: [Shim; 1usize] = [Shim {
    name: "InitCommonControls",
    func: Handler::Sync(wrappers::InitCommonControls),
}];
pub const DLL: BuiltinDLL = BuiltinDLL {
    file_name: "comctl32.dll",
    shims: &SHIMS,
    raw: std::include_bytes!("../../../dll/comctl32.dll"),
};
