use super::{MMRESULT, get_state};
use crate::{
    Machine,
    winapi::{self, Handles},
};
use bitflags::bitflags;
use memory::{Extensions, ExtensionsMut};
use std::{collections::VecDeque, sync::Arc};
use win32_system::{System, Wait, host};

pub type HWAVEOUT = winapi::HANDLE<WaveOut>;

#[derive(Default)]
pub struct WaveState {
    pub wave_outs: Handles<HWAVEOUT, WaveOut>,
}

#[derive(Clone)]
enum Notify {
    Function(u32, u32),
}

enum MM_WOM {
    // OPEN = 0x3BB,
    // CLOSE = 0x3BC,
    DONE = 0x3BD,
}

pub struct WaveOut {
    audio: Box<dyn host::Audio>,

    /// Queue of WAVEHDR addresses to process.
    blocks: VecDeque<u32>,
    host_ready: Arc<winapi::kernel32::EventObject>,
    block_ready: Arc<winapi::kernel32::EventObject>,
    /// How to notify the exe when a WAVEHDR is done.
    notify: Option<Notify>,
}

#[win32_derive::dllexport]
pub async fn retrowin32_wave_thread_main(machine: &mut Machine, hwo: HWAVEOUT) {
    // TODO: send the OPEN message here.
    loop {
        let host_ready = {
            let mut state = get_state(machine);
            let wave_out = state.wave.wave_outs.get_mut(hwo).unwrap();
            wave_out.host_ready.clone()
        };
        let ready = winapi::kernel32::wait_for_objects(
            machine,
            Box::new([winapi::kernel32::KernelObject::Event(host_ready)]),
            false,
            Wait::Forever,
        )
        .await;

        {
            let mut state = get_state(machine);
            let wave_out = state.wave.wave_outs.get_mut(hwo).unwrap();
            if wave_out.blocks.is_empty() {
                // TODO: wait on a condition variable
                let block_ready = wave_out.block_ready.clone();
                drop(state);

                let ready = winapi::kernel32::wait_for_objects(
                    machine,
                    Box::new([winapi::kernel32::KernelObject::Event(block_ready)]),
                    false,
                    Wait::Forever,
                )
                .await;
            }
        }

        let mut state = get_state(machine);
        let wave_out = state.wave.wave_outs.get_mut(hwo).unwrap();
        let wave_hdr_addr = wave_out.blocks.pop_front().unwrap();

        let mem = machine.memory.mem();
        let mut wave_hdr = mem.get_pod::<WAVEHDR>(wave_hdr_addr);
        let samples = mem.sub32(wave_hdr.lpData, wave_hdr.dwBufferLength);
        wave_out.audio.write(samples);

        wave_hdr.dwFlags = WHDR::DONE.bits();
        mem.put_pod(wave_hdr_addr, wave_hdr); // ick, rewrites the whole block

        let notify = wave_out.notify.clone();
        drop(state);

        match notify {
            Some(Notify::Function(callback, instance)) => {
                // void CALLBACK waveOutProc(
                //     HWAVEOUT  hwo,
                //     UINT      uMsg,
                //     DWORD_PTR dwInstance,
                //     DWORD_PTR dwParam1,
                //     DWORD_PTR dwParam2
                // );

                machine
                    .call_x86(
                        callback,
                        vec![
                            hwo.to_raw(),
                            MM_WOM::DONE as u32,
                            instance,
                            wave_hdr_addr,
                            0,
                        ],
                    )
                    .await;
            }
            None => {}
        }
    }
}

