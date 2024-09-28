use crate::Pod;
use std::mem::size_of;

#[inline(never)]
fn aligned_panic(ptr: usize, align: usize) {
    // TODO: this should be a panic but anatyda.exe currently uses unaligned access
    // in the FromStack bits, needs a rethink.
    log::warn!("pointer {ptr:x} should be aligned to {align}",);
}

fn check_aligned<T: Pod>(ptr: *const T) {
    let align = std::mem::align_of::<T>();
    if ptr as usize % align != 0 {
        aligned_panic(ptr as usize, align);
    }
}

pub trait Extensions<'m>: Sized {
    fn as_slice(self) -> &'m [u8];

    fn get_ptr<T: Pod>(self, ofs: u32) -> *const T;
    fn get_pod<T: Clone + Pod>(self, ofs: u32) -> T {
        unsafe { std::ptr::read_unaligned(self.get_ptr::<T>(ofs)) }
    }
    fn get_aligned_ref<T: Pod>(self, ofs: u32) -> &'m T {
        let ptr = self.get_ptr::<T>(ofs);
        check_aligned(ptr);
        unsafe { &*(ptr as *const T) }
    }
    fn get_aligned_ref_mut<T: Pod>(self, ofs: u32) -> &'m mut T {
        let ptr = self.get_ptr::<T>(ofs);
        check_aligned(ptr);
        unsafe { &mut *(ptr as *mut T) }
    }

    fn sub32(self, ofs: u32, len: u32) -> &'m [u8];
    fn slicez(self, ofs: u32) -> &'m [u8];

    /// Create an iterator over a contiguous array of Pod types.
    fn into_iter_pod<T: Clone + Pod>(self) -> Iterator<'m, T> {
        Iterator {
            buf: self.as_slice(),
            _marker: Default::default(),
        }
    }

    fn iter_pod<T: Clone + Pod>(self, ofs: u32, count: u32) -> Iterator<'m, T> {
        self.sub32(ofs, count * size_of::<T>() as u32)
            .into_iter_pod()
    }
}

pub trait ExtensionsMut<'m>: Sized {
    fn get_ptr_mut<T: Pod>(self, ofs: u32) -> *mut T;
    fn sub32_mut(self, ofs: u32, len: u32) -> &'m mut [u8];
    fn put_pod<T: Clone + Pod>(self, ofs: u32, val: T) {
        unsafe {
            std::ptr::write_unaligned(self.get_ptr_mut::<T>(ofs), val);
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
    fn as_slice(self) -> &'m [u8] {
        self
    }

    fn get_ptr<T: Pod>(self, ofs: u32) -> *const T {
        unsafe {
            if ofs as usize + size_of::<T>() > self.len() {
                oob_panic(ofs, size_of::<T>());
            }
            self.as_ptr().add(ofs as usize) as *const T
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

impl<'m> ExtensionsMut<'m> for &'m mut [u8] {
    fn get_ptr_mut<T: Pod>(self, ofs: u32) -> *mut T {
        (self as &[u8]).get_ptr::<T>(ofs) as *mut T
    }
    fn sub32_mut(self, ofs: u32, len: u32) -> &'m mut [u8] {
        &mut self[ofs as usize..][..len as usize]
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
    pub fn from_ptrs(range: std::ops::Range<*const u8>) -> Mem<'m> {
        Mem {
            ptr: range.start as *mut u8,
            end: range.end as *mut u8,
            _marker: std::marker::PhantomData::default(),
        }
    }
    pub fn from_slice(s: &'m [u8]) -> Mem<'m> {
        Mem::from_ptrs(s.as_ptr_range())
    }

    pub fn is_oob<T>(&self, addr: u32) -> bool {
        self.ptr as usize + addr as usize + size_of::<T>() > self.end as usize
    }

    fn get_ptr_unchecked(&self, ofs: u32) -> *mut u8 {
        // Avoid using self.ptr.add here, because when self.ptr is 0 (for native Mems)
        // a later bounds check gets optimized out into always panicking.
        ((self.ptr as usize) + ofs as usize) as *mut u8
    }

    pub fn copy(&self, src: u32, dst: u32, len: u32) {
        unsafe {
            let src = self.get_ptr::<u8>(src);
            let dst = self.get_ptr_mut::<u8>(dst);
            std::ptr::copy(src, dst, len as usize);
        }
    }

    pub fn len(&self) -> u32 {
        unsafe { self.end.offset_from(self.ptr) as u32 }
    }

    pub fn slice(&self, b: impl std::ops::RangeBounds<u32>) -> &'m [u8] {
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
            std::slice::from_raw_parts(ptr, end.offset_from(ptr) as usize)
        }
    }

    /// Create a new Mem with arbitrary lifetime.  Very unsafe, used in stack_args codegen.
    pub unsafe fn detach<'a, 'b>(&'a self) -> Mem<'b> {
        std::mem::transmute(*self)
    }
}

#[inline(never)]
fn oob_panic(ofs: u32, size: usize) -> ! {
    panic!("oob at {ofs:x}+{size:x}");
}

impl<'m> Extensions<'m> for Mem<'m> {
    fn as_slice(self) -> &'m [u8] {
        unimplemented!()
    }

    fn get_ptr<T: Pod>(self, ofs: u32) -> *const T {
        let ptr = self.get_ptr_unchecked(ofs) as *const T;
        unsafe {
            if ptr.add(1) as *const _ > self.end {
                oob_panic(ofs, size_of::<T>());
            }
        }
        ptr
    }

    fn sub32(self, ofs: u32, len: u32) -> &'m [u8] {
        self.slice(ofs..(ofs + len))
    }

    fn slicez(self, ofs: u32) -> &'m [u8] {
        unsafe {
            let start = self.ptr.add(ofs as usize);
            let mut len = 0;
            while start.add(len) < self.end && *start.add(len) != 0 {
                len += 1;
            }
            std::slice::from_raw_parts(start, len)
        }
    }
}

impl<'m> ExtensionsMut<'m> for Mem<'m> {
    fn get_ptr_mut<T: Pod>(self, ofs: u32) -> *mut T {
        self.get_ptr::<T>(ofs) as *mut T
    }
    fn sub32_mut(self, ofs: u32, len: u32) -> &'m mut [u8] {
        assert!(ofs + len <= self.len());
        unsafe { std::slice::from_raw_parts_mut(self.ptr.add(ofs as usize), len as usize) }
    }
}
