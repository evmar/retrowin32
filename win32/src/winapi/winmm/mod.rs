#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

mod misc;
mod mixer;
mod time;
mod wave;

#[derive(Copy, Clone, Debug)]
pub enum MMRESULT {
    MMSYSERR_NOERROR = 0,
}
impl super::stack_args::ToX86 for MMRESULT {
    fn to_raw(&self) -> u32 {
        *self as u32
    }
}

pub use misc::*;
pub use mixer::*;
pub use time::*;
pub use wave::*;
