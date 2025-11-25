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
use std::cell::RefMut;
use time::TimeThread;
use wave::WaveState;
use win32_system::{System, generic_get_state};
use win32_winapi::calling_convention::ABIReturn;

#[derive(Copy, Clone, Debug)]
pub enum MMRESULT {
    MMSYSERR_NOERROR = 0,
    MMSYSERR_NOTENABLED = 3,
}

impl Into<ABIReturn> for MMRESULT {
    fn into(self) -> ABIReturn {
        (self as u32).into()
    }
}

// TODO: we could have separate state for wave and time.
#[derive(Default)]
pub struct State {
    pub audio_enabled: bool,
    pub time_thread: Option<TimeThread>,
    pub wave: WaveState,
}

#[inline]
pub fn get_state(sys: &dyn System) -> RefMut<'_, State> {
    generic_get_state::<State>(sys)
}
