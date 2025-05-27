#![doc = r" Generated code, do not edit.  See winapi/builtin.rs for an overview."]
#![allow(unused_imports)]
#![allow(unused_variables)]
use win32_system::dll::*;
mod wrappers {
    use crate as retrowin32_test;
    use crate::*;
    use ::memory::Extensions;
    use win32_system::{System, trace};
    use win32_winapi::{calling_convention::*, *};
    pub unsafe fn retrowin32_test_callback1(
        sys: &mut dyn System,
        stack_args: u32,
    ) -> ::std::pin::Pin<Box<dyn ::std::future::Future<Output = ABIReturn> + '_>> {
        use retrowin32_test::*;
        unsafe {
            let mem = sys.mem().detach();
            let func = <u32>::from_stack(mem, stack_args + 0u32);
            let data = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("retrowin32_test") {
                trace::Record::new(
                    retrowin32_test::retrowin32_test_callback1_pos,
                    "retrowin32_test",
                    "retrowin32_test_callback1",
                    &[("func", &func), ("data", &data)],
                )
                .enter()
            } else {
                None
            };
            let sys = sys as *mut dyn System;
            Box::pin(async move {
                let sys = &mut *sys;
                let result = retrowin32_test::retrowin32_test_callback1(sys, func, data).await;
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
    raw: std::include_bytes!("../retrowin32_test.dll"),
};
