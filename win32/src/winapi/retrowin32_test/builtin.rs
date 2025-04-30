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
        system::System,
        winapi::{self, *},
    };
    use ::memory::Extensions;
    use winapi::retrowin32_test::*;
    pub unsafe fn retrowin32_test_callback1(
        sys: &mut dyn System,
        stack_args: u32,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ABIReturn>>> {
        unsafe {
            let mem = sys.mem().detach();
            let func = <u32>::from_stack(mem, stack_args + 0u32);
            let data = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if crate::winapi::trace::enabled("retrowin32_test") {
                crate::winapi::trace::Record::new(
                    winapi::retrowin32_test::retrowin32_test_callback1_pos,
                    "retrowin32_test",
                    "retrowin32_test_callback1",
                    &[("func", &func), ("data", &data)],
                )
                .enter()
            } else {
                None
            };
            let machine: *mut Machine = sys.machine();
            Box::pin(async move {
                let machine = &mut *machine;
                let result =
                    winapi::retrowin32_test::retrowin32_test_callback1(machine, func, data).await;
                if let Some(mut __trace_record) = __trace_record {
                    __trace_record.exit(&result);
                }
                result.into()
            })
        }
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
