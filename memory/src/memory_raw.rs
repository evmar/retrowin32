use crate::Mem;

#[derive(Default)]
pub struct RawMem {}

impl RawMem {
    pub fn mem(&self) -> Mem {
        let s = unsafe { std::slice::from_raw_parts(0 as *const u8, 1 << 30) };
        Mem::from_slice(s)
    }
    pub fn len(&self) -> u32 {
        0xFFFF_FFFF
    }
    pub fn resize(&mut self, _size: u32, _value: u8) {
        unreachable!()
    }
}
