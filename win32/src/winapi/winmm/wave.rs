use super::MMRESULT;
use crate::machine::Machine;
use bitflags::bitflags;
use memory::Extensions;

#[win32_derive::dllexport]
pub fn waveOutGetNumDevs(machine: &mut Machine) -> u32 {
    if machine.state.winmm.audio_enabled {
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
    #[derive(Debug, win32_derive::TryFromBitflags)]
    pub struct WaveOutOpenFlags: u32 {
        const CALLBACK_FUNCTION = 0x0003_0000;
    }
}

#[win32_derive::dllexport]
pub fn waveOutOpen(
    machine: &mut Machine,
    phwo: Option<&mut HWAVEOUT>,
    uDeviceID: u32,
    pwfx: Option<&WAVEFORMATEX>,
    dwCallback: u32,
    dwInstance: u32,
    fdwOpen: Result<WaveOutOpenFlags, u32>,
) -> MMRESULT {
    if !machine.state.winmm.audio_enabled {
        // Note that pocoman doesn't call waveOutGetNumDevs, but just directly calls
        // waveOutOpen and decides whether to do sound based on whether it succeeds.
        return MMRESULT::MMSYSERR_NOTENABLED;
    }

    let flags = fdwOpen.unwrap();
    if flags.contains(WaveOutOpenFlags::CALLBACK_FUNCTION) {
        log::error!("todo");
    }
    *phwo.unwrap() = 1;

    let fmt = pwfx.unwrap();
    machine.state.winmm.audio = Some(machine.host.init_audio(fmt.nSamplesPerSec));

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

// These are TIME_* values, but that name conflicts with the TIME_* values in time.rs.
#[derive(Debug, win32_derive::TryFromEnum)]
pub enum MMTIME_TIME {
    MS = 0x0001,
    SAMPLES = 0x0002,
    BYTES = 0x0004,
    SMPTE = 0x0008,
    MIDI = 0x0010,
    TICKS = 0x0020,
}

impl std::fmt::Debug for MMTIME {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let time = MMTIME_TIME::try_from(self.wType);
        let mut st = f.debug_struct("MMTIME");
        st.field("wType", &time);
        unsafe {
            match time {
                Ok(MMTIME_TIME::MS) => {
                    st.field("ms", &self.u.ms);
                }
                Ok(MMTIME_TIME::SAMPLES) => {
                    st.field("sample", &self.u.sample);
                }
                Ok(MMTIME_TIME::BYTES) => {
                    st.field("cb", &self.u.cb);
                }
                Ok(MMTIME_TIME::SMPTE) => {
                    st.field("smpte", &self.u.smpte);
                }
                Ok(MMTIME_TIME::MIDI) => {
                    st.field("midi", &self.u.midi);
                }
                Ok(MMTIME_TIME::TICKS) => {
                    st.field("ticks", &self.u.ticks);
                }
                Err(_) => {}
            }
        }
        st.finish()
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
    machine: &mut Machine,
    hwo: HWAVEOUT,
    pmmt: Option<&mut MMTIME>,
    cbmmt: u32,
) -> MMRESULT {
    assert_eq!(cbmmt, std::mem::size_of::<MMTIME>() as u32);

    let pos = machine.state.winmm.audio.as_mut().unwrap().pos();
    let mmt = pmmt.unwrap();
    match MMTIME_TIME::try_from(mmt.wType).unwrap() {
        MMTIME_TIME::BYTES => mmt.u.cb = pos as u32 * 2,
        t => todo!("{:?}", t),
    }
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
    #[derive(Debug, win32_derive::TryFromBitflags)]
    pub struct WHDR: u32 {
        const DONE      = 0x00000001;
        const PREPARED  = 0x00000002;
        const BEGINLOOP = 0x00000004;
        const ENDLOOP   = 0x00000008;
        const INQUEUE   = 0x00000010;
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
    machine: &mut Machine,
    hwo: HWAVEOUT,
    pwh: Option<&WAVEHDR>,
    cbwh: u32,
) -> MMRESULT {
    assert_eq!(cbwh, std::mem::size_of::<WAVEHDR>() as u32);
    let hdr = pwh.unwrap();
    let buf = machine.memory.mem().sub32(hdr.lpData, hdr.dwBufferLength);
    machine.state.winmm.audio.as_mut().unwrap().write(buf);
    MMRESULT::MMSYSERR_NOERROR
}

#[win32_derive::dllexport]
pub fn waveOutPause(_machine: &mut Machine, hwo: HWAVEOUT) -> u32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn waveOutRestart(_machine: &mut Machine, hwo: HWAVEOUT) -> u32 {
    todo!()
}
