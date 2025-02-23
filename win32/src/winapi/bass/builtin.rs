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
        winapi::{self, calling_convention::*, *},
    };
    use ::memory::Extensions;
    use winapi::bass::*;
    pub unsafe fn BASS_ChannelGetPosition(machine: &mut Machine, stack_args: u32) -> u64 {
        let mem = machine.mem().detach();
        let mode = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::winapi::trace::enabled("bass") {
            crate::winapi::trace::Record::new(
                winapi::bass::BASS_ChannelGetPosition_pos,
                "bass",
                "BASS_ChannelGetPosition",
                &[("mode", &mode)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::bass::BASS_ChannelGetPosition(machine, mode);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into_abireturn()
    }
    pub unsafe fn BASS_Free(machine: &mut Machine, stack_args: u32) -> u64 {
        let mem = machine.mem().detach();
        let arg1 = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::winapi::trace::enabled("bass") {
            crate::winapi::trace::Record::new(
                winapi::bass::BASS_Free_pos,
                "bass",
                "BASS_Free",
                &[("arg1", &arg1)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::bass::BASS_Free(machine, arg1);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into_abireturn()
    }
    pub unsafe fn BASS_Init(machine: &mut Machine, stack_args: u32) -> u64 {
        let mem = machine.mem().detach();
        let arg1 = <u32>::from_stack(mem, stack_args + 0u32);
        let arg2 = <u32>::from_stack(mem, stack_args + 4u32);
        let arg3 = <u32>::from_stack(mem, stack_args + 8u32);
        let arg4 = <u32>::from_stack(mem, stack_args + 12u32);
        let __trace_record = if crate::winapi::trace::enabled("bass") {
            crate::winapi::trace::Record::new(
                winapi::bass::BASS_Init_pos,
                "bass",
                "BASS_Init",
                &[
                    ("arg1", &arg1),
                    ("arg2", &arg2),
                    ("arg3", &arg3),
                    ("arg4", &arg4),
                ],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::bass::BASS_Init(machine, arg1, arg2, arg3, arg4);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into_abireturn()
    }
    pub unsafe fn BASS_MusicLoad(machine: &mut Machine, stack_args: u32) -> u64 {
        let mem = machine.mem().detach();
        let arg1 = <u32>::from_stack(mem, stack_args + 0u32);
        let arg2 = <u32>::from_stack(mem, stack_args + 4u32);
        let arg3 = <u32>::from_stack(mem, stack_args + 8u32);
        let arg4 = <u32>::from_stack(mem, stack_args + 12u32);
        let arg5 = <u32>::from_stack(mem, stack_args + 16u32);
        let __trace_record = if crate::winapi::trace::enabled("bass") {
            crate::winapi::trace::Record::new(
                winapi::bass::BASS_MusicLoad_pos,
                "bass",
                "BASS_MusicLoad",
                &[
                    ("arg1", &arg1),
                    ("arg2", &arg2),
                    ("arg3", &arg3),
                    ("arg4", &arg4),
                    ("arg5", &arg5),
                ],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::bass::BASS_MusicLoad(machine, arg1, arg2, arg3, arg4, arg5);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into_abireturn()
    }
    pub unsafe fn BASS_MusicPlay(machine: &mut Machine, stack_args: u32) -> u64 {
        let mem = machine.mem().detach();
        let arg1 = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_record = if crate::winapi::trace::enabled("bass") {
            crate::winapi::trace::Record::new(
                winapi::bass::BASS_MusicPlay_pos,
                "bass",
                "BASS_MusicPlay",
                &[("arg1", &arg1)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::bass::BASS_MusicPlay(machine, arg1);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into_abireturn()
    }
    pub unsafe fn BASS_MusicSetPositionScaler(machine: &mut Machine, stack_args: u32) -> u64 {
        let mem = machine.mem().detach();
        let arg1 = <u32>::from_stack(mem, stack_args + 0u32);
        let arg2 = <u32>::from_stack(mem, stack_args + 4u32);
        let __trace_record = if crate::winapi::trace::enabled("bass") {
            crate::winapi::trace::Record::new(
                winapi::bass::BASS_MusicSetPositionScaler_pos,
                "bass",
                "BASS_MusicSetPositionScaler",
                &[("arg1", &arg1), ("arg2", &arg2)],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::bass::BASS_MusicSetPositionScaler(machine, arg1, arg2);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into_abireturn()
    }
    pub unsafe fn BASS_Start(machine: &mut Machine, stack_args: u32) -> u64 {
        let mem = machine.mem().detach();
        let __trace_record = if crate::winapi::trace::enabled("bass") {
            crate::winapi::trace::Record::new(
                winapi::bass::BASS_Start_pos,
                "bass",
                "BASS_Start",
                &[],
            )
            .enter()
        } else {
            None
        };
        let result = winapi::bass::BASS_Start(machine);
        if let Some(mut __trace_record) = __trace_record {
            __trace_record.exit(&result);
        }
        result.into_abireturn()
    }
}
const SHIMS: [Shim; 7usize] = [
    Shim {
        name: "BASS_ChannelGetPosition",
        func: Handler::Sync(wrappers::BASS_ChannelGetPosition),
    },
    Shim {
        name: "BASS_Free",
        func: Handler::Sync(wrappers::BASS_Free),
    },
    Shim {
        name: "BASS_Init",
        func: Handler::Sync(wrappers::BASS_Init),
    },
    Shim {
        name: "BASS_MusicLoad",
        func: Handler::Sync(wrappers::BASS_MusicLoad),
    },
    Shim {
        name: "BASS_MusicPlay",
        func: Handler::Sync(wrappers::BASS_MusicPlay),
    },
    Shim {
        name: "BASS_MusicSetPositionScaler",
        func: Handler::Sync(wrappers::BASS_MusicSetPositionScaler),
    },
    Shim {
        name: "BASS_Start",
        func: Handler::Sync(wrappers::BASS_Start),
    },
];
pub const DLL: BuiltinDLL = BuiltinDLL {
    file_name: "bass.dll",
    shims: &SHIMS,
    raw: std::include_bytes!("../../../dll/bass.dll"),
};
