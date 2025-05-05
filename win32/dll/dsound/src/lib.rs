#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

mod builtin;

pub use builtin::DLL;

use memory::ExtensionsMut;
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use win32_system::{Heap, System};
use win32_winapi::{com::GUID, vtable};

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
    heap: Rc<Heap>,
    buffers: HashMap<u32, Buffer>,
}

impl State {
    pub fn init(sys: &mut dyn System) {
        let heap = sys.memory().new_heap(
            128 << 10, // chillin needs a 64kb buffer
            "dsound.dll heap".into(),
        );
        *get_state(sys).borrow_mut() = State {
            heap,
            buffers: Default::default(),
        };
    }
}

fn get_state(sys: &dyn System) -> &RefCell<State> {
    sys.state(&std::any::TypeId::of::<RefCell<State>>())
        .downcast_ref::<RefCell<State>>()
        .unwrap()
}

#[derive(Default)]
struct Buffer {
    addr: u32,
    size: u32,
    lock: Option<Lock>,
}

struct Lock {
    addr: u32,
    size: u32,
}

bitflags::bitflags! {
    #[derive(Debug, win32_derive::TryFromBitflags)]
    pub struct DSBCAPS: u32 {
        const PRIMARYBUFFER       = 0x00000001;
        const STATIC              = 0x00000002;
        const LOCHARDWARE         = 0x00000004;
        const LOCSOFTWARE         = 0x00000008;
        const CTRL3D              = 0x00000010;
        const CTRLFREQUENCY       = 0x00000020;
        const CTRLPAN             = 0x00000040;
        const CTRLVOLUME          = 0x00000080;
        const CTRLPOSITIONNOTIFY  = 0x00000100;
        const CTRLFX              = 0x00000200;
        const STICKYFOCUS         = 0x00004000;
        const GLOBALFOCUS         = 0x00008000;
        const GETCURRENTPOSITION2 = 0x00010000;
        const MUTE3DATMAXDISTANCE = 0x00020000;
        const LOCDEFER            = 0x00040000;
    }
}

