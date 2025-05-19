use crate::Mem;

#[derive(Default)]
pub struct RawMem {}

impl RawMem {
    pub fn mem(&self) -> Mem {
        Mem::from_ptrs(0 as *mut u8..(1 << 30) as *mut u8)
    }
    pub fn len(&self) -> u32 {
        0xFFFF_FFFF
    }
}
