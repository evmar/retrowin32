#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::alloc::Alloc;
use super::types::DWORD;
use crate::machine::Machine;
use crate::winapi::vtable;
use x86::Memory;

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
    pub fn new_empty() -> Self {
        State {
            hheap: 0,
            vtable_IDirectSound: 0,
            vtable_IDirectSoundBuffer: 0,
        }
    }
    pub fn new_init(machine: &mut Machine) -> Self {
        let mut dsound = State::new_empty();
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

mod IDirectSound {
    use super::*;

    vtable![shims
        QueryInterface todo,
        AddRef todo,
        Release todo,
        CreateSoundBuffer todo,
        GetCaps todo,
        DuplicateSoundBuffer todo,
        SetCooperativeLevel todo,
        Compact todo,
        GetSpeakerConfig todo,
        SetSpeakerConfig todo,
        Initialize todo,
    ];
}

mod IDirectSoundBuffer {
    use super::*;

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
        Lock todo,
        Play todo,
        SetCurrentPosition todo,
        SetFormat todo,
        SetVolume todo,
        SetPan todo,
        SetFrequency todo,
        Stop todo,
        Unlock todo,
        Restore todo,
    ];
}

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