#[win32_derive::dllexport]
pub fn waveOutGetNumDevs(sys: &dyn System) -> u32 {
    if get_state(sys).audio_enabled { 1 } else { 0 }
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
    sys: &dyn System,
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

/// The types of callbacks that can be used with waveOutOpen.
#[derive(Debug, win32_derive::TryFromEnum)]
pub enum CALLBACK {
    NULL = 0x00000000,
    WINDOW = 0x00010000,
    TASK = 0x00020000,
    FUNCTION = 0x00030000,
    EVENT = 0x00050000,
}

#[derive(Debug)]
pub struct WaveOutOpenFlags {
    callback: CALLBACK,
    // todo: there are other possible flags
}

impl<'a> crate::calling_convention::FromStack<'a> for WaveOutOpenFlags {
    fn from_stack(mem: memory::Mem<'a>, sp: u32) -> Self {
        let flags = mem.get_pod::<u32>(sp);
        let callback = CALLBACK::try_from(flags & 0x00070000).unwrap();
        let flags = flags & !0x00070000;
        if flags != 0 {
            todo!();
        }
        WaveOutOpenFlags { callback }
    }
}

#[win32_derive::dllexport]
pub fn waveOutOpen(
    sys: &mut dyn System,
    phwo: Option<&mut HWAVEOUT>,
    uDeviceID: u32,
    pwfx: Option<&WAVEFORMATEX>,
    dwCallback: u32,
    dwInstance: u32,
    fdwOpen: WaveOutOpenFlags,
) -> MMRESULT {
    if !get_state(sys).audio_enabled {
        // Note that pocoman doesn't call waveOutGetNumDevs, but just directly calls
        // waveOutOpen and decides whether to do sound based on whether it succeeds.
        return MMRESULT::MMSYSERR_NOTENABLED;
    }

    let host_ready =
        winapi::kernel32::EventObject::new(Some("winmm host ready".into()), false, true);

    let fmt = pwfx.unwrap();
    let audio = sys.host().init_audio(
        fmt.nSamplesPerSec,
        Box::new({
            let host_ready = host_ready.clone();
            move || {
                host_ready.signal();
                // TODO: need to unblock the thread: machine.unblock_all();
            }
        }),
    );

    let notify = match fdwOpen.callback {
        CALLBACK::NULL => None,
        CALLBACK::WINDOW => {
            let hwnd = winapi::HWND::from_raw(dwCallback);
            log::warn!("TODO: waveOutOpen callback with window={hwnd:?}");
            None
        }
        CALLBACK::FUNCTION => Some(Notify::Function(dwCallback, dwInstance)),
        _ => todo!("callback {:?}", fdwOpen.callback),
    };

    let wave_out = WaveOut {
        audio,
        blocks: VecDeque::new(),
        host_ready,
        block_ready: winapi::kernel32::EventObject::new(
            Some("winmm block ready".into()),
            false,
            false,
        ),
        notify,
    };
    let hwo = get_state(sys).wave.wave_outs.add(wave_out);
    *phwo.unwrap() = hwo;

    let retrowin32_wave_thread_main = sys.get_symbol("winmm.dll", "retrowin32_wave_thread_main");
    sys.new_thread(0x1000, retrowin32_wave_thread_main, &[hwo.to_raw()]);

    MMRESULT::MMSYSERR_NOERROR
}

#[win32_derive::dllexport]
pub fn waveOutReset(sys: &dyn System, hwo: HWAVEOUT) -> MMRESULT {
    MMRESULT::MMSYSERR_NOERROR
}

#[win32_derive::dllexport]
pub fn waveOutClose(sys: &dyn System, hwo: HWAVEOUT) -> MMRESULT {
    let mut state = get_state(sys);
    let wave_out = state.wave.wave_outs.get_mut(hwo).unwrap();
    state.wave.wave_outs.remove(hwo);

    MMRESULT::MMSYSERR_NOERROR
}

#[win32_derive::dllexport]
pub fn waveOutSetVolume(sys: &dyn System, hwo: HWAVEOUT, dwVolume: u32) -> MMRESULT {
    todo!()
}

#[win32_derive::dllexport]
pub fn waveOutGetVolume(sys: &dyn System, hwo: HWAVEOUT, pdwVolume: u32) -> MMRESULT {
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
    sys: &dyn System,
    hwo: HWAVEOUT,
    pmmt: Option<&mut MMTIME>,
    cbmmt: u32,
) -> MMRESULT {
    assert_eq!(cbmmt, std::mem::size_of::<MMTIME>() as u32);

    let mut state = get_state(sys);
    let wave_out = state.wave.wave_outs.get_mut(hwo).unwrap();

    let pos = wave_out.audio.pos();
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
    sys: &dyn System,
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
    sys: &dyn System,
    hwo: HWAVEOUT,
    pwh: Option<&mut WAVEHDR>,
    cbwh: u32,
) -> MMRESULT {
    MMRESULT::MMSYSERR_NOERROR
}

#[win32_derive::dllexport]
pub fn waveOutWrite(machine: &mut Machine, hwo: HWAVEOUT, pwh: u32, cbwh: u32) -> MMRESULT {
    assert_eq!(cbwh, std::mem::size_of::<WAVEHDR>() as u32);

    let mut state = get_state(machine);
    let wave_out = state.wave.wave_outs.get_mut(hwo).unwrap();
    wave_out.blocks.push_back(pwh);
    wave_out.block_ready.signal();
    drop(state);

    // TODO: use condition variable to notify the thread
    machine.unblock_all();

    MMRESULT::MMSYSERR_NOERROR
}

#[win32_derive::dllexport]
pub fn waveOutPause(sys: &dyn System, hwo: HWAVEOUT) -> u32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn waveOutRestart(sys: &dyn System, hwo: HWAVEOUT) -> u32 {
    todo!()
}
