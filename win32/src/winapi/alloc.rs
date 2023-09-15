use memory::Mem;

pub trait Alloc {
    fn alloc(&mut self, size: u32) -> u32;
    fn size(&self, addr: u32) -> u32;
    fn free(&mut self, addr: u32);
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ArenaInfo {
    pub addr: u32,
    pub size: u32,
    next: u32,
}
impl ArenaInfo {
    pub fn new(addr: u32, size: u32) -> Self {
        ArenaInfo {
            addr,
            size,
            next: 0,
        }
    }
    pub fn get<'a, 'm>(&'a mut self, mem: Mem<'m>) -> Arena<'a, 'm> {
        Arena { info: self, mem }
    }
}

pub struct Arena<'a, 'm> {
    info: &'a mut ArenaInfo,
    mem: Mem<'m>,
}

pub fn align32(n: u32) -> u32 {
    (n + 3) & !3
}

impl<'a, 'm> Alloc for Arena<'a, 'm> {
    fn alloc(&mut self, size: u32) -> u32 {
        let alloc_size = align32(size + 4);
        if self.info.next + alloc_size > self.info.size {
            log::error!(
                "Arena::alloc cannot allocate {:x}, using {:x}/{:x}",
                size,
                self.info.size - self.info.next,
                self.info.size
            );
            return 0;
        }
        let addr = self.info.addr + self.info.next;
        self.mem.put::<u32>(addr, size);
        self.info.next += alloc_size;
        addr + 4
    }

    fn size(&self, addr: u32) -> u32 {
        assert!(addr >= self.info.addr + 4 && addr < self.info.addr + self.info.size);
        self.mem.get::<u32>(addr - 4)
    }

    fn free(&mut self, _addr: u32) {
        unimplemented!("can't free from arena")
    }
}
