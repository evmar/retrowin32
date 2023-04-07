use crate::{machine::Machine, pe::ImageSectionFlags, winapi::alloc::Alloc};
use bitflags::bitflags;
use std::cmp::max;
use tsify::Tsify;

const TRACE_CONTEXT: &'static str = "kernel32/memory";

pub fn round_up_to_page_granularity(size: u32) -> u32 {
    size + (0x1000 - 1) & !(0x1000 - 1)
}

/// Memory span as managed by the kernel.  Some come from the exe and others are allocated dynamically.
#[derive(Debug, tsify::Tsify, serde::Serialize, serde::Deserialize)]
pub struct Mapping {
    pub addr: u32,
    pub size: u32,
    pub desc: String,
    pub flags: ImageSectionFlags,
}

/// The set of Mappings managed by the kernel.
/// These get visualized in the debugger when you hover a pointer.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Mappings(Vec<Mapping>);
impl Mappings {
    pub fn new() -> Self {
        Mappings(vec![Mapping {
            addr: 0,
            size: x86::NULL_POINTER_REGION_SIZE,
            desc: "avoid null pointers".into(),
            flags: ImageSectionFlags::empty(),
        }])
    }

    pub fn add(&mut self, mut mapping: Mapping, truncate_previous: bool) -> &Mapping {
        mapping.size = round_up_to_page_granularity(mapping.size);
        let pos = self
            .0
            .iter()
            .position(|m| m.addr > mapping.addr)
            .unwrap_or(self.0.len());
        if pos > 0 {
            let prev = &mut self.0[pos - 1];
            if prev.addr + prev.size >= mapping.addr {
                if truncate_previous {
                    prev.size = mapping.addr - prev.addr;
                } else {
                    panic!("mapping conflict");
                }
            }
        }
        if pos < self.0.len() {
            let next = &self.0[pos];
            assert!(mapping.addr + mapping.size <= next.addr);
        }
        self.0.insert(pos, mapping);
        &self.0[pos]
    }

    pub fn alloc(&mut self, size: u32, desc: String, mem: &mut Vec<u8>) -> &Mapping {
        let size = round_up_to_page_granularity(size);
        if size > 20 << 20 {
            panic!("new mapping {:?} {size:x} bytes", desc);
        }
        let mut prev_end = 0;
        let pos = self
            .0
            .iter()
            .position(|mapping| {
                let space = mapping.addr - prev_end;
                if space > size {
                    return true;
                }
                prev_end = mapping.addr + mapping.size;
                false
            })
            .unwrap_or_else(|| {
                let space = if mem.len() as u32 > prev_end {
                    mem.len() as u32 - prev_end
                } else {
                    0
                };
                if space < size {
                    let new_size = (prev_end + size) as usize;
                    mem.resize(new_size, 0);
                }
                self.0.len()
            });

        self.0.insert(
            pos,
            Mapping {
                addr: prev_end,
                size,
                desc,
                flags: ImageSectionFlags::empty(),
            },
        );
        &self.0[pos]
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
}

bitflags! {
    pub struct HeapAllocFlags: u32 {
        const HEAP_GENERATE_EXCEPTIONS = 0x4;
        const HEAP_NO_SERIALIZE = 0x1;
        const HEAP_ZERO_MEMORY = 0x8;
    }
}

#[win32_derive::dllexport]
pub fn HeapAlloc(machine: &mut Machine, hHeap: u32, dwFlags: u32, dwBytes: u32) -> u32 {
    let mut flags = HeapAllocFlags::from_bits(dwFlags).unwrap_or_else(|| {
        log::warn!("HeapAlloc invalid flags {dwFlags:x}");
        HeapAllocFlags::empty()
    });
    flags.remove(HeapAllocFlags::HEAP_GENERATE_EXCEPTIONS); // todo: OOM
    flags.remove(HeapAllocFlags::HEAP_NO_SERIALIZE); // todo: threads
    let mut heap = match machine.state.kernel32.get_heap(&mut machine.x86.mem, hHeap) {
        None => {
            log::error!("HeapAlloc({hHeap:x}): no such heap");
            return 0;
        }
        Some(heap) => heap,
    };
    let addr = heap.alloc(dwBytes);
    if addr == 0 {
        log::warn!("HeapAlloc({hHeap:x}) failed");
    }
    if flags.contains(HeapAllocFlags::HEAP_ZERO_MEMORY) {
        machine.x86.mem[addr as usize..(addr + dwBytes) as usize].fill(0);
        flags.remove(HeapAllocFlags::HEAP_ZERO_MEMORY);
    }
    if !flags.is_empty() {
        log::error!("HeapAlloc: unhandled flags {flags:?}");
    }
    addr
}

#[win32_derive::dllexport]
pub fn HeapFree(machine: &mut Machine, hHeap: u32, dwFlags: u32, lpMem: u32) -> u32 {
    if dwFlags != 0 {
        log::warn!("HeapFree flags {dwFlags:x}");
    }
    let mut heap = match machine.state.kernel32.get_heap(&mut machine.x86.mem, hHeap) {
        None => {
            log::error!("HeapFree({hHeap:x}): no such heap");
            return 0;
        }
        Some(heap) => heap,
    };
    heap.free(lpMem);
    1 // success
}

#[win32_derive::dllexport]
pub fn HeapSize(machine: &mut Machine, hHeap: u32, dwFlags: u32, lpMem: u32) -> u32 {
    if dwFlags != 0 {
        log::warn!("HeapSize flags {dwFlags:x}");
    }
    let heap = match machine.state.kernel32.get_heap(&mut machine.x86.mem, hHeap) {
        None => {
            log::error!("HeapSize({hHeap:x}): no such heap");
            return 0;
        }
        Some(heap) => heap,
    };
    heap.size(lpMem)
}

#[win32_derive::dllexport]
pub fn HeapReAlloc(
    machine: &mut Machine,
    hHeap: u32,
    dwFlags: u32,
    lpMem: u32,
    dwBytes: u32,
) -> u32 {
    if dwFlags != 0 {
        log::warn!("HeapReAlloc flags: {:x}", dwFlags);
    }
    let mut heap = match machine.state.kernel32.get_heap(&mut machine.x86.mem, hHeap) {
        None => {
            log::error!("HeapSize({hHeap:x}): no such heap");
            return 0;
        }
        Some(heap) => heap,
    };
    let old_size = heap.size(lpMem);
    let new_addr = heap.alloc(dwBytes);
    log::info!("realloc {lpMem:x}/{old_size:x} => {new_addr:x}/{dwBytes:x}");
    machine.x86.mem.copy_within(
        lpMem as usize..(lpMem + old_size) as usize,
        new_addr as usize,
    );
    new_addr
}

bitflags! {
    pub struct HeapCreateFlags: u32 {
        const HEAP_CREATE_ENABLE_EXECUTE = 0x00040000;
        const HEAP_GENERATE_EXCEPTIONS = 0x00000004;
        const HEAP_NO_SERIALIZE = 0x00000001;
    }
}
impl TryFrom<u32> for HeapCreateFlags {
    type Error = u32;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        HeapCreateFlags::from_bits(value).ok_or(value)
    }
}

