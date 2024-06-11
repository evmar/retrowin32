use crate::machine::Machine;
use bitflags::bitflags;
use memory::Pod;

const TRACE_CONTEXT: &'static str = "winmm/wave";

// This module isn't fully implemented yet.  This toggle lets us tell apps
// we don't support sound so they don't call into unimplemented code yet.
const ENABLED: bool = false;

#[win32_derive::dllexport]
pub fn waveOutGetNumDevs(_machine: &mut Machine) -> u32 {
    if ENABLED {
        1
    } else {
        0
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct WAVEOUTCAPS {
    pub wMid: u16,
    pub wPid: u16,
    pub vDriverVersion: u32,
    // TODO: TCHAR, could this be unicode based on cbwoc param?
    pub szPname: [u8; 32],
    pub dwFormats: u32,
    pub wChannels: u16,
    pub wReserved1: u16,
    pub dwSupport: u32,
}
unsafe impl memory::Pod for WAVEOUTCAPS {}

#[win32_derive::dllexport]
pub fn waveOutGetDevCapsA(
    _machine: &mut Machine,
    uDeviceID: u32,
    pwoc: Option<&mut WAVEOUTCAPS>,
    cbwoc: u32,
) -> u32 {
    let woc = pwoc.unwrap();
    assert_eq!(cbwoc, std::mem::size_of::<WAVEOUTCAPS>() as u32);
    *woc = WAVEOUTCAPS::zeroed();
    woc.dwFormats = 0x00000400; // WAVE_FORMAT_4M16
    0
}

pub type HWAVEOUT = u32;

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

bitflags! {
    pub struct WaveOutOpenFlags: u32 {
        const CALLBACK_FUNCTION = 0x00030000;
    }
}
impl TryFrom<u32> for WaveOutOpenFlags {
    type Error = u32;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        WaveOutOpenFlags::from_bits(value).ok_or(value)
    }
}

#[win32_derive::dllexport]
pub fn waveOutOpen(
    _machine: &mut Machine,
    phwo: Option<&mut HWAVEOUT>,
    uDeviceID: u32,
    pwfx: Option<&WAVEFORMATEX>,
    dwCallback: u32,
    dwInstance: u32,
    fdwOpen: Result<WaveOutOpenFlags, u32>,
) -> u32 {
    let flags = fdwOpen.unwrap();
    if flags.contains(WaveOutOpenFlags::CALLBACK_FUNCTION) {
        todo!();
    }
    *phwo.unwrap() = 1;
    0
}

#[win32_derive::dllexport]
pub fn waveOutReset(_machine: &mut Machine, hwo: HWAVEOUT) -> u32 {
    0
}

#[win32_derive::dllexport]
pub fn waveOutClose(_machine: &mut Machine, hwo: HWAVEOUT) -> u32 {
    0
}

#[repr(C)]
pub struct MMTIME {
    wType: u32,
    u: MMTIME_union,
}
unsafe impl memory::Pod for MMTIME {}

impl std::fmt::Debug for MMTIME {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MMTIME")
            .field("wType", &self.wType)
            .field("u", &"TODO")
            .finish()
    }
}

#[repr(C)]
pub union MMTIME_union {
    ms: u32,
    sample: u32,
    cb: u32,
    ticks: u32,
    smpte: MMTIME_smpte,
    midi: u32,
}
unsafe impl memory::Pod for MMTIME_union {}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct MMTIME_smpte {
    hour: u8,
    min: u8,
    sec: u8,
    frame: u8,
    fps: u8,
    dummy: u8,
    pad: [u8; 2],
}
unsafe impl memory::Pod for MMTIME_smpte {}

#[win32_derive::dllexport]
pub fn waveOutGetPosition(
    _machine: &mut Machine,
    hwo: HWAVEOUT,
    pmmt: Option<&mut MMTIME>,
    cbmmt: u32,
) -> u32 {
    assert_eq!(cbmmt, std::mem::size_of::<MMTIME>() as u32);
    let mmt = pmmt.unwrap();
    mmt.u.sample = 0;
    0
}

#[repr(C)]
#[derive(Debug)]
pub struct WAVEHDR {
    lpData: u32,
    dwBufferLength: u32,
    dwBytesRecorded: u32,
    dwUser: u32,
    dwFlags: u32,
    dwLoops: u32,
    lpNext: u32,
    reserved: u32,
}
unsafe impl memory::Pod for WAVEHDR {}

#[win32_derive::dllexport]
pub fn waveOutPrepareHeader(
    _machine: &mut Machine,
    hwo: HWAVEOUT,
    pwh: Option<&WAVEHDR>,
    cbwh: u32,
) -> u32 {
    assert_eq!(cbwh, std::mem::size_of::<WAVEHDR>() as u32);
    0
}

#[win32_derive::dllexport]
pub fn waveOutWrite(
    _machine: &mut Machine,
    hwo: HWAVEOUT,
    pwh: Option<&WAVEHDR>,
    cbwh: u32,
) -> u32 {
    assert_eq!(cbwh, std::mem::size_of::<WAVEHDR>() as u32);
    0
}
