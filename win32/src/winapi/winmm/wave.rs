use super::MMRESULT;
use crate::machine::Machine;
use bitflags::bitflags;

#[win32_derive::dllexport]
pub fn waveOutGetNumDevs(_machine: &mut Machine) -> u32 {
    // TODO: set me to 1 to enable sound:
    0
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

enum WAVE_FORMAT {
    _4M16 = 0x0000_0400,
}

#[win32_derive::dllexport]
pub fn waveOutGetDevCapsA(
    _machine: &mut Machine,
    uDeviceID: u32,
    pwoc: Option<&mut WAVEOUTCAPS>,
    cbwoc: u32,
) -> MMRESULT {
    assert_eq!(cbwoc, std::mem::size_of::<WAVEOUTCAPS>() as u32);

    *pwoc.unwrap() = WAVEOUTCAPS {
        wMid: 0,
        wPid: 0,
        vDriverVersion: 1,
        szPname: [0; 32],
        dwFormats: WAVE_FORMAT::_4M16 as u32,
        wChannels: 1, // mono
        wReserved1: 0,
        dwSupport: 0, // no features
    };
    MMRESULT::MMSYSERR_NOERROR
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
        const CALLBACK_FUNCTION = 0x0003_0000;
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
) -> MMRESULT {
    let flags = fdwOpen.unwrap();
    if flags.contains(WaveOutOpenFlags::CALLBACK_FUNCTION) {
        log::error!("todo");
    }
    *phwo.unwrap() = 1;
    MMRESULT::MMSYSERR_NOERROR
}

#[win32_derive::dllexport]
pub fn waveOutReset(_machine: &mut Machine, hwo: HWAVEOUT) -> MMRESULT {
    MMRESULT::MMSYSERR_NOERROR
}

#[win32_derive::dllexport]
pub fn waveOutClose(_machine: &mut Machine, hwo: HWAVEOUT) -> MMRESULT {
    MMRESULT::MMSYSERR_NOERROR
}

#[win32_derive::dllexport]
pub fn waveOutSetVolume(_machine: &mut Machine, hwo: HWAVEOUT, dwVolume: u32) -> MMRESULT {
    todo!()
}

#[win32_derive::dllexport]
pub fn waveOutGetVolume(_machine: &mut Machine, hwo: HWAVEOUT, pdwVolume: u32) -> MMRESULT {
    todo!()
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
) -> MMRESULT {
    assert_eq!(cbmmt, std::mem::size_of::<MMTIME>() as u32);
    let mmt = pmmt.unwrap();
    mmt.u.sample = 0;
    MMRESULT::MMSYSERR_NOERROR
}

#[repr(C)]
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

bitflags! {
    pub struct WHDR: u32 {
        const DONE      = 0x00000001;
        const PREPARED  = 0x00000002;
        const BEGINLOOP = 0x00000004;
        const ENDLOOP   = 0x00000008;
        const INQUEUE   = 0x00000010;
    }
}
impl TryFrom<u32> for WHDR {
    type Error = u32;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        WHDR::from_bits(value).ok_or(value)
    }
}

impl std::fmt::Debug for WAVEHDR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Implemented to specialize dwFlags and omit reserved fields.
        f.debug_struct("WAVEHDR")
            .field("lpData", &self.lpData)
            .field("dwBufferLength", &self.dwBufferLength)
            .field("dwBytesRecorded", &self.dwBytesRecorded)
            .field("dwUser", &self.dwUser)
            .field("dwFlags", &WHDR::try_from(self.dwFlags))
            .field("dwLoops", &self.dwLoops)
            .finish()
    }
}

#[win32_derive::dllexport]
pub fn waveOutPrepareHeader(
    _machine: &mut Machine,
    hwo: HWAVEOUT,
    pwh: Option<&WAVEHDR>,
    cbwh: u32,
) -> MMRESULT {
    assert_eq!(cbwh, std::mem::size_of::<WAVEHDR>() as u32);
    // This function is supposed to fill in fields in the WAVEHDR, but there's nothing
    // for us to do here.
    MMRESULT::MMSYSERR_NOERROR
}

#[win32_derive::dllexport]
pub fn waveOutUnprepareHeader(
    _machine: &mut Machine,
    hwo: HWAVEOUT,
    pwh: Option<&mut WAVEHDR>,
    cbwh: u32,
) -> MMRESULT {
    MMRESULT::MMSYSERR_NOERROR
}

#[win32_derive::dllexport]
pub fn waveOutWrite(
    _machine: &mut Machine,
    hwo: HWAVEOUT,
    pwh: Option<&WAVEHDR>,
    cbwh: u32,
) -> MMRESULT {
    log::info!("TODO: write audio");
    assert_eq!(cbwh, std::mem::size_of::<WAVEHDR>() as u32);
    MMRESULT::MMSYSERR_NOERROR
}
