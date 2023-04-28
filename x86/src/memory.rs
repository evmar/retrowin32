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

/// A trait for access into the x86 memory.  Distinct from a slice to better catch
/// accesses and also to expose casts to/from Pod types.
pub trait Memory {
    /// TODO: don't expose slices of memory, as we might not have contiguous pages.
    fn as_slice_todo(&self) -> &[u8];
    /// TODO: don't expose slices of memory, as we might not have contiguous pages.
    fn as_mut_slice_todo(&mut self) -> &mut [u8];
    fn len(&self) -> usize;

    fn slice(&self, b: impl std::ops::RangeBounds<usize>) -> &Mem;
    fn slice_mut(&mut self, b: impl std::ops::RangeBounds<usize>) -> &mut Mem;

    fn view<T: Pod>(&self, ofs: u32) -> &T;
    fn view_mut<T: Pod>(&mut self, ofs: u32) -> &mut T;
    fn view_n<T: Pod>(&self, ofs: usize, count: usize) -> &[T];

    fn read_u32(&self, ofs: u32) -> u32;
    fn write_u32(&mut self, ofs: u32, value: u32);
    /// Read a nul-terminated string.
    fn read_strz(&self) -> &str;
    /// Read a nul-terminated string, including the trailing nul.
    fn read_strz_with_nul(&self) -> &str;
}

fn read_strz(buf: &[u8]) -> &str {
    let str = read_strz_with_nul(buf);
    &str[..str.len() - 1]
}

fn read_strz_with_nul(buf: &[u8]) -> &str {
    let mut span = buf;
    if let Some(nul) = buf.iter().position(|&c| c == 0) {
        span = &buf[0..nul + 1];
    }
    match std::str::from_utf8(span) {
        Err(err) => {
            log::error!("invalid utf8 {err:?}, {span:?}");
            "[err]"
        }
        Ok(str) => str,
    }
}

pub struct Mem(pub [u8]);

impl Mem {
    pub fn from_slice(s: &[u8]) -> &Mem {
        // Safety: this is how OsStr does it, shruggie.
        unsafe { &*(s as *const [u8] as *const Mem) }
    }
    pub fn from_slice_mut(s: &mut [u8]) -> &mut Mem {
        // Safety: this is how OsStr does it, shruggie.
        unsafe { &mut *(s as *mut [u8] as *mut Mem) }
    }
}

fn start_from_bound(b: std::ops::Bound<&usize>) -> usize {
    match b {
        std::ops::Bound::Included(&n) => n,
        std::ops::Bound::Excluded(&n) => n + 1,
        std::ops::Bound::Unbounded => 0,
    }
}

fn end_from_bound(m: &Mem, b: std::ops::Bound<&usize>) -> usize {
    match b {
        std::ops::Bound::Included(&n) => n + 1,
        std::ops::Bound::Excluded(&n) => n,
        std::ops::Bound::Unbounded => m.0.len(),
    }
}

impl Memory for Mem {
    fn as_slice_todo(&self) -> &[u8] {
        &self.0
    }

    fn as_mut_slice_todo(&mut self) -> &mut [u8] {
        &mut self.0
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn slice(&self, b: impl std::ops::RangeBounds<usize>) -> &Mem {
        let start = start_from_bound(b.start_bound());
        let end = end_from_bound(self, b.end_bound());
        Mem::from_slice(&self.0[start..end])
    }
    fn slice_mut(&mut self, b: impl std::ops::RangeBounds<usize>) -> &mut Mem {
        let start = start_from_bound(b.start_bound());
        let end = end_from_bound(self, b.end_bound());
        Mem::from_slice_mut(&mut self.0[start..end])
    }

    fn view<T: Pod>(&self, addr: u32) -> &T {
        let ofs = addr as usize;
        let buf = &self.0[ofs..(ofs + size_of::<T>())];
        // Safety: the above slice has already verified bounds.
        unsafe { &*(buf.as_ptr() as *const T) }
    }
    fn view_mut<T: Pod>(&mut self, addr: u32) -> &mut T {
        let ofs = addr as usize;
        let buf = &mut self.0[ofs..(ofs + size_of::<T>())];
        // Safety: the above slice has already verified bounds.
        unsafe { &mut *(buf.as_mut_ptr() as *mut T) }
    }

    fn view_n<T: Pod>(&self, ofs: usize, count: usize) -> &[T] {
        let size = size_of::<T>() * count;
        if ofs + size > self.0.len() {
            panic!("out of bounds");
        }
        unsafe { std::slice::from_raw_parts(&self.0[ofs] as *const u8 as *const T, count) }
    }

    fn read_u32(&self, addr: u32) -> u32 {
        *self.view::<u32>(addr)
    }

    fn write_u32(&mut self, addr: u32, value: u32) {
        *self.view_mut::<u32>(addr) = value;
    }

    fn read_strz(&self) -> &str {
        read_strz(&self.0)
    }

    fn read_strz_with_nul(&self) -> &str {
        read_strz_with_nul(&self.0)
    }
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct VecMem(#[serde(with = "serde_bytes")] Vec<u8>);

impl VecMem {
    pub fn resize(&mut self, size: usize, value: u8) {
        self.0.resize(size, value);
    }
}

impl std::ops::Deref for VecMem {
    type Target = Mem;

    fn deref(&self) -> &Self::Target {
        Mem::from_slice(&self.0)
    }
}

impl std::ops::DerefMut for VecMem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        Mem::from_slice_mut(&mut self.0)
    }
}
