use crate::Pod;
use std::mem::size_of;

pub trait Extensions<'m>: Sized {
    fn get_ptr<T: Pod>(self, ofs: u32) -> *mut T;
    fn get_pod<T: Clone + Pod>(self, ofs: u32) -> T {
        unsafe { std::ptr::read_unaligned(self.get_ptr::<T>(ofs)) }
    }
    fn sub32(self, ofs: u32, len: u32) -> &'m [u8];
    fn slicez(self, ofs: u32) -> &'m [u8];

    /// Create an iterator over a contiguous array of Pod types.
    fn iter_pod<T: Clone + Pod>(self, ofs: u32, count: u32) -> Iterator<'m, T> {
        Iterator {
            buf: self.sub32(ofs, count * size_of::<T>() as u32),
            _marker: Default::default(),
        }
    }
}

/// See iter_pod.
pub struct Iterator<'m, T> {
    buf: &'m [u8],
    _marker: std::marker::PhantomData<&'m T>,
}
impl<'m, T: Pod + Clone> std::iter::Iterator for Iterator<'m, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.len() < size_of::<T>() {
            return None;
        }
        let obj = self.buf.get_pod::<T>(0);
        self.buf = &self.buf[size_of::<T>()..];
        Some(obj)
    }
}

impl<'m> Extensions<'m> for &'m [u8] {
    fn get_ptr<T: Pod>(self, ofs: u32) -> *mut T {
        unsafe {
            if ofs as usize + size_of::<T>() > self.len() {
                panic!("oob");
            }
            self.as_ptr().add(ofs as usize) as *mut T
        }
    }

    fn sub32(self, ofs: u32, len: u32) -> &'m [u8] {
        &self[ofs as usize..][..len as usize]
    }

    fn slicez(self, ofs: u32) -> &'m [u8] {
        let ofs = ofs as usize;
        let slice = &self[ofs..];
        let nul = slice.iter().position(|&c| c == 0).unwrap();
        &slice[..nul]
    }
}

/// A view into the x86 memory.
/// Basically a slice, but using pointers to:
/// 1. make accesses more explicit
/// 2. handle unaligned reads
/// 3. expose casts to/from Pod types
/// 4. in the case where mem literally slices from address 0, avoid making an illegal slice
///
/// TODO: some of the accessors on here that return references likely violate
/// Rust rules around aliasing.  It might be better to instead always copy from
/// memory, rather than aliasing into it.
#[derive(Copy, Clone)]
pub struct Mem<'m> {
    ptr: *mut u8,
    end: *mut u8,
    _marker: std::marker::PhantomData<&'m u8>,
}

impl<'m> Mem<'m> {
    pub fn from_slice(s: &'m [u8]) -> Mem<'m> {
        let range = s.as_ptr_range();
        Mem {
            ptr: range.start as *mut u8,
            end: range.end as *mut u8,
            _marker: std::marker::PhantomData::default(),
        }
    }

    pub fn is_oob<T>(&self, addr: u32) -> bool {
        self.ptr as usize + addr as usize + size_of::<T>() > self.end as usize
    }

    fn get_ptr_unchecked(&self, ofs: u32) -> *mut u8 {
        // Avoid using self.ptr.add here, because when self.ptr is 0 (for native Mems)
        // a later bounds check gets optimized out into always panicking.
        ((self.ptr as usize) + ofs as usize) as *mut u8
    }

    pub fn put<T: Copy + Pod>(&self, ofs: u32, val: T) {
        unsafe {
            let ptr = self.get_ptr::<u8>(ofs);
            // Need write_volatile here to ensure optimizer doesn't elide the write.
            std::ptr::write_volatile(ptr as *mut T, val)
        }
    }

    pub fn as_slice_todo(&self) -> &'m [u8] {
        unsafe { std::slice::from_raw_parts(self.ptr, self.len() as usize) }
    }

    /// TODO: don't expose slices of memory, as we might not have contiguous pages.
    pub fn as_mut_slice_todo(&self) -> &'m mut [u8] {
        unsafe {
            let len = self.end.offset_from(self.ptr) as usize;
            std::slice::from_raw_parts_mut(self.ptr, len)
        }
    }

    pub fn offset_from(&self, origin: Mem) -> u32 {
        (self.ptr as usize - origin.ptr as usize) as u32
    }

    pub fn len(&self) -> u32 {
        unsafe { self.end.offset_from(self.ptr) as u32 }
    }

    pub fn slice(&self, b: impl std::ops::RangeBounds<u32>) -> Mem<'m> {
        let bstart = match b.start_bound() {
            std::ops::Bound::Included(&n) => n,
            std::ops::Bound::Excluded(&n) => n + 1,
            std::ops::Bound::Unbounded => 0,
        };
        let bend = match b.end_bound() {
            std::ops::Bound::Included(&n) => n + 1,
            std::ops::Bound::Excluded(&n) => n,
            std::ops::Bound::Unbounded => self.len(),
        };
        unsafe {
            let ptr = self.get_ptr_unchecked(bstart);
            let end = self.get_ptr_unchecked(bend);
            if !(self.ptr..self.end).contains(&ptr) || !(self.ptr..self.end.add(1)).contains(&end) {
                panic!("oob slice: {bstart:x?}..{bend:x?}",);
            }
            Mem {
                ptr,
                end,
                _marker: std::marker::PhantomData::default(),
            }
        }
    }

    pub fn sub(&self, ofs: u32, len: u32) -> Mem<'m> {
        self.slice(ofs..(ofs + len))
    }

    // TODO: this fails if addr isn't appropriately aligned.
    // We need to revisit this whole "view" API...
    pub fn view<T: Pod>(&self, addr: u32) -> &'m T {
        unsafe {
            let ptr = self.get_ptr::<u8>(addr);
            &*(ptr as *const T)
        }
    }

    // TODO: this fails if addr isn't appropriately aligned.
    // We need to revisit this whole "view" API...
    pub fn view_mut<T: Pod>(&self, addr: u32) -> &'m mut T {
        unsafe {
            let ptr = self.get_ptr::<u8>(addr);
            &mut *(ptr as *mut T)
        }
    }

    pub fn view_n<T: Pod>(&self, ofs: u32, count: u32) -> &'m [T] {
        unsafe {
            let ptr = self.get_ptr_unchecked(ofs);
            if ptr.add(size_of::<T>() * count as usize) > self.end {
                panic!("oob");
            }
            std::slice::from_raw_parts(ptr as *const T, count as usize)
        }
    }

    /// Note: can returned unaligned pointers depending on addr.
    pub fn ptr_mut<T: Pod + Copy>(&self, addr: u32) -> *mut T {
        self.get_ptr::<T>(addr)
    }

    /// Create a new Mem with arbitrary lifetime.  Very unsafe, used in stack_args codegen.
    pub unsafe fn detach<'a, 'b>(&'a self) -> Mem<'b> {
        std::mem::transmute(*self)
    }
}

impl<'m> Extensions<'m> for Mem<'m> {
    fn get_ptr<T: Pod>(self, ofs: u32) -> *mut T {
        if ofs + size_of::<T>() as u32 > self.len() {
            panic!("oob");
        }
        self.get_ptr_unchecked(ofs) as *mut T
    }

    fn sub32(self, ofs: u32, len: u32) -> &'m [u8] {
        self.sub(ofs, len).as_slice_todo()
    }

    fn slicez(self, ofs: u32) -> &'m [u8] {
        let ofs = ofs as usize;
        let slice = &self.as_slice_todo()[ofs..];
        let nul = slice.iter().position(|&c| c == 0).unwrap();
        &slice[..nul]
    }
}
