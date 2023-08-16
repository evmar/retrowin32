use crate::Pod;
use std::mem::size_of;

/// A view into the x86 memory.  Distinct from a slice to better catch
/// accesses and also to expose casts to/from Pod types.
#[derive(Copy, Clone)]
pub struct Mem<'m>(&'m [u8]);

impl<'m> Mem<'m> {
    pub fn from_slice(s: &'m [u8]) -> Mem<'m> {
        //println!("resv {}", memory_raw::resv() as u64);

        Mem(s)
    }

    pub fn get<T: Clone + Pod>(&self, ofs: u32) -> T {
        let ofs = ofs as usize;
        let buf = &self.0[ofs..(ofs + size_of::<T>())];
        // Safety: the above slice has already verified bounds.
        // Sketchy: https://doc.rust-lang.org/std/ptr/fn.read.html#ownership-of-the-returned-value
        // The impl seems to just clone the bytes, so maybe we don't .clone() here?
        unsafe { std::ptr::read_unaligned(buf.as_ptr() as *const T) }
    }

    pub fn put<T: Copy + Pod>(&self, ofs: u32, val: T) {
        // TODO: alignment?
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
    pub fn as_mut_slice_todo(&self) -> &'m mut [u8] {
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

    // TODO: this fails if addr isn't appropriately aligned.
    // We need to revisit this whole "view" API...
    pub fn view<T: Pod>(&self, addr: u32) -> &'m T {
        let ofs = addr as usize;
        let buf = &self.0[ofs..(ofs + size_of::<T>())];
        // Safety: the above slice has already verified bounds.
        unsafe { &*(buf.as_ptr() as *const T) }
    }

    // TODO: this fails if addr isn't appropriately aligned.
    // We need to revisit this whole "view" API...
    pub fn view_mut<T: Pod>(&self, addr: u32) -> &'m mut T {
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

    /// Note: can returned unaligned pointers depending on addr.
    pub fn ptr_mut<T: Pod + Copy>(&self, addr: u32) -> *mut T {
        let ofs = addr as usize;
        let s = self.as_mut_slice_todo();
        let buf = &mut s[ofs..(ofs + size_of::<T>())];
        // Safety: the above slice has already verified bounds.
        buf.as_mut_ptr() as *mut T
    }
}
