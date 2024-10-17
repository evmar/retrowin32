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
    use winapi::retrowin32_test::*;
    pub unsafe fn retrowin32_test_callback1(
        machine: &mut Machine,
        stack_args: u32,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
        let mem = machine.mem().detach();
        let func = <u32>::from_stack(mem, stack_args + 0u32);
        let data = <u32>::from_stack(mem, stack_args + 4u32);
        let __trace_context = if crate::trace::enabled("retrowin32_test") {
            Some(crate::trace::trace_begin(
                "retrowin32_test",
                "retrowin32_test_callback1",
                &[("func", &func), ("data", &data)],
            ))
        } else {
            None
        };
        let machine: *mut Machine = machine;
        Box::pin(async move {
            let machine = unsafe { &mut *machine };
            let result =
                winapi::retrowin32_test::retrowin32_test_callback1(machine, func, data).await;
            if let Some(__trace_context) = __trace_context {
                crate::trace::trace_return(
                    &__trace_context,
                    winapi::retrowin32_test::retrowin32_test_callback1_pos.0,
                    winapi::retrowin32_test::retrowin32_test_callback1_pos.1,
                    &result,
                );
            }
            result.to_raw()
        })
    }
}
const SHIMS: [Shim; 1usize] = [Shim {
    name: "retrowin32_test_callback1",
    func: Handler::Async(wrappers::retrowin32_test_callback1),
}];
pub const DLL: BuiltinDLL = BuiltinDLL {
    file_name: "retrowin32_test.dll",
    shims: &SHIMS,
    raw: std::include_bytes!("../../../dll/retrowin32_test.dll"),
};