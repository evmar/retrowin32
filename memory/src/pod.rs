// Idea for this Pod type comes from https://github.com/CasualX/pelite.
// I didn't copy the code but it's MIT-licensed anyway.

/// A trait for types where it's safe to:
/// - reintepret_cast<> from/to random memory blocks;
/// - initialize from all-zeros memory
pub unsafe trait Pod: 'static + Sized {
    fn zeroed() -> Self {
        // Safety: the all-zeroes struct is valid per Pod requirements.
        unsafe { std::mem::zeroed() }
    }

    /// Clear a Pod by a specified count of bytes.
    /// This is for cases where there's a variable-length C struct, with a header
    /// and some runtime-determined quantity of bytes following.
    unsafe fn clear_memory(&mut self, count: u32) {
        unsafe {
            std::ptr::write_bytes(self as *mut Self as *mut u8, 0, count as usize);
        }
    }
}

// See discussion of endianness in doc/design_notes.md.
unsafe impl Pod for u8 {}
unsafe impl Pod for [u8; 4] {} // pixels
unsafe impl Pod for [u8; 10] {} // long double
unsafe impl Pod for u16 {}
unsafe impl Pod for i16 {}
unsafe impl Pod for u32 {}
unsafe impl Pod for i32 {}
unsafe impl Pod for u64 {}
unsafe impl Pod for i64 {}
unsafe impl Pod for f32 {}
unsafe impl Pod for f64 {}
