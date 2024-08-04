use crate::{
    machine::{Machine, MemImpl},
    pe::ImageSectionFlags,
    winapi::stack_args,
};
use bitflags::bitflags;
use memory::{Extensions, ExtensionsMut, Mem};
use std::cmp::max;

const TRACE_CONTEXT: &'static str = "kernel32/memory";

pub fn round_up_to_page_granularity(size: u32) -> u32 {
    size + (0x1000 - 1) & !(0x1000 - 1)
}

/// Memory span as managed by the kernel.  Some come from the exe and others are allocated dynamically.
#[derive(Debug, serde::Serialize)]
#[cfg_attr(feature = "wasm", derive(tsify::Tsify))]
pub struct Mapping {
    pub addr: u32,
    pub size: u32,
    pub desc: String,
    pub flags: ImageSectionFlags,
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
            desc: "avoid null pointers".into(),
            flags: ImageSectionFlags::empty(),
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
            let prev = &mut self.0[pos - 1];
            if prev.addr + prev.size > mapping.addr {
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
            if space > size {
                break;
            }
            prev_end = mapping.addr + mapping.size;
        }
        prev_end
    }

    pub fn alloc(&mut self, size: u32, desc: String, mem: &mut MemImpl) -> &Mapping {
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
            desc,
            flags: ImageSectionFlags::empty(),
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

bitflags! {
    #[derive(Default)]
    pub struct HeapAllocFlags: u32 {
        const HEAP_GENERATE_EXCEPTIONS = 0x4;
        const HEAP_NO_SERIALIZE = 0x1;
        const HEAP_ZERO_MEMORY = 0x8;
    }
}
impl TryFrom<u32> for HeapAllocFlags {
    type Error = u32;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        HeapAllocFlags::from_bits(value).ok_or(value)
    }
}

#[win32_derive::dllexport]
pub fn HeapAlloc(
    machine: &mut Machine,
    hHeap: u32,
    dwFlags: Result<HeapAllocFlags, u32>,
    dwBytes: u32,
) -> u32 {
    let mut flags = dwFlags.unwrap_or_else(|_| {
        log::warn!("HeapAlloc invalid flags {dwFlags:x?}");
        HeapAllocFlags::empty()
    });
    flags.remove(HeapAllocFlags::HEAP_GENERATE_EXCEPTIONS); // todo: OOM
    flags.remove(HeapAllocFlags::HEAP_NO_SERIALIZE); // todo: threads
    let heap = match machine.state.kernel32.get_heap(hHeap) {
        None => {
            log::error!("HeapAlloc({hHeap:x}): no such heap");
            return 0;
        }
        Some(heap) => heap,
    };
    let addr = heap.alloc(machine.emu.memory.mem(), dwBytes);
    if addr == 0 {
        log::warn!("HeapAlloc({hHeap:x}) failed");
    }
    if flags.contains(HeapAllocFlags::HEAP_ZERO_MEMORY) {
        machine.mem().sub32_mut(addr, dwBytes).fill(0);
        flags.remove(HeapAllocFlags::HEAP_ZERO_MEMORY);
    }
    if !flags.is_empty() {
        log::error!("HeapAlloc: unhandled flags {flags:?}");
    }
    addr
}

#[win32_derive::dllexport]
pub fn HeapFree(machine: &mut Machine, hHeap: u32, dwFlags: u32, lpMem: u32) -> bool {
    if dwFlags != 0 {
        log::warn!("HeapFree flags {dwFlags:x}");
    }
    machine
        .state
        .kernel32
        .get_heap(hHeap)
        .unwrap()
        .free(machine.emu.memory.mem(), lpMem);
    true
}

