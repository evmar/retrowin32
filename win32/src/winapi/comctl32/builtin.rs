#![doc = r" Generated code, do not edit.  See winapi/builtin.rs for an overview."]
#![allow(unused_imports)]
#![allow(unused_variables)]
use win32_system::dll::*;
mod wrappers {
    use crate::winapi::comctl32::{self, *};
    use ::memory::Extensions;
    use win32_system::{System, trace};
    use win32_winapi::{calling_convention::*, *};
    pub unsafe fn _TrackMouseEvent(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpEventTrack = <Option<&mut TRACKMOUSEEVENT>>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("comctl32") {
                trace::Record::new(
                    comctl32::_TrackMouseEvent_pos,
                    "comctl32",
                    "_TrackMouseEvent",
                    &[("lpEventTrack", &lpEventTrack)],
                )
                .enter()
            } else {
                None
            };
            let result = comctl32::_TrackMouseEvent(sys, lpEventTrack);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn InitCommonControls(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let __trace_record = if trace::enabled("comctl32") {
                trace::Record::new(
                    comctl32::InitCommonControls_pos,
                    "comctl32",
                    "InitCommonControls",
                    &[],
                )
                .enter()
            } else {
                None
            };
            let result = comctl32::InitCommonControls(sys);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
}
const SHIMS: [Shim; 2usize] = [
    Shim {
        name: "_TrackMouseEvent",
        func: Handler::Sync(wrappers::_TrackMouseEvent),
    },
    Shim {
        name: "InitCommonControls",
        func: Handler::Sync(wrappers::InitCommonControls),
    },
];
pub const DLL: BuiltinDLL = BuiltinDLL {
    file_name: "comctl32.dll",
    shims: &SHIMS,
    raw: std::include_bytes!("../../../dll/comctl32.dll"),
};
