#![doc = r" Generated code, do not edit.  See winapi/builtin.rs for an overview."]
#![allow(unused_imports)]
#![allow(unused_variables)]
use win32_system::dll::*;
mod wrappers {
    use crate::winapi::dsound::{self, *};
    use ::memory::Extensions;
    use win32_system::{System, trace};
    use win32_winapi::{calling_convention::*, *};
    pub unsafe fn DirectSoundCreate(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpGuid = <Option<&GUID>>::from_stack(mem, stack_args + 0u32);
            let ppDS = <Option<&mut u32>>::from_stack(mem, stack_args + 4u32);
            let pUnkOuter = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("dsound") {
                trace::Record::new(
                    dsound::DirectSoundCreate_pos,
                    "dsound",
                    "DirectSoundCreate",
                    &[
                        ("lpGuid", &lpGuid),
                        ("ppDS", &ppDS),
                        ("pUnkOuter", &pUnkOuter),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = dsound::DirectSoundCreate(
                &mut *(sys.machine() as *mut crate::Machine),
                lpGuid,
                ppDS,
                pUnkOuter,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn DirectSoundEnumerateA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let lpDSEnumCallback = <u32>::from_stack(mem, stack_args + 0u32);
            let lpContext = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("dsound") {
                trace::Record::new(
                    dsound::DirectSoundEnumerateA_pos,
                    "dsound",
                    "DirectSoundEnumerateA",
                    &[
                        ("lpDSEnumCallback", &lpDSEnumCallback),
                        ("lpContext", &lpContext),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = dsound::DirectSoundEnumerateA(sys, lpDSEnumCallback, lpContext);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IDirectSoundBuffer_GetCurrentPosition(
        sys: &mut dyn System,
        stack_args: u32,
    ) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let lpdwCurrentPlayCursor = <Option<&mut u32>>::from_stack(mem, stack_args + 4u32);
            let lpdwCurrentWriteCursor = <Option<&mut u32>>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("dsound") {
                trace::Record::new(
                    dsound::IDirectSoundBuffer::GetCurrentPosition_pos,
                    "dsound",
                    "IDirectSoundBuffer::GetCurrentPosition",
                    &[
                        ("this", &this),
                        ("lpdwCurrentPlayCursor", &lpdwCurrentPlayCursor),
                        ("lpdwCurrentWriteCursor", &lpdwCurrentWriteCursor),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = dsound::IDirectSoundBuffer::GetCurrentPosition(
                sys,
                this,
                lpdwCurrentPlayCursor,
                lpdwCurrentWriteCursor,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IDirectSoundBuffer_GetStatus(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let lpdwStatus = <Option<&mut u32>>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("dsound") {
                trace::Record::new(
                    dsound::IDirectSoundBuffer::GetStatus_pos,
                    "dsound",
                    "IDirectSoundBuffer::GetStatus",
                    &[("this", &this), ("lpdwStatus", &lpdwStatus)],
                )
                .enter()
            } else {
                None
            };
            let result = dsound::IDirectSoundBuffer::GetStatus(sys, this, lpdwStatus);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IDirectSoundBuffer_Lock(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let dwWriteCursor = <u32>::from_stack(mem, stack_args + 4u32);
            let dwWriteBytes = <u32>::from_stack(mem, stack_args + 8u32);
            let lplpvAudioPtr1 = <Option<&mut u32>>::from_stack(mem, stack_args + 12u32);
            let lpdwAudioBytes1 = <Option<&mut u32>>::from_stack(mem, stack_args + 16u32);
            let lplpvAudioPtr2 = <Option<&mut u32>>::from_stack(mem, stack_args + 20u32);
            let lpdwAudioBytes2 = <Option<&mut u32>>::from_stack(mem, stack_args + 24u32);
            let dwFlags = <Result<DSBLOCK, u32>>::from_stack(mem, stack_args + 28u32);
            let __trace_record = if trace::enabled("dsound") {
                trace::Record::new(
                    dsound::IDirectSoundBuffer::Lock_pos,
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
                )
                .enter()
            } else {
                None
            };
            let result = dsound::IDirectSoundBuffer::Lock(
                &mut *(sys.machine() as *mut crate::Machine),
                this,
                dwWriteCursor,
                dwWriteBytes,
                lplpvAudioPtr1,
                lpdwAudioBytes1,
                lplpvAudioPtr2,
                lpdwAudioBytes2,
                dwFlags,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IDirectSoundBuffer_Play(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let dwReserved1 = <u32>::from_stack(mem, stack_args + 4u32);
            let dwReserved2 = <u32>::from_stack(mem, stack_args + 8u32);
            let dwFlags = <u32>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if trace::enabled("dsound") {
                trace::Record::new(
                    dsound::IDirectSoundBuffer::Play_pos,
                    "dsound",
                    "IDirectSoundBuffer::Play",
                    &[
                        ("this", &this),
                        ("dwReserved1", &dwReserved1),
                        ("dwReserved2", &dwReserved2),
                        ("dwFlags", &dwFlags),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result =
                dsound::IDirectSoundBuffer::Play(sys, this, dwReserved1, dwReserved2, dwFlags);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IDirectSoundBuffer_Release(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("dsound") {
                trace::Record::new(
                    dsound::IDirectSoundBuffer::Release_pos,
                    "dsound",
                    "IDirectSoundBuffer::Release",
                    &[("this", &this)],
                )
                .enter()
            } else {
                None
            };
            let result = dsound::IDirectSoundBuffer::Release(sys, this);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IDirectSoundBuffer_SetFormat(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let lpcfxFormat = <Option<&WAVEFORMATEX>>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("dsound") {
                trace::Record::new(
                    dsound::IDirectSoundBuffer::SetFormat_pos,
                    "dsound",
                    "IDirectSoundBuffer::SetFormat",
                    &[("this", &this), ("lpcfxFormat", &lpcfxFormat)],
                )
                .enter()
            } else {
                None
            };
            let result = dsound::IDirectSoundBuffer::SetFormat(sys, this, lpcfxFormat);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IDirectSoundBuffer_Unlock(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let lpvAudioPtr1 = <u32>::from_stack(mem, stack_args + 4u32);
            let dwAudioBytes1 = <u32>::from_stack(mem, stack_args + 8u32);
            let lpvAudioPtr2 = <u32>::from_stack(mem, stack_args + 12u32);
            let dwAudioBytes2 = <u32>::from_stack(mem, stack_args + 16u32);
            let __trace_record = if trace::enabled("dsound") {
                trace::Record::new(
                    dsound::IDirectSoundBuffer::Unlock_pos,
                    "dsound",
                    "IDirectSoundBuffer::Unlock",
                    &[
                        ("this", &this),
                        ("lpvAudioPtr1", &lpvAudioPtr1),
                        ("dwAudioBytes1", &dwAudioBytes1),
                        ("lpvAudioPtr2", &lpvAudioPtr2),
                        ("dwAudioBytes2", &dwAudioBytes2),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = dsound::IDirectSoundBuffer::Unlock(
                &mut *(sys.machine() as *mut crate::Machine),
                this,
                lpvAudioPtr1,
                dwAudioBytes1,
                lpvAudioPtr2,
                dwAudioBytes2,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IDirectSound_CreateSoundBuffer(
        sys: &mut dyn System,
        stack_args: u32,
    ) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let lpcDSBufferDesc = <Option<&DSBUFFERDESC>>::from_stack(mem, stack_args + 4u32);
            let lplpDirectSoundBuffer = <Option<&mut u32>>::from_stack(mem, stack_args + 8u32);
            let pUnkOuter = <u32>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if trace::enabled("dsound") {
                trace::Record::new(
                    dsound::IDirectSound::CreateSoundBuffer_pos,
                    "dsound",
                    "IDirectSound::CreateSoundBuffer",
                    &[
                        ("this", &this),
                        ("lpcDSBufferDesc", &lpcDSBufferDesc),
                        ("lplpDirectSoundBuffer", &lplpDirectSoundBuffer),
                        ("pUnkOuter", &pUnkOuter),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = dsound::IDirectSound::CreateSoundBuffer(
                &mut *(sys.machine() as *mut crate::Machine),
                this,
                lpcDSBufferDesc,
                lplpDirectSoundBuffer,
                pUnkOuter,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IDirectSound_Release(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("dsound") {
                trace::Record::new(
                    dsound::IDirectSound::Release_pos,
                    "dsound",
                    "IDirectSound::Release",
                    &[("this", &this)],
                )
                .enter()
            } else {
                None
            };
            let result = dsound::IDirectSound::Release(sys, this);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IDirectSound_SetCooperativeLevel(
        sys: &mut dyn System,
        stack_args: u32,
    ) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let hwnd = <u32>::from_stack(mem, stack_args + 4u32);
            let dwLevel = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("dsound") {
                trace::Record::new(
                    dsound::IDirectSound::SetCooperativeLevel_pos,
                    "dsound",
                    "IDirectSound::SetCooperativeLevel",
                    &[("this", &this), ("hwnd", &hwnd), ("dwLevel", &dwLevel)],
                )
                .enter()
            } else {
                None
            };
            let result = dsound::IDirectSound::SetCooperativeLevel(sys, this, hwnd, dwLevel);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
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
