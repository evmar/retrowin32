// Idea for this Pod type comes from https://github.com/CasualX/pelite.
// I didn't copy the code but it's MIT-licensed anyway.

/// A trait for types where it's safe to reintepret_cast<> from/to random memory blocks.
pub unsafe trait Pod: 'static + Sized {
    unsafe fn clear_memory(&mut self, count: u32) {
        std::ptr::write_bytes(self as *mut Self as *mut u8, 0, count as usize);
    }

    fn clear_struct(&mut self) {
        // Safety: the all-zeroes struct is valid per Pod requirements.
        unsafe {
            std::ptr::write_bytes(self as *mut Self, 0, 1);
        }
    }
}

// See discussion of endianness in doc/design_notes.md.
unsafe impl Pod for u8 {}
unsafe impl Pod for [u8; 4] {} // pixels
unsafe impl Pod for u16 {}
unsafe impl Pod for u32 {}
unsafe impl Pod for u64 {}
unsafe impl Pod for f32 {}
unsafe impl Pod for f64 {}
