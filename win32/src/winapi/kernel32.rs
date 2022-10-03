#![allow(non_snake_case)]

use std::collections::HashMap;

use super::{x86, x86::X86};
use crate::{reader::read_strz, winapi};
use tsify::Tsify;

// For now, a magic variable that makes it easier to spot.
pub const STDOUT_HFILE: u32 = 0xF11E_0100;
pub const STDERR_HFILE: u32 = 0xF11E_0101;

struct DWORD([u8; 4]);
impl DWORD {
    fn set(&mut self, val: u32) {
        self.0[0] = val as u8;
        self.0[1] = (val >> 8) as u8;
        self.0[2] = (val >> 16) as u8;
        self.0[3] = (val >> 24) as u8;
    }
}

#[derive(Debug, tsify::Tsify, serde::Serialize)]
pub struct Mapping {
    pub addr: u32,
    pub size: u32,
    pub desc: String,
}

struct Heap {
    addr: u32,
    size: u32,
    next: u32,
}
impl Heap {
    fn new(addr: u32, size: u32) -> Self {
        Heap {
            addr,
            size,
            next: 0,
        }
    }

    fn alloc(&mut self, size: u32) -> u32 {
        if self.next + size > self.size {
            log::error!("Heap::alloc cannot allocate {:x}", size);
            return 0;
        }
        let ptr = self.next;
        self.next += size;
        self.addr + ptr
    }
}

pub struct State {
    // Address image was loaded at.
    pub image_base: u32,
    // Address of TEB (FS register).
    pub teb: u32,
    pub mappings: Vec<Mapping>,
    heaps: HashMap<u32, Heap>,
}
impl State {
    pub fn new() -> Self {
        let mappings = vec![Mapping {
            addr: 0,
            size: x86::NULL_POINTER_REGION_SIZE,
            desc: "avoid null pointers".into(),
        }];
        State {
            image_base: 0,
            teb: 0,
            mappings,
            heaps: HashMap::new(),
        }
    }

    pub fn add_mapping(&mut self, mapping: Mapping) {
        let pos = self
            .mappings
            .iter()
            .position(|m| m.addr > mapping.addr)
            .unwrap_or(self.mappings.len());
        if pos > 0 {
            let prev = &self.mappings[pos - 1];
            assert!(prev.addr + prev.size <= mapping.addr);
        }
        if pos < self.mappings.len() {
            let next = &self.mappings[pos];
            assert!(mapping.addr + mapping.size <= next.addr);
        }
        self.mappings.insert(pos, mapping);
    }

    pub fn alloc(&mut self, size: u32, desc: String, mem: &mut Vec<u8>) -> &Mapping {
        let mut end = 0;
        let pos = self
            .mappings
            .iter()
            .position(|mapping| {
                let space = mapping.addr - end;
                if space > size {
                    return true;
                }
                end = mapping.addr + mapping.size + (0x1000 - 1) & !(0x1000 - 1);
                false
            })
            .unwrap_or_else(|| {
                let space = mem.len() as u32 - end;
                if space < size {
                    mem.resize((end + size) as usize, 0);
                }
                self.mappings.len()
            });

        self.mappings.insert(
            pos,
            Mapping {
                addr: end,
                size,
                desc,
            },
        );
        return &self.mappings[pos];
    }
}

fn ExitProcess(x86: &mut X86, uExitCode: u32) -> u32 {
    x86.host.exit(uExitCode);
    0
}

fn GetEnvironmentVariableA(_x86: &mut X86, _lpName: u32, _lpBuffer: u32, _nSize: u32) -> u32 {
    // Fail for now.
    0
}

fn GetModuleFileNameA(_x86: &mut X86, hModule: u32, lpFilename: u32, nSize: u32) -> u32 {
    log::warn!("GetModuleFileNameA({hModule:x}, {lpFilename:x}, {nSize:x})");
    0 // fail
}

fn GetModuleHandleA(x86: &mut X86, lpModuleName: u32) -> u32 {
    if lpModuleName != 0 {
        log::error!("unimplemented: GetModuleHandle(non-null)")
    }
    // HMODULE is base address of current module.
    x86.state.kernel32.image_base
}

#[repr(C)]
struct STARTUPINFOA {
    cb: DWORD,
    lpReserved: DWORD,
    lpDesktop: DWORD,
    lpTitle: DWORD,
    dwX: DWORD,
    dwY: DWORD,
    dwXSize: DWORD,
    dwYSize: DWORD,
    dwXCountChars: DWORD,
    dwYCountChars: DWORD,
    dwFillAttribute: DWORD,
    dwFlags: DWORD,
    wShowWindow: u16,
    cbReserved2: u16,
    lpReserved2: DWORD,
    hStdInput: DWORD,
    hStdOutput: DWORD,
    hStdError: DWORD,
}

fn GetStartupInfoA(x86: &mut X86, lpStartupInfo: u32) -> u32 {
    let ofs = lpStartupInfo as usize;
    let size = std::mem::size_of::<STARTUPINFOA>();
    x86.mem[ofs..ofs + size].fill(0);

    let buf = &mut x86.mem[ofs..ofs + std::mem::size_of::<STARTUPINFOA>()];
    let info: &mut STARTUPINFOA =
        unsafe { (buf.as_mut_ptr() as *mut STARTUPINFOA).as_mut().unwrap() };
    info.cb.set(size as u32);
    0
}

fn GetStdHandle(_x86: &mut X86, nStdHandle: u32) -> u32 {
    match nStdHandle as i32 {
        -10 => unimplemented!("GetStdHandle(stdin)"),
        -11 => STDOUT_HFILE,
        -12 => STDERR_HFILE,
        _ => (-1i32) as u32,
    }
}

