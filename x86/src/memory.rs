use std::mem::size_of;

// Idea for this Pod type comes from https://github.com/CasualX/pelite.
// I didn't copy the code but it's MIT-licensed anyway.

/// A trait for types where it's safe to reintepret_cast<> from/to random memory blocks.
pub unsafe trait Pod: 'static + Sized {
    unsafe fn clear_memory(&mut self, count: u32) {
        std::ptr::write_bytes(self as *mut Self as *mut u8, 0, count as usize);
    }

    unsafe fn clear_struct(&mut self) {
        std::ptr::write_bytes(self as *mut Self, 0, 1);
    }
}

// See discussion of endianness in doc/design_notes.md.
unsafe impl Pod for u8 {}
unsafe impl Pod for u16 {}
unsafe impl Pod for u32 {}
unsafe impl Pod for u64 {}
unsafe impl Pod for f64 {}

/// A view into the x86 memory.  Distinct from a slice to better catch
/// accesses and also to expose casts to/from Pod types.
pub struct Mem<'m>(&'m [u8]);

impl<'m> Mem<'m> {
    pub fn from_slice(s: &'m [u8]) -> Mem<'m> {
        Mem(s)
    }

    pub fn get<T: Copy + Pod>(&self, ofs: u32) -> T {
        *self.view(ofs)
    }

    pub fn put<T: Copy + Pod>(&mut self, ofs: u32, val: T) {
        *self.view_mut(ofs) = val;
    }

    pub fn slicez(&self, ofs: u32) -> Option<Mem<'m>> {
        let nul = self.0[ofs as usize..].iter().position(|&c| c == 0)? as u32;
        Some(self.sub(ofs, nul))
    }

    pub fn to_ascii(&self) -> &'m str {
        match std::str::from_utf8(self.0) {
            Ok(str) => str,
            Err(err) => {
                // If we hit one of these, we ought to change the caller to not use to_ascii().
                panic!("failed to ascii convert {:?}: {}", self.0, err);
            }
        }
    }

    /// TODO: don't expose slices of memory, as we might not have contiguous pages.
    pub fn as_slice_todo(&self) -> &'m [u8] {
        &self.0
    }

    /// TODO: don't expose slices of memory, as we might not have contiguous pages.
    pub fn as_mut_slice_todo(&mut self) -> &'m mut [u8] {
        // Safety: this is totally unsafe, just need to revisit this API.
        // Possibly should be using a raw pointer for the underlying type.
        unsafe { &mut *(self.0 as *const [u8] as *mut [u8]) }
    }

    pub fn len(&self) -> u32 {
        self.0.len() as u32
    }

    pub fn slice(&self, b: impl std::ops::RangeBounds<u32>) -> Mem<'m> {
        let start = match b.start_bound() {
            std::ops::Bound::Included(&n) => n as usize,
            std::ops::Bound::Excluded(&n) => n as usize + 1,
            std::ops::Bound::Unbounded => 0,
        };
        let end = match b.end_bound() {
            std::ops::Bound::Included(&n) => n as usize + 1,
            std::ops::Bound::Excluded(&n) => n as usize,
            std::ops::Bound::Unbounded => self.0.len(),
        };
        Mem::from_slice(&self.0[start..end])
    }

    pub fn sub(&self, ofs: u32, len: u32) -> Mem<'m> {
        self.slice(ofs..(ofs + len))
    }

    pub fn view<T: Pod>(&self, addr: u32) -> &'m T {
        let ofs = addr as usize;
        let buf = &self.0[ofs..(ofs + size_of::<T>())];
        // Safety: the above slice has already verified bounds.
        unsafe { &*(buf.as_ptr() as *const T) }
    }

    pub fn view_mut<T: Pod>(&mut self, addr: u32) -> &'m mut T {
        let ofs = addr as usize;
        let s = self.as_mut_slice_todo();
        let buf = &mut s[ofs..(ofs + size_of::<T>())];
        // Safety: the above slice has already verified bounds.
        unsafe { &mut *(buf.as_mut_ptr() as *mut T) }
    }

    pub fn view_n<T: Pod>(&self, ofs: u32, count: u32) -> &'m [T] {
        let size = size_of::<T>() as u32 * count;
        if ofs + size > self.0.len() as u32 {
            panic!("out of bounds");
        }
        unsafe {
            std::slice::from_raw_parts(
                &self.0[ofs as usize] as *const u8 as *const T,
                count as usize,
            )
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct VecMem(#[serde(with = "serde_bytes")] Vec<u8>);

impl VecMem {
    pub fn resize(&mut self, size: u32, value: u8) {
        self.0.resize(size as usize, value);
    }
    pub fn len(&self) -> u32 {
        self.0.len() as u32
    }
    pub fn mem(&self) -> Mem {
        Mem::from_slice(&self.0)
    }
}
