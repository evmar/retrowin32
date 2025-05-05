//! Heap memory allocator. Used for win32 API HeapCreate() etc. implementation
//! and also for win32-visible allocations created by other calls (like in
//! DirectDraw).

use std::cell::RefCell;

use memory::{Extensions, ExtensionsMut, Mem};

use crate::memory::align_to;

pub struct Heap {
    pub addr: u32,
    pub size: u32,
    freelist: RefCell<FreeList>,
}

impl Heap {
    pub fn new(addr: u32, size: u32) -> Self {
        Heap {
            addr,
            size,
            freelist: RefCell::new(FreeList::new(addr, size)),
        }
    }

    fn range(&self) -> std::ops::Range<u32> {
        self.addr..self.addr + self.size
    }

    pub fn alloc(&self, mem: Mem, size: u32) -> u32 {
        self.freelist
            .borrow_mut()
            .alloc(mem, size)
            .unwrap_or_else(|| panic!("heap size {:x} oom {:x}", self.size, size))
    }

    pub fn size(&self, mem: Mem, addr: u32) -> u32 {
        mem.get_pod::<u32>(addr - 4) - 4
    }

    pub fn free(&self, mem: Mem, addr: u32) {
        if !self.range().contains(&(addr - 4)) {
            log::error!("free of addr not on heap");
            return;
        }
        self.freelist.borrow_mut().free(mem, addr);
    }
}

// TODO: remove this
impl Default for Heap {
    fn default() -> Self {
        Heap::new(0, 0)
    }
}

struct FreeList {
    nodes: Vec<FreeNode>,
}

impl FreeList {
    fn new(addr: u32, size: u32) -> Self {
        FreeList {
            nodes: vec![FreeNode { addr, size }],
        }
    }

    fn alloc(&mut self, mem: Mem, size: u32) -> Option<u32> {
        let size = align_to(size, 4) + 4;
        let i = self.nodes.iter().position(|f| f.size >= size)?;
        let free = &mut self.nodes[i];
        let addr = free.addr;
        free.size -= size;
        free.addr += size;
        if free.size == 0 {
            self.nodes.remove(i);
        }
        mem.put_pod::<u32>(addr, size);
        Some(addr + 4)
    }

    fn free(&mut self, mem: Mem, addr: u32) {
        let addr = addr - 4;
        let size = mem.get_pod::<u32>(addr);

        let mut insert_index = 0;
        for (i, node) in self.nodes.iter().enumerate() {
            if node.range().contains(&addr) {
                // address is within already free block
                log::warn!("ignoring double free");
                return;
            }
            if node.addr > addr {
                insert_index = i;
                break;
            }
        }

        let mut joined = false;
        if insert_index > 0 {
            // Check if merging with earlier block.
            let prev_i = insert_index - 1;
            let prev = &mut self.nodes[prev_i];
            if prev.addr + prev.size == addr {
                prev.size += size;
                joined = true;
            }
        }

        if insert_index < self.nodes.len() {
            // Check if merging with later block.
            let next = &mut self.nodes[insert_index];
            if addr + size == next.addr {
                if joined {
                    let next_size = next.size;
                    let prev = &mut self.nodes[insert_index - 1];
                    prev.size += next_size;
                    self.nodes.remove(insert_index);
                } else {
                    next.addr -= size;
                    next.size += size;
                    joined = true;
                }
            }
        }

        if !joined {
            let free = FreeNode { addr, size };
            self.nodes.insert(insert_index, free);
        }
    }
}

/// Entry in the FreeList.
#[derive(Debug)]
struct FreeNode {
    addr: u32,
    size: u32,
}

impl FreeNode {
    fn range(&self) -> std::ops::Range<u32> {
        self.addr..self.addr + self.size
    }
}