#[win32_derive::dllexport]
pub fn HeapCreate(
    machine: &mut Machine,
    flOptions: Result<HeapCreateFlags, u32>,
    dwInitialSize: u32,
    dwMaximumSize: u32,
) -> u32 {
    flOptions.unwrap();
    // Currently none of the flags will affect behavior, but we might need to revisit this
    // with exceptions or threads support...
    let size = max(dwInitialSize as usize, 1 << 20);
    machine
        .state
        .kernel32
        .new_heap(&mut machine.x86.mem, size, "HeapCreate".into())
}

#[win32_derive::dllexport]
pub fn HeapDestroy(_machine: &mut Machine, hHeap: u32) -> u32 {
    log::warn!("HeapDestroy({hHeap:x})");
    1 // success
}

#[win32_derive::dllexport]
pub fn VirtualAlloc(
    machine: &mut Machine,
    lpAddress: u32,
    dwSize: u32,
    _flAllocationType: u32,
    _flProtec: u32,
) -> u32 {
    if lpAddress != 0 {
        // Changing flags on an existing address, hopefully.
        match machine
            .state
            .kernel32
            .mappings
            .vec()
            .iter()
            .find(|&mapping| mapping.addr == lpAddress)
        {
            None => {
                log::error!("failing VirtualAlloc({lpAddress:x}, ...) refers to unknown mapping");
                return 0;
            }
            Some(_) => {
                // adjusting flags on existing mapping, ignore.
                return lpAddress;
            }
        }
    }
    // TODO round dwSize to page boundary

    let mapping =
        machine
            .state
            .kernel32
            .mappings
            .alloc(dwSize, "VirtualAlloc".into(), &mut machine.x86.mem);
    mapping.addr
}

#[win32_derive::dllexport]
pub fn VirtualFree(_machine: &mut Machine, lpAddress: u32, dwSize: u32, dwFreeType: u32) -> u32 {
    log::warn!("VirtualFree({lpAddress:x}, {dwSize:x}, {dwFreeType:x})");
    1 // success
}

#[win32_derive::dllexport]
pub fn IsBadReadPtr(_machine: &mut Machine, lp: u32, ucb: u32) -> bool {
    false // all pointers are valid
}

#[win32_derive::dllexport]
pub fn IsBadWritePtr(_machine: &mut Machine, lp: u32, ucb: u32) -> bool {
    false // all pointers are valid
}
