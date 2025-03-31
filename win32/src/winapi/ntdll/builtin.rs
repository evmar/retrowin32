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
    use winapi::ntdll::*;
    pub unsafe fn NtCurrentTeb(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let __trace_record = if crate::winapi::trace::enabled("ntdll") {
            crate::winapi::trace::Record::new(
                winapi::ntdll::NtCurrentTeb_pos,
                "ntdll",
                "NtCurrentTeb",
                &[],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ntdll::NtCurrentTeb(machine);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn NtReadFile(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let FileHandle = <HFILE>::from_stack(mem, stack_args + 0u32);
        let Event = <u32>::from_stack(mem, stack_args + 4u32);
        let ApcRoutine = <u32>::from_stack(mem, stack_args + 8u32);
        let ApcContext = <u32>::from_stack(mem, stack_args + 12u32);
        let IoStatusBlock = <Option<&mut IO_STATUS_BLOCK>>::from_stack(mem, stack_args + 16u32);
        let Buffer = <ArrayOut<u8>>::from_stack(mem, stack_args + 20u32);
        let ByteOffset = <Option<&mut u64>>::from_stack(mem, stack_args + 28u32);
        let Key = <u32>::from_stack(mem, stack_args + 32u32);
        let __trace_record = if crate::winapi::trace::enabled("ntdll") {
            crate::winapi::trace::Record::new(
                winapi::ntdll::NtReadFile_pos,
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
            )
            .enter()
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
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
    pub unsafe fn RtlExitUserProcess(machine: &mut Machine, stack_args: u32) -> ABIReturn {
        let mem = machine.mem().detach();
        let exit_code = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::winapi::trace::enabled("ntdll") {
            crate::winapi::trace::Record::new(
                winapi::ntdll::RtlExitUserProcess_pos,
                "ntdll",
                "RtlExitUserProcess",
                &[("exit_code", &exit_code)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::ntdll::RtlExitUserProcess(machine, exit_code);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into()
    }
}
const SHIMS: [Shim; 3usize] = [
    Shim {
        name: "NtCurrentTeb",
        func: Handler::Sync(wrappers::NtCurrentTeb),
    },
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
