#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::alloc::Alloc;
use super::types::DWORD;
use crate::machine::Machine;
use crate::winapi::vtable;
use x86::Memory;

const TRACE: bool = true;

pub const DS_OK: u32 = 0;
#[allow(unused)]
const E_FAIL: u32 = 0x80004005;
#[allow(unused)]
pub const DSERR_GENERIC: u32 = E_FAIL;

pub struct State {
    hheap: u32,
    vtable_IDirectSound: u32,
    vtable_IDirectSoundBuffer: u32,
}

impl State {
    pub fn new_init(machine: &mut Machine) -> Self {
        let mut dsound = State::default();
        dsound.hheap =
            machine
                .state
                .kernel32
                .new_heap(&mut machine.x86.mem, 0x1000, "dsound.dll heap".into());

        dsound.vtable_IDirectSound = IDirectSound::vtable(&mut dsound, machine);
        dsound.vtable_IDirectSoundBuffer = IDirectSoundBuffer::vtable(&mut dsound, machine);
        dsound
    }
}

impl Default for State {
    fn default() -> Self {
        State {
            hheap: 0,
            vtable_IDirectSound: 0,
            vtable_IDirectSoundBuffer: 0,
        }
    }
}

#[win32_derive::shims_from_x86]
mod IDirectSound {
    use super::*;

    pub fn CreateSoundBuffer(
        machine: &mut Machine,
        _this: u32,
        _lpcDSBufferDesc: u32,
        lplpDirectSoundBuffer: u32,
        _pUnkOuter: u32,
    ) -> u32 {
        let x86_buffer = IDirectSoundBuffer::new(machine);
        machine.x86.mem.write_u32(lplpDirectSoundBuffer, x86_buffer);
        DS_OK
    }

    pub fn SetCooperativeLevel(
        _machine: &mut Machine,
        _this: u32,
        _hwnd: u32,
        _dwLevel: u32,
    ) -> u32 {
        DS_OK
    }

    vtable![shims
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
        let lpDirectSoundBuffer = machine
            .state
            .kernel32
            .get_heap(&mut machine.x86.mem, dsound.hheap)
            .unwrap()
            .alloc(4);
        let vtable = dsound.vtable_IDirectSoundBuffer;
        machine.x86.mem.write_u32(lpDirectSoundBuffer, vtable);
        lpDirectSoundBuffer
    }

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

    pub fn Play(
        _machine: &mut Machine,
        _this: u32,
        _dwReserved1: u32,
        _dwReserved2: u32,
        _dwFlags: u32,
    ) -> u32 {
        DS_OK
    }

    pub fn SetFormat(_machine: &mut Machine, _this: u32, _lpcfxFormat: u32) -> u32 {
        DS_OK
    }

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

    vtable![shims
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

#[win32_derive::dllexport]
pub fn DirectSoundCreate(machine: &mut Machine, _lpGuid: u32, ppDS: u32, _pUnkOuter: u32) -> u32 {
    if machine.state.dsound.hheap == 0 {
        machine.state.dsound = State::new_init(machine);
    }
    let dsound = &mut machine.state.dsound;

    let lpDirectSound = machine
        .state
        .kernel32
        .get_heap(&mut machine.x86.mem, dsound.hheap)
        .unwrap()
        .alloc(4);
    let vtable = dsound.vtable_IDirectSound;
    machine.x86.write_u32(lpDirectSound, vtable);
    machine.x86.write_u32(ppDS, lpDirectSound);
    DS_OK
}
