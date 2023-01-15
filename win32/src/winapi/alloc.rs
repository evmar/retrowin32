use crate::memory::Memory;

pub trait Alloc {
    fn alloc(&mut self, mem: &mut [u8], size: u32) -> u32;
    fn size(&self, mem: &[u8], addr: u32) -> u32;
    fn free(&self, mem: &[u8], addr: u32) -> u32;
}

pub struct Heap {
    pub addr: u32,
    pub size: u32,
    next: u32,
}
impl Heap {
    pub fn new(addr: u32, size: u32) -> Self {
        Heap {
            addr,
            size,
            next: 0,
        }
    }

    pub fn alloc(&mut self, mem: &mut [u8], size: u32) -> u32 {
        let alloc_size = size + 4; // TODO: align?
        if self.next + alloc_size > self.size {
            log::error!(
                "Heap::alloc cannot allocate {:x}, using {:x}/{:x}",
                size,
                self.size - self.next,
                self.size
            );
            return 0;
        }
        let addr = self.addr + self.next;
        mem.write_u32(addr, size);
        self.next += alloc_size;
        addr + 4
    }

    pub fn size(&self, mem: &[u8], addr: u32) -> u32 {
        assert!(addr >= self.addr + 4 && addr < self.addr + self.size);
        mem.read_u32(addr - 4)
    }
}
