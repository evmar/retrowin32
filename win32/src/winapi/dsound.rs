#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use crate::machine::Machine;

const E_FAIL: u32 = 0x80004005;
const DSERR_GENERIC: u32 = E_FAIL;

pub fn DirectSoundCreate(_machine: &mut Machine, _lpGuid: u32, _ppDS: u32, _pUnkOuter: u32) -> u32 {
    DSERR_GENERIC
}
