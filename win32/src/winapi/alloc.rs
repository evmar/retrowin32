use crate::memory::{Memory, Pod};

pub trait Alloc {
    fn alloc(&mut self, mem: &mut [u8], size: u32) -> u32;
    fn size(&self, mem: &[u8], addr: u32) -> u32;
    fn free(&mut self, mem: &mut [u8], addr: u32);
}

pub struct Arena {
    pub addr: u32,
    pub size: u32,
    next: u32,
}
impl Arena {
    pub fn new(addr: u32, size: u32) -> Self {
        Arena {
            addr,
            size,
            next: 0,
        }
    }
}
impl Alloc for Arena {
    fn alloc(&mut self, mem: &mut [u8], size: u32) -> u32 {
        let alloc_size = size + 4; // TODO: align?
        if self.next + alloc_size > self.size {
            log::error!(
                "Arena::alloc cannot allocate {:x}, using {:x}/{:x}",
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

    fn size(&self, mem: &[u8], addr: u32) -> u32 {
        assert!(addr >= self.addr + 4 && addr < self.addr + self.size);
        mem.read_u32(addr - 4)
    }

    fn free(&mut self, _mem: &mut [u8], _addr: u32) {
        unimplemented!("can't free from arena")
    }
}

pub struct Heap {
    pub addr: u32,
    pub size: u32,
    /// Pointer to first free block: head of the FreeNode list.
    free: u32,
}

impl Heap {
    pub fn new(mem: &mut [u8], addr: u32, size: u32) -> Self {
        *FreeNode::get(mem, addr) = FreeNode { size, next: 0 };
        Heap {
            addr,
            size,
            free: addr,
        }
    }

    /// Attempt to coalesce the freelist node at addr with any subsequent
    /// adjacent blocks of free memory.
    fn try_coalesce(&mut self, mem: &mut [u8], addr: u32) {
        loop {
            let FreeNode { next, size } = *FreeNode::get(mem, addr);
            if next != addr + size {
                break;
            }
            let next = FreeNode::get(mem, next);
            *FreeNode::get(mem, addr) = FreeNode {
                next: next.next,
                size: size + next.size,
            }
        }
    }
}

#[derive(Debug)]
#[repr(C)]
struct FreeNode {
    size: u32,
    /// Pointer to next node.
    next: u32,
}
unsafe impl Pod for FreeNode {}
impl FreeNode {
    fn get(mem: &mut [u8], addr: u32) -> &mut Self {
        mem.view_mut::<FreeNode>(addr)
    }
}

impl Alloc for Heap {
    fn alloc(&mut self, mem: &mut [u8], size: u32) -> u32 {
        let alloc_size = size + 4;

        // Find a FreeNode large enough to accommodate alloc_size.
        // To use it, update the previous node to point past it.
        let mut prev = 0;
        let mut cur = self.free;
        let mut blocks = 0;
        while cur != 0 {
            blocks += 1;
            let node = FreeNode::get(mem, cur);
            if node.size >= alloc_size {
                break;
            }
            prev = cur;
            cur = node.next;
        }
        if cur == 0 {
            panic!("heap OOM allocating {alloc_size:#x} freelist {blocks} entries");
        }

        // Find the pointer to the point after the allocated block.
        let next = if FreeNode::get(mem, cur).size > alloc_size + 8 {
            // Split cur block into smaller piece; create a new FreeNode in
            // the remaining space.
            let next = cur + alloc_size;
            let cur = FreeNode::get(mem, cur);
            *FreeNode::get(mem, next) = FreeNode {
                size: cur.size - alloc_size,
                next: cur.next,
            };
            next
        } else {
            FreeNode::get(mem, cur).next
        };

        // Link next node into the list.
        if prev == 0 {
            self.free = next;
        } else {
            FreeNode::get(mem, prev).next = next;
        }

        mem.write_u32(cur, size);
        cur + 4
    }

    fn size(&self, mem: &[u8], addr: u32) -> u32 {
        mem.read_u32(addr - 4)
    }

    fn free(&mut self, mem: &mut [u8], addr: u32) {
        let free_size = self.size(mem, addr) + 4;
        let addr = addr - 4;

        let mut prev = 0;
        let mut next = self.free;
        while next < addr {
            prev = next;
            next = FreeNode::get(mem, next).next;
        }

        // Insert freelist node at addr.
        *FreeNode::get(mem, addr) = FreeNode {
            next,
            size: free_size,
        };
        if prev > 0 {
            FreeNode::get(mem, prev).next = addr;
            self.try_coalesce(mem, prev);
        } else {
            self.free = addr;
            self.try_coalesce(mem, addr);
        }
    }
}
