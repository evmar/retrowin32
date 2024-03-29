#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::heap::Heap;
use super::types::DWORD;
use crate::{machine::Emulator, machine::Machine, winapi::vtable};

const TRACE_CONTEXT: &'static str = "dsound";

pub const DS_OK: u32 = 0;
#[allow(unused)]
const E_FAIL: u32 = 0x80004005;
#[allow(unused)]
pub const DSERR_GENERIC: u32 = E_FAIL;

pub struct State {
    heap: Heap,
    vtable_IDirectSound: u32,
    vtable_IDirectSoundBuffer: u32,
}

impl State {
    pub fn new_init(machine: &mut Machine) -> Self {
        let mut dsound = State::default();
        dsound.heap = machine.state.kernel32.new_private_heap(
            &mut machine.emu.memory,
            0x1000,
            "dsound.dll heap".into(),
        );

        dsound.vtable_IDirectSound = IDirectSound::vtable(&mut dsound, machine);
        dsound.vtable_IDirectSoundBuffer = IDirectSoundBuffer::vtable(&mut dsound, machine);
        dsound
    }
}

impl Default for State {
    fn default() -> Self {
        State {
            heap: Heap::default(),
            vtable_IDirectSound: 0,
            vtable_IDirectSoundBuffer: 0,
        }
    }
}

#[win32_derive::shims_from_x86]
mod IDirectSound {
    use super::*;

    #[win32_derive::dllexport]
    pub fn CreateSoundBuffer(
        machine: &mut Machine,
        _this: u32,
        _lpcDSBufferDesc: u32,
        lplpDirectSoundBuffer: u32,
        _pUnkOuter: u32,
    ) -> u32 {
        let x86_buffer = IDirectSoundBuffer::new(machine);
        machine.mem().put::<u32>(lplpDirectSoundBuffer, x86_buffer);
        DS_OK
    }

    #[win32_derive::dllexport]
    pub fn SetCooperativeLevel(
        _machine: &mut Machine,
        _this: u32,
        _hwnd: u32,
        _dwLevel: u32,
    ) -> u32 {
        DS_OK
    }

    vtable![IDirectSound shims
        QueryInterface todo,
        AddRef todo,
        Release todo,
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
        let vtable = dsound.vtable_IDirectSoundBuffer;
        machine.mem().put::<u32>(lpDirectSoundBuffer, vtable);
        lpDirectSoundBuffer
    }

    #[win32_derive::dllexport]
    pub fn Lock(
        _machine: &mut Machine,
        _this: u32,
        _dwWriteCursor: u32,
        _dwWriteBytes: u32,
        _lplpvAudioPtr1: u32,
        _lpdwAudioBytes1: u32,
        _lplpvAudioPtr2: u32,
        _lpdwAudioBytes2: u32,
        _dwFlags: u32,
    ) -> u32 {
        DS_OK
    }

    #[win32_derive::dllexport]
    pub fn Play(
        _machine: &mut Machine,
        _this: u32,
        _dwReserved1: u32,
        _dwReserved2: u32,
        _dwFlags: u32,
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
        _this: u32,
        _lpvAudioPtr1: u32,
        _dwAudioBytes1: u32,
        _lpvAudioPtr2: u32,
        _dwAudioBytes2: u32,
    ) -> u32 {
        DS_OK
    }

    vtable![IDirectSound shims
        QueryInterface todo,
        AddRef todo,
        Release todo,
        GetCaps todo,
        GetCurrentPosition todo,
        GetFormat todo,
        GetVolume todo,
        GetPan todo,
        GetFrequency todo,
        GetStatus todo,
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
pub fn DirectSoundCreate(machine: &mut Machine, _lpGuid: u32, ppDS: u32, _pUnkOuter: u32) -> u32 {
    if machine.state.dsound.heap.addr == 0 {
        machine.state.dsound = State::new_init(machine);
    }
    let dsound = &mut machine.state.dsound;

    let lpDirectSound = dsound.heap.alloc(machine.emu.memory.mem(), 4);
    let vtable = dsound.vtable_IDirectSound;
    machine.mem().put::<u32>(lpDirectSound, vtable);
    machine.mem().put::<u32>(ppDS, lpDirectSound);
    DS_OK
}
