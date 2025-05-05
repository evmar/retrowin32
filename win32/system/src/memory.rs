use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{MemImpl, heap::Heap};
use memory::{Extensions, Mem};

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

pub fn round_up_to_page_granularity(size: u32) -> u32 {
    size + (0x1000 - 1) & !(0x1000 - 1)
}

/// Memory span.  Some come from the exe and others are allocated dynamically.
#[derive(Debug, serde::Serialize)]
#[cfg_attr(target_family = "wasm", derive(tsify::Tsify))]
pub struct Mapping {
    pub addr: u32,
    pub size: u32,
    pub module: Option<String>,
    pub desc: String,
    pub flags: pe::IMAGE_SCN,
}

impl Mapping {
    pub fn contains(&self, addr: u32) -> bool {
        addr >= self.addr && addr < self.addr + self.size
    }
}

/// The set of Mappings managed by the kernel.
/// These get visualized in the debugger when you hover a pointer.
#[derive(serde::Serialize, Debug)]
pub struct Mappings(Vec<Mapping>);
impl Mappings {
    pub fn new() -> Self {
        Mappings(vec![Mapping {
            addr: 0,
            size: 0x1000,
            module: None,
            desc: "avoid null pointers".into(),
            flags: pe::IMAGE_SCN::empty(),
        }])
    }

    pub fn add(&mut self, mut mapping: Mapping) -> &Mapping {
        mapping.size = round_up_to_page_granularity(mapping.size);
        let pos = self
            .0
            .iter()
            .position(|m| m.addr > mapping.addr)
            .unwrap_or(self.0.len());
        if pos > 0 {
            let prev = &self.0[pos - 1];
            if prev.addr + prev.size > mapping.addr {
                self.dump();
                panic!("mapping conflict loading {mapping:x?} conflicts with {prev:x?}",);
            }
        }
        if pos < self.0.len() {
            let next = &self.0[pos];
            assert!(mapping.addr + mapping.size <= next.addr);
        }
        self.0.insert(pos, mapping);
        &self.0[pos]
    }

    /// Find an address where we can create a new mapping of given size.
    pub fn find_space(&self, size: u32) -> u32 {
        let size = round_up_to_page_granularity(size);
        let mut prev_end = 0;
        for mapping in &self.0 {
            let space = mapping.addr - prev_end;
            if space >= size {
                break;
            }
            prev_end = mapping.addr + mapping.size;
        }
        prev_end
    }

    pub fn alloc(&mut self, mem: Mem, size: u32, desc: String) -> &Mapping {
        let size = round_up_to_page_granularity(size);
        if size > 32 << 20 {
            panic!("new mapping {:?} too large: {size:x} bytes", desc);
        }
        let addr = self.find_space(size);
        if addr + size > mem.len() {
            panic!(
                "not enough memory reserved, need at least {}mb",
                (addr + size) >> 20
            );
        }
        self.add(Mapping {
            addr,
            size,
            module: None,
            desc,
            flags: pe::IMAGE_SCN::empty(),
        })
    }

    pub fn vec(&self) -> &Vec<Mapping> {
        &self.0
    }

    pub fn grow(&mut self, addr: u32, min_growth: u32) -> u32 {
        let pos = self.0.iter().position(|m| m.addr == addr).unwrap();
        let mapping = &self.0[pos];
        let mut new_size = mapping.size;
        while new_size - mapping.size < min_growth {
            new_size *= 2;
        }

        // Check if we run into a mapping after this one.
        if pos + 1 < self.0.len() {
            let next = &self.0[pos + 1];
            if mapping.addr + new_size > next.addr {
                panic!("cannot grow {:?}", mapping);
            }
        }

        let mapping = &mut self.0[pos];
        let growth = new_size - mapping.size;
        mapping.size = new_size;
        log::info!(
            "grew mapping {:?} by {:#x}, new size {:#x}",
            mapping.desc,
            growth,
            new_size
        );
        log::warn!("might need to grow backing memory after growth");
        growth
    }

    pub fn dump(&self) {
        for map in &self.0 {
            println!(
                "{:08x}-{:08x} {:?} {:?}",
                map.addr,
                map.addr + map.size,
                map.desc,
                map.flags
            );
        }
    }

    pub fn dump_memory(&self, mem: Mem) {
        for map in &self.0 {
            println!("{map:x?}");
            for addr in (map.addr..map.addr + map.size).step_by(16) {
                println!("{addr:x} {:x?}", mem.sub32(addr, 16));
            }
        }
    }
}

pub struct Memory {
    pub imp: MemImpl,
    pub mappings: Mappings,
    pub labels: HashMap<u32, String>,

    pub heaps: HashMap<u32, Rc<RefCell<Heap>>>,
    /// The "process heap" is a per-process default heap exposed via GetProcessHeap and used
    /// by default.
    /// We also use it for our own random allocations, e.g. buffers allocated by other APIs.
    pub process_heap: Rc<RefCell<Heap>>,
}

impl Memory {
    pub fn new(imp: MemImpl) -> Self {
        Memory {
            imp,
            mappings: Mappings::new(),
            labels: Default::default(),
            heaps: Default::default(),
            process_heap: Rc::new(RefCell::new(Heap::default())),
        }
    }

    pub fn len(&self) -> u32 {
        self.imp.len()
    }

    pub fn mem(&self) -> Mem {
        self.imp.mem()
    }

    pub fn new_heap(&mut self, size: usize, desc: String) -> Rc<RefCell<Heap>> {
        let mapping = self.mappings.alloc(self.imp.mem(), size as u32, desc);
        let heap = Rc::new(RefCell::new(Heap::new(mapping.addr, mapping.size)));
        self.heaps.insert(mapping.addr, heap.clone());
        heap
    }
}