bitflags::bitflags! {
    #[derive(Debug, win32_derive::TryFromBitflags)]
    pub struct DSBLOCK: u32 {
        const FROMWRITECURSOR = 0x00000001;
        const ENTIREBUFFER    = 0x00000002;
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct DSBUFFERDESC {
    pub dwSize: u32,
    pub dwFlags: DSBCAPS,
    pub dwBufferBytes: u32,
    pub dwReserved: u32,
    pub lpwfxFormat: u32,
    // pub guid3DAlgorithm: GUID,
}
unsafe impl memory::Pod for DSBUFFERDESC {}

#[repr(C)]
#[derive(Debug)]
pub struct WAVEFORMATEX {
    pub wFormatTag: u16,
    pub nChannels: u16,
    pub nSamplesPerSec: u32,
    pub nAvgBytesPerSec: u32,
    pub nBlockAlign: u16,
    pub wBitsPerSample: u16,
    pub cbSize: u16,
}
unsafe impl memory::Pod for WAVEFORMATEX {}

#[win32_derive::dllexport]
pub mod IDirectSound {
    use super::*;

    pub fn new(sys: &mut dyn System) -> u32 {
        let heap = get_state(sys).borrow().heap.clone();
        let mem = sys.mem();
        let lpDirectSound = heap.alloc(mem, 4);
        let vtable = sys.get_symbol("dsound.dll", "IDirectSound");
        mem.put_pod::<u32>(lpDirectSound, vtable);
        lpDirectSound
    }

    #[win32_derive::dllexport]
    pub fn Release(sys: &dyn System, this: u32) -> u32 {
        0
    }

    #[win32_derive::dllexport]
    pub fn CreateSoundBuffer(
        sys: &mut dyn System,
        this: u32,
        lpcDSBufferDesc: Option<&DSBUFFERDESC>,
        lplpDirectSoundBuffer: Option<&mut u32>,
        pUnkOuter: u32,
    ) -> u32 {
        let x86_buffer = IDirectSoundBuffer::new(sys);
        let desc = lpcDSBufferDesc.unwrap();
        assert!(desc.dwSize == std::mem::size_of::<DSBUFFERDESC>() as u32);
        *lplpDirectSoundBuffer.unwrap() = x86_buffer;

        let mut dsound = get_state(sys).borrow_mut();
        let mut buffer = Buffer::default();
        if !desc.dwFlags.contains(DSBCAPS::PRIMARYBUFFER) {
            buffer.addr = dsound.heap.alloc(sys.mem(), desc.dwBufferBytes);
            buffer.size = desc.dwBufferBytes;
        }

        dsound.buffers.insert(x86_buffer, buffer);
        DS_OK
    }

    #[win32_derive::dllexport]
    pub fn SetCooperativeLevel(sys: &dyn System, this: u32, hwnd: u32, dwLevel: u32) -> u32 {
        DS_OK
    }

    vtable![
        QueryInterface: todo,
        AddRef: todo,
        Release: ok,
        CreateSoundBuffer: ok,
        GetCaps: todo,
        DuplicateSoundBuffer: todo,
        SetCooperativeLevel: ok,
        Compact: todo,
        GetSpeakerConfig: todo,
        SetSpeakerConfig: todo,
        Initialize: todo,
    ];
}

#[win32_derive::dllexport]
pub mod IDirectSoundBuffer {
    use super::*;

    pub fn new(sys: &mut dyn System) -> u32 {
        let heap = get_state(sys).borrow().heap.clone();
        let mem = sys.mem();
        let lpDirectSoundBuffer = heap.alloc(mem, 4);
        let vtable = sys.get_symbol("dsound.dll", "IDirectSoundBuffer");
        mem.put_pod::<u32>(lpDirectSoundBuffer, vtable);
        lpDirectSoundBuffer
    }

    #[win32_derive::dllexport]
    pub fn Release(sys: &dyn System, this: u32) -> u32 {
        0
    }

    #[win32_derive::dllexport]
    pub fn GetCurrentPosition(
        sys: &dyn System,
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
    pub fn GetStatus(sys: &dyn System, this: u32, lpdwStatus: Option<&mut u32>) -> u32 {
        let status = lpdwStatus.unwrap();
        *status = 0;
        DS_OK
    }

    #[win32_derive::dllexport]
    pub fn Lock(
        sys: &mut dyn System,
        this: u32,
        dwWriteCursor: u32,
        dwWriteBytes: u32,
        lplpvAudioPtr1: Option<&mut u32>,
        lpdwAudioBytes1: Option<&mut u32>,
        lplpvAudioPtr2: Option<&mut u32>,
        lpdwAudioBytes2: Option<&mut u32>,
        dwFlags: Result<DSBLOCK, u32>,
    ) -> u32 {
        let mut dsound = get_state(sys).borrow_mut();
        let flags = dwFlags.unwrap();
        if flags.contains(DSBLOCK::ENTIREBUFFER) {
            let buf = dsound.buffers.get_mut(&this).unwrap();
            assert!(buf.lock.is_none());
            *lplpvAudioPtr1.unwrap() = buf.addr;
            *lpdwAudioBytes1.unwrap() = buf.size;
            buf.lock = Some(Lock {
                addr: buf.addr,
                size: buf.size,
            });
        } else {
            todo!();
        }
        DS_OK
    }

    #[win32_derive::dllexport]
    pub fn Play(
        sys: &dyn System,
        this: u32,
        dwReserved1: u32,
        dwReserved2: u32,
        dwFlags: u32,
    ) -> u32 {
        assert_eq!(dwFlags, 1); // LOOPING
        DS_OK
    }

    #[win32_derive::dllexport]
    pub fn SetFormat(sys: &dyn System, this: u32, lpcfxFormat: Option<&WAVEFORMATEX>) -> u32 {
        let fmt = lpcfxFormat.unwrap();
        // Just check fmt is the one we support..
        assert!(matches!(
            fmt,
            WAVEFORMATEX {
                wFormatTag: 1, // PCM
                nChannels: 2,
                nSamplesPerSec: 44100,
                nAvgBytesPerSec: 176400, // 4*44100
                nBlockAlign: 4,
                wBitsPerSample: 16,
                cbSize: 0,
            }
        ));
        DS_OK
    }

    #[win32_derive::dllexport]
    pub fn Unlock(
        sys: &mut dyn System,
        this: u32,
        lpvAudioPtr1: u32,
        dwAudioBytes1: u32,
        lpvAudioPtr2: u32,
        dwAudioBytes2: u32,
    ) -> u32 {
        let mut dsound = get_state(sys).borrow_mut();
        let buf = dsound.buffers.get_mut(&this).unwrap();
        let lock = buf.lock.take().unwrap();

        assert_eq!(lpvAudioPtr1, lock.addr);
        assert_eq!(dwAudioBytes1, lock.size);
        // TODO: handle case where these don't match

        // Secondary lock not used.
        assert_eq!(lpvAudioPtr2, 0);
        assert_eq!(dwAudioBytes2, 0);

        DS_OK
    }

    vtable![
        QueryInterface: todo,
        AddRef: todo,
        Release: ok,
        GetCaps: todo,
        GetCurrentPosition: ok,
        GetFormat: todo,
        GetVolume: todo,
        GetPan: todo,
        GetFrequency: todo,
        GetStatus: ok,
        Initialize: todo,
        Lock: ok,
        Play: ok,
        SetCurrentPosition: todo,
        SetFormat: ok,
        SetVolume: todo,
        SetPan: todo,
        SetFrequency: todo,
        Stop: todo,
        Unlock: ok,
        Restore: todo,
    ];
}

#[win32_derive::dllexport(ordinal = 1)]
pub fn DirectSoundCreate(
    sys: &mut dyn System,
    lpGuid: Option<&GUID>,
    ppDS: Option<&mut u32>,
    pUnkOuter: u32,
) -> u32 {
    if DISABLE {
        return DSERR_NODRIVER;
    }
    if get_state(sys).borrow().heap.size == 0 {
        State::init(sys);
    }
    let lpDirectSound = IDirectSound::new(sys);
    *ppDS.unwrap() = lpDirectSound;
    DS_OK
}

#[win32_derive::dllexport(ordinal = 2)]
pub fn DirectSoundEnumerateA(sys: &dyn System, lpDSEnumCallback: u32, lpContext: u32) -> u32 {
    // No sound devices => no calling the callback.
    DS_OK
}
