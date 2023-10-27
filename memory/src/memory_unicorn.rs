use crate::Mem;

pub struct MemImpl(Box<[u8]>);

impl MemImpl {
    pub fn new(size: usize) -> Self {
        let mut v = Vec::with_capacity(size);
        v.resize(size, 0);
        Self(v.into_boxed_slice())
    }

    pub fn resize(&mut self, size: u32, _value: u8) {
        unimplemented!("resize memory {size:x}")
    }
    pub fn len(&self) -> u32 {
        self.0.len() as u32
    }
    pub fn mem(&self) -> Mem {
        Mem::from_slice(&self.0)
    }

    pub fn ptr(&mut self) -> *mut u8 {
        self.0.as_mut_ptr()
    }
}
