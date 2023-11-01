use memory::Mem;

pub fn align_to(n: u32, align: usize) -> u32 {
    // log2(align) - 1
    let add = match align {
        1 | 2 | 4 => 3, // still need to align to 4-byte boundary for metadata purposes
        8 => 7,
        _ => todo!("{align}"),
    };
    (n + add) & !add
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

impl<'a, 'm> Arena<'a, 'm> {
    pub fn alloc(&mut self, size: u32, align: usize) -> u32 {
        let next = align_to(self.info.next + 4, align);
        if next + size > self.info.size {
            log::error!(
                "Arena::alloc cannot allocate {:x}, using {:x}/{:x}",
                size,
                self.info.size - self.info.next,
                self.info.size
            );
            return 0;
        }
        let addr = self.info.addr + next;
        self.mem.put::<u32>(addr - 4, size);
        self.info.next = next + size;
        addr
    }
}
