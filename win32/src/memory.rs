use std::mem::size_of;

// Idea for this Pod type comes from https://github.com/CasualX/pelite.
// I didn't copy the code but it's MIT-licensed anyway.

/// A trait for types where it's safe to reintepret_cast<> from/to random memory blocks.
pub unsafe trait Pod: 'static {}

pub trait Memory {
    fn view<T: Pod>(&self, ofs: u32) -> &T;
    fn view_mut<T: Pod>(&mut self, ofs: u32) -> &mut T;
    fn read_u32(&self, ofs: u32) -> u32;
    fn write_u32(&mut self, ofs: u32, value: u32);
    fn read_strz(&self) -> &str;
}

// TODO: wrap the x86 memory with a newtype and use that here instead.
impl Memory for [u8] {
    fn view<T: Pod>(&self, addr: u32) -> &T {
        let ofs = addr as usize;
        let buf = &self[ofs..(ofs + size_of::<T>())];
        // Safety: the above slice has already verified bounds.
        unsafe { &*(buf.as_ptr() as *const T) }
    }
    fn view_mut<T: Pod>(&mut self, addr: u32) -> &mut T {
        let ofs = addr as usize;
        let buf = &mut self[ofs..(ofs + size_of::<T>())];
        // Safety: the above slice has already verified bounds.
        unsafe { &mut *(buf.as_mut_ptr() as *mut T) }
    }

    fn read_u32(&self, addr: u32) -> u32 {
        self.view::<DWORD>(addr).get()
    }
    fn write_u32(&mut self, addr: u32, value: u32) {
        self.view_mut::<DWORD>(addr).set(value)
    }

    fn read_strz(&self) -> &str {
        let mut span = self;
        if let Some(nul) = self.iter().position(|&c| c == 0) {
            span = &self[0..nul];
        }
        std::str::from_utf8(span).unwrap()
    }
}

pub struct WORD([u8; 2]);
impl WORD {
    // pub fn set(&mut self, val: u16) {
    //     self.0[0] = val as u8;
    //     self.0[1] = (val >> 8) as u8;
    // }
    pub fn get(&self) -> u16 {
        (self.0[1] as u16) << 8 | self.0[0] as u16
    }
}
unsafe impl Pod for WORD {}
impl std::fmt::Debug for WORD {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:x}", self.get())
    }
}

#[derive(Clone, Copy)]
pub struct DWORD([u8; 4]);
impl DWORD {
    pub fn new() -> Self {
        DWORD([0, 0, 0, 0])
    }
    pub fn set(&mut self, val: u32) {
        self.0[0] = val as u8;
        self.0[1] = (val >> 8) as u8;
        self.0[2] = (val >> 16) as u8;
        self.0[3] = (val >> 24) as u8;
    }
    pub fn get(&self) -> u32 {
        (self.0[3] as u32) << 24
            | (self.0[2] as u32) << 16
            | (self.0[1] as u32) << 8
            | self.0[0] as u32
    }
}
unsafe impl Pod for DWORD {}
impl std::fmt::Debug for DWORD {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:x}", self.get())
    }
}
impl From<u32> for DWORD {
    fn from(x: u32) -> Self {
        let mut dw = DWORD::new();
        dw.set(x);
        dw
    }
}
