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
    use winapi::ntdll::*;
    pub unsafe fn NtReadFile(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let FileHandle = <HFILE>::from_stack(mem, stack_args + 0u32);
        let Event = <u32>::from_stack(mem, stack_args + 4u32);
        let ApcRoutine = <u32>::from_stack(mem, stack_args + 8u32);
        let ApcContext = <u32>::from_stack(mem, stack_args + 12u32);
        let IoStatusBlock = <Option<&mut IO_STATUS_BLOCK>>::from_stack(mem, stack_args + 16u32);
        let Buffer = <ArrayWithSizeMut<u8>>::from_stack(mem, stack_args + 20u32);
        let ByteOffset = <Option<&mut u64>>::from_stack(mem, stack_args + 28u32);
        let Key = <u32>::from_stack(mem, stack_args + 32u32);
        let __trace_context = if crate::trace::enabled("ntdll") {
            Some(crate::trace::trace_begin(
                "ntdll",
                "NtReadFile",
                &[
                    ("FileHandle", &FileHandle),
                    ("Event", &Event),
                    ("ApcRoutine", &ApcRoutine),
                    ("ApcContext", &ApcContext),
                    ("IoStatusBlock", &IoStatusBlock),
                    ("Buffer", &Buffer),
                    ("ByteOffset", &ByteOffset),
                    ("Key", &Key),
                ],
            ))
        } else {
            None
        };
        let result = winapi::ntdll::NtReadFile(
            machine,
            FileHandle,
            Event,
            ApcRoutine,
            ApcContext,
            IoStatusBlock,
            Buffer,
            ByteOffset,
            Key,
        );
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::ntdll::NtReadFile_pos.0,
                winapi::ntdll::NtReadFile_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn RtlExitUserProcess(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let exit_code = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("ntdll") {
            Some(crate::trace::trace_begin(
                "ntdll",
                "RtlExitUserProcess",
                &[("exit_code", &exit_code)],
            ))
        } else {
            None
        };
        let result = winapi::ntdll::RtlExitUserProcess(machine, exit_code);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::ntdll::RtlExitUserProcess_pos.0,
                winapi::ntdll::RtlExitUserProcess_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
}
const SHIMS: [Shim; 2usize] = [
    Shim {
        name: "NtReadFile",
        func: Handler::Sync(wrappers::NtReadFile),
    },
    Shim {
        name: "RtlExitUserProcess",
        func: Handler::Sync(wrappers::RtlExitUserProcess),
    },
];
pub const DLL: BuiltinDLL = BuiltinDLL {
    file_name: "ntdll.dll",
    shims: &SHIMS,
    raw: std::include_bytes!("../../../dll/ntdll.dll"),
};
