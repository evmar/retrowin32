#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::heap::Heap;
use crate::{machine::Machine, winapi::com::vtable};

const TRACE_CONTEXT: &'static str = "dsound";

/// Set to true to make DirectSoundCreate report no sound device available.
/// Doing this from the beginning would have been a better idea than trying to implement stubs here...
const DISABLE: bool = true;

pub const DS_OK: u32 = 0;
#[allow(unused)]
const E_FAIL: u32 = 0x80004005;
#[allow(unused)]
pub const DSERR_GENERIC: u32 = E_FAIL;
#[allow(unused)]
pub const DSERR_NODRIVER: u32 = make_dhsresult(120);

const fn make_dhsresult(code: u32) -> u32 {
    (1 << 31) | (0x878 << 16) | code
}

#[derive(Default)]
pub struct State {
    heap: Heap,
    vtable_IDirectSound: Option<u32>,
    vtable_IDirectSoundBuffer: Option<u32>,
}

impl State {
    pub fn new_init(machine: &mut Machine) -> Self {
        let mut dsound = State::default();
        dsound.heap = machine.state.kernel32.new_private_heap(
            &mut machine.emu.memory,
            0x1000,
            "dsound.dll heap".into(),
        );
        dsound
    }
}

#[win32_derive::shims_from_x86]
mod IDirectSound {
    use super::*;

    pub fn new(machine: &mut Machine) -> u32 {
        let dsound = &mut machine.state.dsound;
        let lpDirectSoundBuffer = dsound.heap.alloc(machine.emu.memory.mem(), 4);
        let vtable = *dsound.vtable_IDirectSound.get_or_insert_with(|| {
            vtable(machine.emu.memory.mem(), &mut dsound.heap, |shim| {
                machine.emu.shims.add(shim)
            })
        });
        machine.mem().put::<u32>(lpDirectSoundBuffer, vtable);
        lpDirectSoundBuffer
    }

    #[win32_derive::dllexport]
    pub fn Release(_machine: &mut Machine, this: u32) -> u32 {
        0
    }

    #[win32_derive::dllexport]
    pub fn CreateSoundBuffer(
        machine: &mut Machine,
        this: u32,
        lpcDSBufferDesc: u32,
        lplpDirectSoundBuffer: u32,
        pUnkOuter: u32,
    ) -> u32 {
        let x86_buffer = IDirectSoundBuffer::new(machine);
        machine.mem().put::<u32>(lplpDirectSoundBuffer, x86_buffer);
        DS_OK
    }

    #[win32_derive::dllexport]
    pub fn SetCooperativeLevel(_machine: &mut Machine, this: u32, hwnd: u32, dwLevel: u32) -> u32 {
        DS_OK
    }

    vtable![IDirectSound shims
        QueryInterface todo,
        AddRef todo,
        Release ok,
        CreateSoundBuffer ok,
        GetCaps todo,
        DuplicateSoundBuffer todo,
        SetCooperativeLevel ok,
        Compact todo,
        GetSpeakerConfig todo,
        SetSpeakerConfig todo,
        Initialize todo,
    ];
}

#[win32_derive::shims_from_x86]
mod IDirectSoundBuffer {
    use super::*;

    pub fn new(machine: &mut Machine) -> u32 {
        let dsound = &mut machine.state.dsound;
        let lpDirectSoundBuffer = dsound.heap.alloc(machine.emu.memory.mem(), 4);
        let vtable = *dsound.vtable_IDirectSoundBuffer.get_or_insert_with(|| {
            vtable(machine.emu.memory.mem(), &mut dsound.heap, |shim| {
                machine.emu.shims.add(shim)
            })
        });
        machine.mem().put::<u32>(lpDirectSoundBuffer, vtable);
        lpDirectSoundBuffer
    }

    #[win32_derive::dllexport]
    pub fn Release(_machine: &mut Machine, this: u32) -> u32 {
        0
    }

    #[win32_derive::dllexport]
    pub fn GetCurrentPosition(
        _machine: &mut Machine,
        this: u32,
        lpdwCurrentPlayCursor: Option<&mut u32>,
        lpdwCurrentWriteCursor: Option<&mut u32>,
    ) -> u32 {
        match lpdwCurrentPlayCursor {
            Some(play) => *play = 0,
            None => {}
        }
        match lpdwCurrentWriteCursor {
            Some(write) => *write = 0,
            None => {}
        }
        DS_OK
    }

    #[win32_derive::dllexport]
    pub fn GetStatus(_machine: &mut Machine, this: u32, lpdwStatus: Option<&mut u32>) -> u32 {
        let status = lpdwStatus.unwrap();
        *status = 0;
        DS_OK
    }

    #[win32_derive::dllexport]
    pub fn Lock(
        _machine: &mut Machine,
        this: u32,
        dwWriteCursor: u32,
        dwWriteBytes: u32,
        lplpvAudioPtr1: u32,
        lpdwAudioBytes1: u32,
        lplpvAudioPtr2: u32,
        lpdwAudioBytes2: u32,
        dwFlags: u32,
    ) -> u32 {
        DS_OK
    }

    #[win32_derive::dllexport]
    pub fn Play(
        _machine: &mut Machine,
        this: u32,
        dwReserved1: u32,
        dwReserved2: u32,
        dwFlags: u32,
    ) -> u32 {
        DS_OK
    }

    #[win32_derive::dllexport]
    pub fn SetFormat(_machine: &mut Machine, _this: u32, _lpcfxFormat: u32) -> u32 {
        DS_OK
    }

    #[win32_derive::dllexport]
    pub fn Unlock(
        _machine: &mut Machine,
        this: u32,
        lpvAudioPtr1: u32,
        dwAudioBytes1: u32,
        lpvAudioPtr2: u32,
        dwAudioBytes2: u32,
    ) -> u32 {
        DS_OK
    }

    vtable![IDirectSound shims
        QueryInterface todo,
        AddRef todo,
        Release ok,
        GetCaps todo,
        GetCurrentPosition ok,
        GetFormat todo,
        GetVolume todo,
        GetPan todo,
        GetFrequency todo,
        GetStatus ok,
        Initialize todo,
        Lock ok,
        Play ok,
        SetCurrentPosition todo,
        SetFormat ok,
        SetVolume todo,
        SetPan todo,
        SetFrequency todo,
        Stop todo,
        Unlock ok,
        Restore todo,
    ];
}

#[win32_derive::dllexport(1)]
pub fn DirectSoundCreate(machine: &mut Machine, lpGuid: u32, ppDS: u32, pUnkOuter: u32) -> u32 {
    if DISABLE {
        return DSERR_NODRIVER;
    }
    if machine.state.dsound.heap.addr == 0 {
        machine.state.dsound = State::new_init(machine);
    }
    let lpDirectSound = IDirectSound::new(machine);
    machine.mem().put::<u32>(ppDS, lpDirectSound);
    DS_OK
}

#[win32_derive::dllexport(2)]
pub fn DirectSoundEnumerateA(_machine: &mut Machine, lpDSEnumCallback: u32, lpContext: u32) -> u32 {
    // No sound devices => no calling the callback.
    DS_OK
}