#[win32_derive::dllexport]
pub fn HeapSize(machine: &mut Machine, hHeap: u32, dwFlags: u32, lpMem: u32) -> u32 {
    if dwFlags != 0 {
        log::warn!("HeapSize flags {dwFlags:x}");
    }
    let heap = match machine.state.kernel32.get_heap(hHeap) {
        None => {
            log::error!("HeapSize({hHeap:x}): no such heap");
            return 0;
        }
        Some(heap) => heap,
    };
    heap.size(machine.emu.memory.mem(), lpMem)
}

#[win32_derive::dllexport]
pub fn HeapSetInformation(
    _machine: &mut Machine,
    HeapHandle: u32,
    HeapInformationClass: u32,
    HeapInformation: u32,
    HeapInformationLength: u32,
) -> bool {
    true
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
    let heap = match machine.state.kernel32.get_heap(hHeap) {
        None => {
            log::error!("HeapSize({hHeap:x}): no such heap");
            return 0;
        }
        Some(heap) => heap,
    };
    let old_size = heap.size(machine.emu.memory.mem(), lpMem);
    let new_addr = heap.alloc(machine.emu.memory.mem(), dwBytes);
    assert!(dwBytes >= old_size);
    heap.free(machine.emu.memory.mem(), lpMem);
    machine.mem().copy(lpMem, new_addr, old_size);
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
    let size = max(dwInitialSize as usize, 20 << 20);
    machine
        .state
        .kernel32
        .new_heap(&mut machine.emu.memory, size, "HeapCreate".into())
}

#[win32_derive::dllexport]
pub fn HeapDestroy(_machine: &mut Machine, hHeap: u32) -> u32 {
    log::warn!("HeapDestroy({hHeap:x})");
    1 // success
}

bitflags! {
    pub struct MEM: u32 {
        const COMMIT = 0x00001000;
        const RESERVE = 0x00002000;
        const RESET = 0x00080000;
        const RESET_UNDO = 0x1000000;
        const LARGE_PAGES = 0x20000000;
        const PHYSICAL = 0x00400000;
        const TOP_DOWN = 0x00100000;
        const WRITE_WATCH = 0x00200000;
    }
}
impl TryFrom<u32> for MEM {
    type Error = u32;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        MEM::from_bits(value).ok_or(value)
    }
}

bitflags! {
    pub struct PAGE: u32 {
        const EXECUTE = 0x10;
        const EXECUTE_READ = 0x20;
        const EXECUTE_READWRITE = 0x40;
        const EXECUTE_WRITECOPY = 0x80;
        const NOACCESS = 0x01;
        const READONLY = 0x02;
        const READWRITE = 0x04;
        const WRITECOPY = 0x08;
        const TARGETS_INVALID = 0x40000000;
        const TARGETS_NO_UPDATE = 0x40000000;
        const GUARD = 0x100;
        const NOCACHE = 0x200;
        const WRITECOMBINE = 0x400;
    }
}
impl TryFrom<u32> for PAGE {
    type Error = u32;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        PAGE::from_bits(value).ok_or(value)
    }
}

