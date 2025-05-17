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

use std::cell::{RefCell, RefMut};
use win32_system::System;
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

#[derive(Default)]
pub struct State {
    pub audio_enabled: bool,
    pub time_thread: Option<TimeThread>,
    pub wave: WaveState,
}

pub fn get_state(sys: &dyn System) -> RefMut<State> {
    // TODO: we could have separate state for wave and time.
    type SysState = RefCell<State>;
    sys.state(&std::any::TypeId::of::<SysState>())
        .downcast_ref::<SysState>()
        .unwrap()
        .borrow_mut()
}
