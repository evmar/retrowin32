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
    use winapi::dsound::*;
    pub unsafe fn DirectSoundCreate(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpGuid = <Option<&GUID>>::from_stack(mem, stack_args + 0u32);
        let ppDS = <Option<&mut u32>>::from_stack(mem, stack_args + 4u32);
        let pUnkOuter = <u32>::from_stack(mem, stack_args + 8u32);
        let __trace_context = if crate::trace::enabled("dsound") {
            Some(crate::trace::trace_begin(
                "dsound",
                "DirectSoundCreate",
                &[
                    ("lpGuid", &lpGuid),
                    ("ppDS", &ppDS),
                    ("pUnkOuter", &pUnkOuter),
                ],
            ))
        } else {
            None
        };
        let result = winapi::dsound::DirectSoundCreate(machine, lpGuid, ppDS, pUnkOuter);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::dsound::DirectSoundCreate_pos.0,
                winapi::dsound::DirectSoundCreate_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn DirectSoundEnumerateA(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let lpDSEnumCallback = <u32>::from_stack(mem, stack_args + 0u32);
        let lpContext = <u32>::from_stack(mem, stack_args + 4u32);
        let __trace_context = if crate::trace::enabled("dsound") {
            Some(crate::trace::trace_begin(
                "dsound",
                "DirectSoundEnumerateA",
                &[
                    ("lpDSEnumCallback", &lpDSEnumCallback),
                    ("lpContext", &lpContext),
                ],
            ))
        } else {
            None
        };
        let result = winapi::dsound::DirectSoundEnumerateA(machine, lpDSEnumCallback, lpContext);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::dsound::DirectSoundEnumerateA_pos.0,
                winapi::dsound::DirectSoundEnumerateA_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn IDirectSoundBuffer_GetCurrentPosition(
        machine: &mut Machine,
        stack_args: u32,
    ) -> u32 {
        let mem = machine.mem().detach();
        let this = <u32>::from_stack(mem, stack_args + 0u32);
        let lpdwCurrentPlayCursor = <Option<&mut u32>>::from_stack(mem, stack_args + 4u32);
        let lpdwCurrentWriteCursor = <Option<&mut u32>>::from_stack(mem, stack_args + 8u32);
        let __trace_context = if crate::trace::enabled("dsound") {
            Some(crate::trace::trace_begin(
                "dsound",
                "IDirectSoundBuffer::GetCurrentPosition",
                &[
                    ("this", &this),
                    ("lpdwCurrentPlayCursor", &lpdwCurrentPlayCursor),
                    ("lpdwCurrentWriteCursor", &lpdwCurrentWriteCursor),
                ],
            ))
        } else {
            None
        };
        let result = winapi::dsound::IDirectSoundBuffer::GetCurrentPosition(
            machine,
            this,
            lpdwCurrentPlayCursor,
            lpdwCurrentWriteCursor,
        );
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::dsound::IDirectSoundBuffer::GetCurrentPosition_pos.0,
                winapi::dsound::IDirectSoundBuffer::GetCurrentPosition_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn IDirectSoundBuffer_GetStatus(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let this = <u32>::from_stack(mem, stack_args + 0u32);
        let lpdwStatus = <Option<&mut u32>>::from_stack(mem, stack_args + 4u32);
        let __trace_context = if crate::trace::enabled("dsound") {
            Some(crate::trace::trace_begin(
                "dsound",
                "IDirectSoundBuffer::GetStatus",
                &[("this", &this), ("lpdwStatus", &lpdwStatus)],
            ))
        } else {
            None
        };
        let result = winapi::dsound::IDirectSoundBuffer::GetStatus(machine, this, lpdwStatus);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::dsound::IDirectSoundBuffer::GetStatus_pos.0,
                winapi::dsound::IDirectSoundBuffer::GetStatus_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn IDirectSoundBuffer_Lock(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let this = <u32>::from_stack(mem, stack_args + 0u32);
        let dwWriteCursor = <u32>::from_stack(mem, stack_args + 4u32);
        let dwWriteBytes = <u32>::from_stack(mem, stack_args + 8u32);
        let lplpvAudioPtr1 = <Option<&mut u32>>::from_stack(mem, stack_args + 12u32);
        let lpdwAudioBytes1 = <Option<&mut u32>>::from_stack(mem, stack_args + 16u32);
        let lplpvAudioPtr2 = <Option<&mut u32>>::from_stack(mem, stack_args + 20u32);
        let lpdwAudioBytes2 = <Option<&mut u32>>::from_stack(mem, stack_args + 24u32);
        let dwFlags = <Result<DSBLOCK, u32>>::from_stack(mem, stack_args + 28u32);
        let __trace_context = if crate::trace::enabled("dsound") {
            Some(crate::trace::trace_begin(
                "dsound",
                "IDirectSoundBuffer::Lock",
                &[
                    ("this", &this),
                    ("dwWriteCursor", &dwWriteCursor),
                    ("dwWriteBytes", &dwWriteBytes),
                    ("lplpvAudioPtr1", &lplpvAudioPtr1),
                    ("lpdwAudioBytes1", &lpdwAudioBytes1),
                    ("lplpvAudioPtr2", &lplpvAudioPtr2),
                    ("lpdwAudioBytes2", &lpdwAudioBytes2),
                    ("dwFlags", &dwFlags),
                ],
            ))
        } else {
            None
        };
        let result = winapi::dsound::IDirectSoundBuffer::Lock(
            machine,
            this,
            dwWriteCursor,
            dwWriteBytes,
            lplpvAudioPtr1,
            lpdwAudioBytes1,
            lplpvAudioPtr2,
            lpdwAudioBytes2,
            dwFlags,
        );
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::dsound::IDirectSoundBuffer::Lock_pos.0,
                winapi::dsound::IDirectSoundBuffer::Lock_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn IDirectSoundBuffer_Play(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let this = <u32>::from_stack(mem, stack_args + 0u32);
        let dwReserved1 = <u32>::from_stack(mem, stack_args + 4u32);
        let dwReserved2 = <u32>::from_stack(mem, stack_args + 8u32);
        let dwFlags = <u32>::from_stack(mem, stack_args + 12u32);
        let __trace_context = if crate::trace::enabled("dsound") {
            Some(crate::trace::trace_begin(
                "dsound",
                "IDirectSoundBuffer::Play",
                &[
                    ("this", &this),
                    ("dwReserved1", &dwReserved1),
                    ("dwReserved2", &dwReserved2),
                    ("dwFlags", &dwFlags),
                ],
            ))
        } else {
            None
        };
        let result = winapi::dsound::IDirectSoundBuffer::Play(
            machine,
            this,
            dwReserved1,
            dwReserved2,
            dwFlags,
        );
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::dsound::IDirectSoundBuffer::Play_pos.0,
                winapi::dsound::IDirectSoundBuffer::Play_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn IDirectSoundBuffer_Release(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let this = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("dsound") {
            Some(crate::trace::trace_begin(
                "dsound",
                "IDirectSoundBuffer::Release",
                &[("this", &this)],
            ))
        } else {
            None
        };
        let result = winapi::dsound::IDirectSoundBuffer::Release(machine, this);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::dsound::IDirectSoundBuffer::Release_pos.0,
                winapi::dsound::IDirectSoundBuffer::Release_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn IDirectSoundBuffer_SetFormat(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let this = <u32>::from_stack(mem, stack_args + 0u32);
        let lpcfxFormat = <Option<&WAVEFORMATEX>>::from_stack(mem, stack_args + 4u32);
        let __trace_context = if crate::trace::enabled("dsound") {
            Some(crate::trace::trace_begin(
                "dsound",
                "IDirectSoundBuffer::SetFormat",
                &[("this", &this), ("lpcfxFormat", &lpcfxFormat)],
            ))
        } else {
            None
        };
        let result = winapi::dsound::IDirectSoundBuffer::SetFormat(machine, this, lpcfxFormat);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::dsound::IDirectSoundBuffer::SetFormat_pos.0,
                winapi::dsound::IDirectSoundBuffer::SetFormat_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn IDirectSoundBuffer_Unlock(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let this = <u32>::from_stack(mem, stack_args + 0u32);
        let lpvAudioPtr1 = <u32>::from_stack(mem, stack_args + 4u32);
        let dwAudioBytes1 = <u32>::from_stack(mem, stack_args + 8u32);
        let lpvAudioPtr2 = <u32>::from_stack(mem, stack_args + 12u32);
        let dwAudioBytes2 = <u32>::from_stack(mem, stack_args + 16u32);
        let __trace_context = if crate::trace::enabled("dsound") {
            Some(crate::trace::trace_begin(
                "dsound",
                "IDirectSoundBuffer::Unlock",
                &[
                    ("this", &this),
                    ("lpvAudioPtr1", &lpvAudioPtr1),
                    ("dwAudioBytes1", &dwAudioBytes1),
                    ("lpvAudioPtr2", &lpvAudioPtr2),
                    ("dwAudioBytes2", &dwAudioBytes2),
                ],
            ))
        } else {
            None
        };
        let result = winapi::dsound::IDirectSoundBuffer::Unlock(
            machine,
            this,
            lpvAudioPtr1,
            dwAudioBytes1,
            lpvAudioPtr2,
            dwAudioBytes2,
        );
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::dsound::IDirectSoundBuffer::Unlock_pos.0,
                winapi::dsound::IDirectSoundBuffer::Unlock_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn IDirectSound_CreateSoundBuffer(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let this = <u32>::from_stack(mem, stack_args + 0u32);
        let lpcDSBufferDesc = <Option<&DSBUFFERDESC>>::from_stack(mem, stack_args + 4u32);
        let lplpDirectSoundBuffer = <Option<&mut u32>>::from_stack(mem, stack_args + 8u32);
        let pUnkOuter = <u32>::from_stack(mem, stack_args + 12u32);
        let __trace_context = if crate::trace::enabled("dsound") {
            Some(crate::trace::trace_begin(
                "dsound",
                "IDirectSound::CreateSoundBuffer",
                &[
                    ("this", &this),
                    ("lpcDSBufferDesc", &lpcDSBufferDesc),
                    ("lplpDirectSoundBuffer", &lplpDirectSoundBuffer),
                    ("pUnkOuter", &pUnkOuter),
                ],
            ))
        } else {
            None
        };
        let result = winapi::dsound::IDirectSound::CreateSoundBuffer(
            machine,
            this,
            lpcDSBufferDesc,
            lplpDirectSoundBuffer,
            pUnkOuter,
        );
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::dsound::IDirectSound::CreateSoundBuffer_pos.0,
                winapi::dsound::IDirectSound::CreateSoundBuffer_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn IDirectSound_Release(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let this = <u32>::from_stack(mem, stack_args + 0u32);
        let __trace_context = if crate::trace::enabled("dsound") {
            Some(crate::trace::trace_begin(
                "dsound",
                "IDirectSound::Release",
                &[("this", &this)],
            ))
        } else {
            None
        };
        let result = winapi::dsound::IDirectSound::Release(machine, this);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::dsound::IDirectSound::Release_pos.0,
                winapi::dsound::IDirectSound::Release_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
    pub unsafe fn IDirectSound_SetCooperativeLevel(machine: &mut Machine, stack_args: u32) -> u32 {
        let mem = machine.mem().detach();
        let this = <u32>::from_stack(mem, stack_args + 0u32);
        let hwnd = <u32>::from_stack(mem, stack_args + 4u32);
        let dwLevel = <u32>::from_stack(mem, stack_args + 8u32);
        let __trace_context = if crate::trace::enabled("dsound") {
            Some(crate::trace::trace_begin(
                "dsound",
                "IDirectSound::SetCooperativeLevel",
                &[("this", &this), ("hwnd", &hwnd), ("dwLevel", &dwLevel)],
            ))
        } else {
            None
        };
        let result =
            winapi::dsound::IDirectSound::SetCooperativeLevel(machine, this, hwnd, dwLevel);
        if let Some(__trace_context) = __trace_context {
            crate::trace::trace_return(
                &__trace_context,
                winapi::dsound::IDirectSound::SetCooperativeLevel_pos.0,
                winapi::dsound::IDirectSound::SetCooperativeLevel_pos.1,
                &result,
            );
        }
        result.to_raw()
    }
}
const SHIMS: [Shim; 12usize] = [
    Shim {
        name: "DirectSoundCreate",
        func: Handler::Sync(wrappers::DirectSoundCreate),
    },
    Shim {
        name: "DirectSoundEnumerateA",
        func: Handler::Sync(wrappers::DirectSoundEnumerateA),
    },
    Shim {
        name: "IDirectSoundBuffer::GetCurrentPosition",
        func: Handler::Sync(wrappers::IDirectSoundBuffer_GetCurrentPosition),
    },
    Shim {
        name: "IDirectSoundBuffer::GetStatus",
        func: Handler::Sync(wrappers::IDirectSoundBuffer_GetStatus),
    },
    Shim {
        name: "IDirectSoundBuffer::Lock",
        func: Handler::Sync(wrappers::IDirectSoundBuffer_Lock),
    },
    Shim {
        name: "IDirectSoundBuffer::Play",
        func: Handler::Sync(wrappers::IDirectSoundBuffer_Play),
    },
    Shim {
        name: "IDirectSoundBuffer::Release",
        func: Handler::Sync(wrappers::IDirectSoundBuffer_Release),
    },
    Shim {
        name: "IDirectSoundBuffer::SetFormat",
        func: Handler::Sync(wrappers::IDirectSoundBuffer_SetFormat),
    },
    Shim {
        name: "IDirectSoundBuffer::Unlock",
        func: Handler::Sync(wrappers::IDirectSoundBuffer_Unlock),
    },
    Shim {
        name: "IDirectSound::CreateSoundBuffer",
        func: Handler::Sync(wrappers::IDirectSound_CreateSoundBuffer),
    },
    Shim {
        name: "IDirectSound::Release",
        func: Handler::Sync(wrappers::IDirectSound_Release),
    },
    Shim {
        name: "IDirectSound::SetCooperativeLevel",
        func: Handler::Sync(wrappers::IDirectSound_SetCooperativeLevel),
    },
];
pub const DLL: BuiltinDLL = BuiltinDLL {
    file_name: "dsound.dll",
    shims: &SHIMS,
    raw: std::include_bytes!("../../../dll/dsound.dll"),
};
