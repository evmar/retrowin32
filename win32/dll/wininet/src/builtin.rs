#![doc = r" Generated code, do not edit.  See winapi/builtin.rs for an overview."]
#![allow(unused_imports)]
#![allow(unused_variables)]
use win32_system::dll::*;
mod wrappers {
    use crate as wininet;
    use crate::*;
    use ::memory::Extensions;
    use win32_system::{System, trace};
    use win32_winapi::{calling_convention::*, *};
    pub unsafe fn InternetOpenA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpszAgent = <Option<&str>>::from_stack(mem, stack_args + 0u32);
            let dwAccessType = <u32>::from_stack(mem, stack_args + 4u32);
            let lpszProxy = <Option<&str>>::from_stack(mem, stack_args + 8u32);
            let lpszProxyBypass = <Option<&str>>::from_stack(mem, stack_args + 12u32);
            let dwFlags = <u32>::from_stack(mem, stack_args + 16u32);
            let __trace_record = if trace::enabled("src/lib") {
                trace::Record::new(
                    wininet::InternetOpenA_pos,
                    "src/lib",
                    "InternetOpenA",
                    &[
                        ("lpszAgent", &lpszAgent),
                        ("dwAccessType", &dwAccessType),
                        ("lpszProxy", &lpszProxy),
                        ("lpszProxyBypass", &lpszProxyBypass),
                        ("dwFlags", &dwFlags),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = wininet::InternetOpenA(
                sys,
                lpszAgent,
                dwAccessType,
                lpszProxy,
                lpszProxyBypass,
                dwFlags,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
}
const SHIMS: [Shim; 1usize] = [Shim {
    name: "InternetOpenA",
    func: Handler::Sync(wrappers::InternetOpenA),
}];
pub const DLL: BuiltinDLL = BuiltinDLL {
    file_name: "wininet.dll",
    shims: &SHIMS,
    raw: std::include_bytes!("../wininet.dll"),
};
