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
    use winapi::dinput::*;
    pub unsafe fn DirectInputCreateA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let version = <u32>::from_stack(mem, stack_args + 0u32);
        let ppDI = <Option<&mut u32>>::from_stack(mem, stack_args + 4u32);
        let pUnkOuter = <u32>::from_stack(mem, stack_args + 8u32);
        let __trace_context = if crate::trace::enabled("dinput") {
            Some(crate::trace::trace_begin(
                "dinput",
                "DirectInputCreateA",
                &[
                    ("version", &version),
                    ("ppDI", &ppDI),
                    ("pUnkOuter", &pUnkOuter),
                ],
            ))
        } else {
            None
        };
        let result = winapi::dinput::DirectInputCreateA(machine, version, ppDI, pUnkOuter);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::dinput::DirectInputCreateA_pos.0,
                winapi::dinput::DirectInputCreateA_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
}
const SHIMS: [Shim; 1usize] = [Shim {
    name: "DirectInputCreateA",
    func: Handler::Sync(wrappers::DirectInputCreateA),
}];
pub const DLL: BuiltinDLL = BuiltinDLL {
    file_name: "dinput.dll",
    shims: &SHIMS,
    raw: std::include_bytes!("../../../dll/dinput.dll"),
};
