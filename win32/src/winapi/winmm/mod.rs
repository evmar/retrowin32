#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

mod builtin;
mod joy;
mod mci;
mod midi;
mod misc;
mod mixer;
mod time;
mod wave;

pub use builtin::DLL;

pub use joy::*;
pub use mci::*;
pub use midi::*;
pub use misc::*;
pub use mixer::*;
pub use time::*;
pub use wave::*;

#[derive(Copy, Clone, Debug)]
pub enum MMRESULT {
    MMSYSERR_NOERROR = 0,
    MMSYSERR_NOTENABLED = 3,
}

impl Into<crate::calling_convention::ABIReturn> for MMRESULT {
    fn into(self) -> crate::calling_convention::ABIReturn {
        (self as u32).into()
    }
}

#[derive(Default)]
pub struct State {
    pub audio_enabled: bool,
    pub time_thread: Option<TimeThread>,
    pub wave: WaveState,
}
