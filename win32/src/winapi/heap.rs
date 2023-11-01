//! Heap memory allocator. Used for win32 API HeapCreate() etc. implementation
//! and also for win32-visible allocations created by other calls (like in
//! DirectDraw).

use super::alloc::align_to;
use memory::Mem;

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
        mem.put::<u32>(addr, size);
        addr + 4
    }

    pub fn size(&self, mem: Mem, addr: u32) -> u32 {
        mem.get::<u32>(addr - 4)
    }

    pub fn free(&mut self, mem: Mem, addr: u32) {
        let addr = addr - 4;
        let size = mem.get::<u32>(addr);

        let next_i = self
            .freelist
            .iter()
            .position(|f| f.addr > addr)
            .unwrap_or(self.freelist.len());

        let mut joined = false;
        if next_i > 0 {
            let prev_i = next_i - 1;
            let prev = &mut self.freelist[prev_i];
            if prev.addr + prev.size == addr {
                prev.size += size;
                joined = true;
            }
        }

        let next = &mut self.freelist[next_i];
        if addr + size == next.addr {
            if joined {
                let join_size = next.size;
                let prev = &mut self.freelist[next_i - 1];
                prev.size += join_size;
                log::warn!("freelist chunk2 {}", self.freelist.len());
                self.freelist.remove(next_i);
                return;
            }
            next.addr -= size;
            next.size += size;
            return;
        }

        if !joined {
            let free = FreeNode { addr, size };
            self.freelist.insert(next_i, free);
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct FreeNode {
    addr: u32,
    size: u32,
}
