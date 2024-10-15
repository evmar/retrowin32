pub fn align_to(n: u32, align: usize) -> u32 {
    // log2(align) - 1
    let add = match align {
        1 => return n,
        2 => 1,
        4 => 3,
        8 => 7,
        _ => todo!("{align}"),
    };
    (n + add) & !add
}

pub struct Arena {
    pub addr: u32,
    pub size: u32,
    /// Offset within the arena for the next allocation.
    next: u32,
}

impl Arena {
    pub fn new(addr: u32, size: u32) -> Self {
        assert!(addr == align_to(addr, 8));
        Arena {
            addr,
            size,
            next: 0,
        }
    }

    pub fn alloc(&mut self, size: u32, align: usize) -> u32 {
        let next = align_to(self.next, align);
        if next + size > self.size {
            log::error!(
                "Arena::alloc cannot allocate {:x}, using {:x}/{:x}",
                size,
                self.size - self.next,
                self.size
            );
            return 0;
        }
        self.next = next + size;
        self.addr + next
    }
}