#[win32_derive::dllexport]
pub fn VirtualAlloc(
    machine: &mut Machine,
    lpAddress: u32,
    dwSize: u32,
    flAllocationType: Result<MEM, u32>,
    flProtec: Result<PAGE, u32>,
) -> u32 {
    if lpAddress != 0 {
        // Changing flags on an existing address, hopefully.
        match machine
            .state
            .kernel32
            .mappings
            .vec()
            .iter()
            .find(|&mapping| mapping.contains(lpAddress))
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

    let mapping = machine.state.kernel32.mappings.alloc(
        dwSize,
        "VirtualAlloc".into(),
        &mut machine.emu.memory,
    );
    mapping.addr
}

#[derive(Debug)]
pub struct MEMORY_BASIC_INFORMATION {
    pub BaseAddress: u32,
    pub AllocationBase: u32,
    pub AllocationProtect: u32,
    pub PartitionId: u16,
    pub RegionSize: u32,
    pub State: u32,
    pub Protect: u32,
    pub Type: u32,
}
unsafe impl memory::Pod for MEMORY_BASIC_INFORMATION {}

#[win32_derive::dllexport]
pub fn VirtualQuery(
    _machine: &mut Machine,
    lpAddress: u32,
    lpBuffer: Option<&mut MEMORY_BASIC_INFORMATION>,
    dwLength: u32,
) -> u32 {
    0 // fail
}

#[win32_derive::dllexport]
pub fn VirtualFree(_machine: &mut Machine, lpAddress: u32, dwSize: u32, dwFreeType: u32) -> u32 {
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

bitflags! {
    pub struct GMEM: u32 {
        const MOVEABLE = 0x2;
        const ZEROINIT = 0x40;
        const MODIFY = 0x80;
    }
}
impl<'a> stack_args::FromArg<'a> for GMEM {
    unsafe fn from_arg(_mem: Mem<'a>, arg: u32) -> Self {
        // GlobalAlloc accepted many flags, but most are obsolete, so ignore
        // anything other than the flags we have named.
        GMEM::from_bits_truncate(arg)
    }
}

#[win32_derive::dllexport]
pub fn GlobalAlloc(machine: &mut Machine, uFlags: GMEM, dwBytes: u32) -> u32 {
    if uFlags.contains(GMEM::MOVEABLE) {
        todo!("GMEM_MOVEABLE");
    }
    let heap = machine
        .state
        .kernel32
        .get_process_heap(&mut machine.emu.memory); // lazy init process_heap
    let addr = heap.alloc(machine.emu.memory.mem(), dwBytes);
    if uFlags.contains(GMEM::ZEROINIT) {
        machine.mem().sub32_mut(addr, dwBytes).fill(0);
    }
    addr
}

#[win32_derive::dllexport]
pub fn GlobalReAlloc(machine: &mut Machine, hMem: u32, dwBytes: u32, uFlags: GMEM) -> u32 {
    if uFlags.contains(GMEM::MODIFY) {
        todo!("GMEM_MODIFY");
    }
    let heap = machine
        .state
        .kernel32
        .get_process_heap(&mut machine.emu.memory);
    let mem = machine.emu.memory.mem();
    let old_size = heap.size(mem, hMem);
    if dwBytes <= old_size {
        return hMem;
    }
    let addr = heap.alloc(mem, dwBytes);
    mem.copy(hMem, addr, old_size);
    heap.free(mem, hMem);
    if uFlags.contains(GMEM::ZEROINIT) {
        mem.sub32_mut(addr + old_size, dwBytes - old_size).fill(0);
    }
    addr
}

#[win32_derive::dllexport]
pub fn GlobalFree(machine: &mut Machine, hMem: u32) -> u32 {
    let heap = machine
        .state
        .kernel32
        .get_process_heap(&mut machine.emu.memory);
    heap.free(machine.emu.memory.mem(), hMem);
    return 0; // success
}

#[win32_derive::dllexport]
pub fn GlobalFlags(_machine: &mut Machine, hMem: u32) -> u32 {
    0 // stub
}

#[win32_derive::dllexport]
pub fn LocalAlloc(machine: &mut Machine, uFlags: GMEM, dwBytes: u32) -> u32 {
    GlobalAlloc(machine, uFlags, dwBytes)
}

#[win32_derive::dllexport]
pub fn LocalFree(machine: &mut Machine, hMem: u32) -> u32 {
    GlobalFree(machine, hMem)
}

#[win32_derive::dllexport]
pub fn VirtualProtect(
    _machine: &mut Machine,
    lpAddress: u32,
    dwSize: u32,
    flNewProtect: u32,
    lpflOldProtect: Option<&mut u32>,
) -> bool {
    true // success
}

#[win32_derive::dllexport]
pub fn GetProcessHeap(machine: &mut Machine) -> u32 {
    machine
        .state
        .kernel32
        .get_process_heap(&mut machine.emu.memory); // lazy init process_heap
    machine.state.kernel32.process_heap
}
