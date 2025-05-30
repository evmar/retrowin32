use bitflags::bitflags;
use memory::{ExtensionsMut, Mem};
use std::cmp::max;
use win32_system::System;
use win32_winapi::calling_convention;

bitflags! {
    #[derive(Copy, Clone, Debug, Default, win32_derive::TryFromBitflags)]
    pub struct HeapAllocFlags: u32 {
        const HEAP_GENERATE_EXCEPTIONS = 0x4;
        const HEAP_NO_SERIALIZE = 0x1;
        const HEAP_ZERO_MEMORY = 0x8;
    }
}

#[win32_derive::dllexport]
pub fn HeapAlloc(
    sys: &dyn System,
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
    let memory = sys.memory();
    let heap = match memory.heaps.get(&hHeap) {
        None => {
            log::error!("HeapAlloc({hHeap:x}): no such heap");
            return 0;
        }
        Some(heap) => heap,
    };
    let addr = heap.alloc(memory.mem(), dwBytes);
    if addr == 0 {
        log::warn!("HeapAlloc({hHeap:x}) failed");
    }
    if flags.contains(HeapAllocFlags::HEAP_ZERO_MEMORY) {
        memory.mem().sub32_mut(addr, dwBytes).fill(0);
        flags.remove(HeapAllocFlags::HEAP_ZERO_MEMORY);
    }
    if !flags.is_empty() {
        log::error!("HeapAlloc: unhandled flags {flags:?}");
    }
    addr
}

#[win32_derive::dllexport]
pub fn HeapFree(sys: &dyn System, hHeap: u32, dwFlags: u32, lpMem: u32) -> bool {
    if dwFlags != 0 {
        log::warn!("HeapFree flags {dwFlags:x}");
    }
    let memory = sys.memory();
    memory.heaps.get(&hHeap).unwrap().free(memory.mem(), lpMem);
    true
}

#[win32_derive::dllexport]
pub fn HeapSize(sys: &dyn System, hHeap: u32, dwFlags: u32, lpMem: u32) -> u32 {
    if dwFlags != 0 {
        log::warn!("HeapSize flags {dwFlags:x}");
    }
    let memory = sys.memory();
    let heap = match memory.heaps.get(&hHeap) {
        None => {
            log::error!("HeapSize({hHeap:x}): no such heap");
            return 0;
        }
        Some(heap) => heap,
    };
    heap.size(memory.mem(), lpMem)
}

#[win32_derive::dllexport]
pub fn HeapSetInformation(
    sys: &dyn System,
    HeapHandle: u32,
    HeapInformationClass: u32,
    HeapInformation: u32,
    HeapInformationLength: u32,
) -> bool {
    true
}

#[win32_derive::dllexport]
pub fn HeapReAlloc(sys: &dyn System, hHeap: u32, dwFlags: u32, lpMem: u32, dwBytes: u32) -> u32 {
    if dwFlags != 0 {
        log::warn!("HeapReAlloc flags: {:x}", dwFlags);
    }
    let memory = sys.memory();
    let heap = match memory.heaps.get(&hHeap) {
        None => {
            log::error!("HeapSize({hHeap:x}): no such heap");
            return 0;
        }
        Some(heap) => heap,
    };
    let mem = memory.mem();
    let old_size = heap.size(mem, lpMem);
    let new_addr = heap.alloc(mem, dwBytes);
    assert!(dwBytes >= old_size);
    heap.free(mem, lpMem);
    sys.mem().copy(lpMem, new_addr, old_size);
    new_addr
}

bitflags! {
    #[derive(Debug, win32_derive::TryFromBitflags)]
    pub struct HeapCreateFlags: u32 {
        const HEAP_CREATE_ENABLE_EXECUTE = 0x00040000;
        const HEAP_GENERATE_EXCEPTIONS = 0x00000004;
        const HEAP_NO_SERIALIZE = 0x00000001;
    }
}

#[win32_derive::dllexport]
pub fn HeapCreate(
    sys: &mut dyn System,
    flOptions: Result<HeapCreateFlags, u32>,
    dwInitialSize: u32,
    dwMaximumSize: u32,
) -> u32 {
    flOptions.unwrap();
    // Currently none of the flags will affect behavior, but we might need to revisit this
    // with exceptions or threads support...
    let size = max(dwInitialSize as usize, 20 << 20);
    sys.memory_mut().new_heap(size, "HeapCreate".into()).addr
}

#[win32_derive::dllexport]
pub fn HeapDestroy(sys: &dyn System, hHeap: u32) -> u32 {
    log::warn!("HeapDestroy({hHeap:x})");
    1 // success
}

#[win32_derive::dllexport]
pub fn HeapValidate(sys: &dyn System, hHeap: u32, dwFlags: u32, lpMem: u32) -> bool {
    todo!();
}

#[win32_derive::dllexport]
pub fn HeapCompact(sys: &dyn System, hHeap: u32, dwFlags: u32 /* HEAP_FLAGS */) -> u32 {
    todo!()
}

pub type PROCESS_HEAP_ENTRY = u32; // TODO

#[win32_derive::dllexport]
pub fn HeapWalk(sys: &dyn System, hHeap: u32, lpEntry: Option<&mut PROCESS_HEAP_ENTRY>) -> bool {
    todo!()
}