fn GetVersion(_x86: &mut X86) -> u32 {
    // Win95, version 4.0.
    (1 << 31) | 0x4
}

#[repr(C)]
struct OSVERSIONINFO {
    dwOSVersionInfoSize: DWORD,
    dwMajorVersion: DWORD,
    dwMinorVersion: DWORD,
    dwBuildNumber: DWORD,
    dwPlatformId: DWORD,
    //szCSDVersion: [u8; 128],
}

fn GetVersionExA(x86: &mut X86, lpVersionInformation: u32) -> u32 {
    let ofs = lpVersionInformation as usize;
    let size = x86.read_u32(lpVersionInformation) as usize;
    if size < std::mem::size_of::<OSVERSIONINFO>() {
        log::error!("GetVersionExA undersized buffer");
        return 0;
    }
    x86.mem[ofs..ofs + size].fill(0);

    let buf = &mut x86.mem[ofs..ofs + std::mem::size_of::<OSVERSIONINFO>()];
    let info: &mut OSVERSIONINFO =
        unsafe { (buf.as_mut_ptr() as *mut OSVERSIONINFO).as_mut().unwrap() };

    info.dwOSVersionInfoSize.set(size as u32);
    info.dwMajorVersion.set(6); // ? pulled from debugger
    info.dwPlatformId.set(2 /* VER_PLATFORM_WIN32_NT */);

    1
}

fn HeapAlloc(x86: &mut X86, hHeap: u32, dwFlags: u32, dwBytes: u32) -> u32 {
    if dwFlags != 0 {
        log::warn!("HeapAlloc flags {dwFlags:x}");
    }
    let heap = match x86.state.kernel32.heaps.get_mut(&hHeap) {
        None => {
            log::error!("HeapAlloc({hHeap:x}): no such heap");
            return 0;
        }
        Some(heap) => heap,
    };
    heap.alloc(dwBytes)
}

fn HeapCreate(x86: &mut X86, flOptions: u32, dwInitialSize: u32, dwMaximumSize: u32) -> u32 {
    log::warn!("HeapCreate({flOptions:x}, {dwInitialSize:x}, {dwMaximumSize:x})");
    let mapping = x86
        .state
        .kernel32
        .alloc(dwInitialSize, "HeapCreate".into(), &mut x86.mem);
    let addr = mapping.addr;
    let size = mapping.size;
    x86.state.kernel32.heaps.insert(addr, Heap::new(addr, size));
    addr
}

fn HeapDestroy(_x86: &mut X86, hHeap: u32) -> u32 {
    log::warn!("HeapDestroy({hHeap:x})");
    1 // success
}

fn LoadLibraryA(x86: &mut X86, lpLibFileName: u32) -> u32 {
    let filename = read_strz(&x86.mem[lpLibFileName as usize..]);
    log::error!("LoadLibrary({filename:?})");
    0 // fail
}

fn WriteFile(
    x86: &mut X86,
    hFile: u32,
    lpBuffer: u32,
    nNumberOfBytesToWrite: u32,
    lpNumberOfBytesWritten: u32,
    lpOverlapped: u32,
) -> u32 {
    assert!(hFile == STDOUT_HFILE || hFile == STDERR_HFILE);
    assert!(lpOverlapped == 0);
    let buf = &x86.mem[lpBuffer as usize..(lpBuffer + nNumberOfBytesToWrite) as usize];

    let n = x86.host.write(buf);

    x86.write_u32(lpNumberOfBytesWritten, n as u32);
    1
}

fn VirtualAlloc(
    x86: &mut X86,
    lpAddress: u32,
    dwSize: u32,
    _flAllocationType: u32,
    _flProtec: u32,
) -> u32 {
    if lpAddress != 0 {
        // Changing flags on an existing address, hopefully.
        match x86
            .state
            .kernel32
            .mappings
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

    let mapping = x86
        .state
        .kernel32
        .alloc(dwSize, "VirtualAlloc".into(), &mut x86.mem);
    mapping.addr
}

fn VirtualFree(_x86: &mut X86, lpAddress: u32, dwSize: u32, dwFreeType: u32) -> u32 {
    log::warn!("VirtualFree({lpAddress:x}, {dwSize:x}, {dwFreeType:x})");
    1 // success
}

winapi!(
    fn ExitProcess(uExitCode: u32);
    fn GetEnvironmentVariableA(lpName: u32, lpBuffer: u32, nSize: u32);
    fn GetModuleFileNameA(hModule: u32, lpFilename: u32, nSize: u32);
    fn GetModuleHandleA(lpModuleName: u32);
    fn GetStartupInfoA(lpStartupInfo: u32);
    fn GetStdHandle(nStdHandle: u32);
    fn GetVersion();
    fn GetVersionExA(lpVersionInformation: u32);
    fn HeapAlloc(hHeap: u32, dwFlags: u32, dwBytes: u32);
    fn HeapCreate(flOptions: u32, dwInitialSize: u32, dwMaximumSize: u32);
    fn HeapDestroy(hHeap: u32);
    fn LoadLibraryA(lpLibFileName: u32);
    fn WriteFile(
        hFile: u32,
        lpBuffer: u32,
        nNumberOfBytesToWrite: u32,
        lpNumberOfBytesWritten: u32,
        lpOverlapped: u32,
    );
    fn VirtualAlloc(lpAddress: u32, dwSize: u32, _flAllocationType: u32, _flProtec: u32);
    fn VirtualFree(lpAddress: u32, dwSize: u32, dwFreeType: u32);
);
