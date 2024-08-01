//! Heap memory allocator. Used for win32 API HeapCreate() etc. implementation
//! and also for win32-visible allocations created by other calls (like in
//! DirectDraw).

use super::alloc::align_to;
use memory::{Extensions, Mem};

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct Heap {
    pub addr: u32,
    pub size: u32,
    freelist: Vec<FreeNode>,
}

impl Heap {
    pub fn new(addr: u32, size: u32) -> Self {
        let freelist = vec![FreeNode { addr, size }];
        Heap {
            addr,
            size,
            freelist,
        }
    }

    fn range(&self) -> std::ops::Range<u32> {
        self.addr..self.addr + self.size
    }

    pub fn alloc(&mut self, mem: Mem, size: u32) -> u32 {
        let size = align_to(size, 4) + 4;
        let i = self
            .freelist
            .iter()
            .position(|f| f.size >= size)
            .unwrap_or_else(|| panic!("heap size {:x} oom {:x}", self.size, size));
        let free = &mut self.freelist[i];
        let addr = free.addr;
        free.size -= size;
        free.addr += size;
        if free.size == 0 {
            self.freelist.remove(i);
        }
        mem.put_pod::<u32>(addr, size);
        addr + 4
    }

    pub fn size(&self, mem: Mem, addr: u32) -> u32 {
        mem.get_pod::<u32>(addr - 4) - 4
    }

    pub fn free(&mut self, mem: Mem, addr: u32) {
        let addr = addr - 4;
        let size = mem.get_pod::<u32>(addr);

        if !self.range().contains(&addr) {
            panic!("free of addr not on heap");
        }

        let mut insert_index = 0;
        for (i, node) in self.freelist.iter().enumerate() {
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
            let prev = &mut self.freelist[prev_i];
            if prev.addr + prev.size == addr {
                prev.size += size;
                joined = true;
            }
        }

        if insert_index < self.freelist.len() {
            // Check if merging with later block.
            let next = &mut self.freelist[insert_index];
            if addr + size == next.addr {
                if joined {
                    let next_size = next.size;
                    let prev = &mut self.freelist[insert_index - 1];
                    prev.size += next_size;
                    self.freelist.remove(insert_index);
                } else {
                    next.addr -= size;
                    next.size += size;
                    joined = true;
                }
            }
        }

        if !joined {
            let free = FreeNode { addr, size };
            self.freelist.insert(insert_index, free);
        }
    }
}

/// Entry in the FreeList.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct FreeNode {
    addr: u32,
    size: u32,
}

impl FreeNode {
    fn range(&self) -> std::ops::Range<u32> {
        self.addr..self.addr + self.size
    }
}