bitflags! {
    #[derive(Debug, win32_derive::TryFromBitflags)]
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

bitflags! {
    #[derive(Debug, win32_derive::TryFromBitflags)]
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

#[win32_derive::dllexport]
pub fn VirtualAlloc(
    sys: &mut dyn System,
    lpAddress: u32,
    dwSize: u32,
    flAllocationType: Result<MEM, u32>,
    flProtec: Result<PAGE, u32>,
) -> u32 {
    let memory = sys.memory_mut();
    if lpAddress != 0 {
        // Changing flags on an existing address, hopefully.
        match memory
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

    let mapping = memory
        .mappings
        .alloc(memory.imp.mem(), dwSize, "VirtualAlloc".into());
    mapping.addr
}

#[repr(C)]
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
    sys: &dyn System,
    lpAddress: u32,
    lpBuffer: Option<&mut MEMORY_BASIC_INFORMATION>,
    dwLength: u32,
) -> u32 {
    0 // fail
}

#[win32_derive::dllexport]
pub fn VirtualFree(sys: &dyn System, lpAddress: u32, dwSize: u32, dwFreeType: u32) -> u32 {
    1 // success
}

#[win32_derive::dllexport]
pub fn IsBadReadPtr(sys: &dyn System, lp: u32, ucb: u32) -> bool {
    false // all pointers are valid
}

#[win32_derive::dllexport]
pub fn IsBadWritePtr(sys: &dyn System, lp: u32, ucb: u32) -> bool {
    false // all pointers are valid
}

#[win32_derive::dllexport]
pub fn IsBadCodePtr(sys: &dyn System, lpfn: u32) -> bool {
    false // all pointers are valid
}

bitflags! {
    #[derive(Debug)]
    pub struct GMEM: u32 {
        const MOVEABLE = 0x2;
        const ZEROINIT = 0x40;
        const MODIFY = 0x80;
    }
}
impl<'a> calling_convention::FromArg<'a> for GMEM {
    fn from_arg(_mem: Mem<'a>, arg: u32) -> Self {
        // GlobalAlloc accepted many flags, but most are obsolete, so ignore
        // anything other than the flags we have named.
        GMEM::from_bits_truncate(arg)
    }
}

fn alloc(sys: &mut dyn System, uFlags: GMEM, dwBytes: u32) -> u32 {
    // We ignore GMEM::MOVEABLE; see comments on HGLOBAL.
    let memory = sys.memory_mut();
    let addr = memory.process_heap.alloc(memory.imp.mem(), dwBytes);
    if uFlags.contains(GMEM::ZEROINIT) {
        memory.mem().sub32_mut(addr, dwBytes).fill(0);
    }
    addr
}

#[win32_derive::dllexport]
pub fn GlobalAlloc(sys: &mut dyn System, uFlags: GMEM, dwBytes: u32) -> u32 {
    alloc(sys, uFlags, dwBytes)
}

// In principle, callers can GlobalAlloc a handle that they then GlobalLock
// to get a pointer from, which allows things like reallocing that handle
// without changing its value.
// We don't support this, and instead just use addresses as the handle.
pub type HGLOBAL = u32;

#[win32_derive::dllexport]
pub fn GlobalHandle(sys: &dyn System, pMem: u32) -> HGLOBAL {
    // See comments on HGLOBAL.
    pMem
}

#[win32_derive::dllexport]
pub fn GlobalLock(sys: &dyn System, hMem: HGLOBAL) -> u32 {
    // See comments on HGLOBAL.
    hMem
}

#[win32_derive::dllexport]
pub fn GlobalReAlloc(sys: &mut dyn System, hMem: u32, dwBytes: u32, uFlags: GMEM) -> u32 {
    if uFlags.contains(GMEM::MODIFY) {
        todo!("GMEM_MODIFY");
    }

    // Otherwise, we are just reallocating the memory from scratch.
    // TODO: It's not clear what it means if you pass GMEM::MOVEABLE on a fixed block;
    // does that mean free the old block and allocate a new block with a moveable handle?
    GlobalFree(sys, hMem);
    return GlobalAlloc(sys, uFlags, dwBytes);

    // let heap = &sys.memory.process_heap;
    // let mem = sys.memory.mem();
    // let old_size = heap.size(mem, hMem);
    // if dwBytes <= old_size {
    //     return hMem;
    // }
    // let addr = heap.alloc(mem, dwBytes);
    // mem.copy(hMem, addr, old_size);
    // heap.free(mem, hMem);
    // if uFlags.contains(GMEM::ZEROINIT) {
    //     mem.sub32_mut(addr + old_size, dwBytes - old_size).fill(0);
    // }
    // addr
}

#[win32_derive::dllexport]
pub fn GlobalUnlock(sys: &dyn System, hMem: HGLOBAL) -> bool {
    // See comments on HGLOBAL.
    true // success
}

fn free(sys: &dyn System, hMem: u32) -> u32 {
    let memory = sys.memory();
    memory.process_heap.free(memory.mem(), hMem);
    return 0; // success
}

#[win32_derive::dllexport]
pub fn GlobalFree(sys: &dyn System, hMem: u32) -> u32 {
    free(sys, hMem)
}

#[win32_derive::dllexport]
pub fn GlobalFlags(sys: &dyn System, hMem: u32) -> u32 {
    0 // stub
}

#[win32_derive::dllexport]
pub fn LocalAlloc(sys: &mut dyn System, uFlags: GMEM, dwBytes: u32) -> u32 {
    // In theory this takes LMEM_* flags, but they are the same as GMEM_*.
    alloc(sys, uFlags, dwBytes)
}

#[win32_derive::dllexport]
pub fn LocalFree(sys: &dyn System, hMem: u32) -> u32 {
    free(sys, hMem)
}

#[win32_derive::dllexport]
pub fn VirtualProtect(
    sys: &dyn System,
    lpAddress: u32,
    dwSize: u32,
    flNewProtect: u32,
    lpflOldProtect: Option<&mut u32>,
) -> bool {
    true // success
}

#[win32_derive::dllexport]
pub fn GetProcessHeap(sys: &dyn System) -> u32 {
    sys.memory().process_heap.addr
}
