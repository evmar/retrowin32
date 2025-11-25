use crate::Mem;

// TODO: Unicorn needs this to be Pin.
// In theory x86-emu could work with it not being pinned, not sure.
// Maybe make a different impl?
pub struct BoxMem(Box<[u8]>);

impl BoxMem {
    pub fn new(size: usize) -> Self {
        let mut buf = Vec::<u8>::with_capacity(size);
        unsafe {
            buf.set_len(size);
        }
        Self(buf.into_boxed_slice())
    }

    // TODO: we can support growing under wasm by using a custom allocator that
    // ensures this is the last thing in the heap.
    // pub fn grow();

    pub fn len(&self) -> u32 {
        self.0.len() as u32
    }

    pub fn mem(&self) -> Mem<'_> {
        Mem::from_slice(&self.0)
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.0.as_ptr()
    }
}
