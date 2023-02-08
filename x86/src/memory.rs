use std::mem::size_of;

// Idea for this Pod type comes from https://github.com/CasualX/pelite.
// I didn't copy the code but it's MIT-licensed anyway.

/// A trait for types where it's safe to reintepret_cast<> from/to random memory blocks.
pub unsafe trait Pod: 'static {}

// See discussion of endianness in doc/design_notes.md.
unsafe impl Pod for u16 {}
unsafe impl Pod for u32 {}
unsafe impl Pod for u64 {}

pub trait Memory {
    fn view<T: Pod>(&self, ofs: u32) -> &T;
    fn view_mut<T: Pod>(&mut self, ofs: u32) -> &mut T;
    fn read_u32(&self, ofs: u32) -> u32;
    fn write_u32(&mut self, ofs: u32, value: u32);
    /// Read a nul-terminated string.
    fn read_strz(&self) -> &str;
    /// Read a nul-terminated string, including the trailing nul.
    fn read_strz_with_nul(&self) -> &str;
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
        *self.view::<u32>(addr)
    }
    fn write_u32(&mut self, addr: u32, value: u32) {
        *self.view_mut::<u32>(addr) = value;
    }

    fn read_strz(&self) -> &str {
        let mut span = self;
        if let Some(nul) = self.iter().position(|&c| c == 0) {
            span = &self[0..nul];
        }
        std::str::from_utf8(span).unwrap()
    }

    fn read_strz_with_nul(&self) -> &str {
        let mut span = self;
        if let Some(nul) = self.iter().position(|&c| c == 0) {
            span = &self[0..nul + 1];
        }
        std::str::from_utf8(span).unwrap()
    }
}
