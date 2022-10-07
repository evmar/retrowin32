#![allow(non_snake_case)]

use std::collections::HashMap;

use super::{x86, x86::X86};
use crate::{
    memory::{Memory, DWORD},
    winapi,
};
use std::io::Write;
use tsify::Tsify;

// For now, a magic variable that makes it easier to spot.
pub const STDIN_HFILE: u32 = 0xF11E_0100;
pub const STDOUT_HFILE: u32 = 0xF11E_0101;
pub const STDERR_HFILE: u32 = 0xF11E_0102;

#[derive(Debug, tsify::Tsify, serde::Serialize)]
pub struct Mapping {
    pub addr: u32,
    pub size: u32,
    pub desc: String,
}

pub struct Heap {
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

    pub fn alloc(&mut self, mem: &mut [u8], size: u32) -> u32 {
        let alloc_size = size + 4; // TODO: align?
        if self.next + alloc_size > self.size {
            log::error!("Heap::alloc cannot allocate {:x}", size);
            return 0;
        }
        let addr = self.addr + self.next;
        mem.write_u32(addr, size);
        self.next += alloc_size;
        addr + 4
    }

    pub fn size(&self, mem: &[u8], addr: u32) -> u32 {
        assert!(addr >= self.addr + 4 && addr < self.addr + self.size);
        mem.read_u32(addr - 4)
    }
}

pub struct State {
    // Address image was loaded at.
    pub image_base: u32,
    // Address of TEB (FS register).
    pub teb: u32,
    pub mappings: Vec<Mapping>,
    pub heaps: HashMap<u32, Heap>,
    cmdline: u32,
    env: u32,
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
            cmdline: 0,
            env: 0,
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

    pub fn new_heap(&mut self, mem: &mut Vec<u8>, size: usize, desc: String) -> u32 {
        let mapping = self.alloc(size as u32, desc, mem);
        let addr = mapping.addr;
        let size = mapping.size;
        self.heaps.insert(addr, Heap::new(addr, size));
        addr
    }
}

/// Set up TEB, PEB, and other process info.
/// The FS register points at the TEB (thread info), which points at the PEB (process info).
pub fn init_teb_peb(x86: &mut X86) {
    let mapping = x86
        .state
        .kernel32
        .alloc(0x1000, "PEB/TEB".into(), &mut x86.mem);
    // Fill region with garbage so it's clearer when we access something we don't intend to.
    x86.mem[mapping.addr as usize..(mapping.addr + mapping.size) as usize].fill(0xde);

    let peb_addr = mapping.addr;
    let params_addr = mapping.addr + 0x100;
    let teb_addr = params_addr + 0x100;

    // XXX need a better place for this.
    let cmdline_addr = teb_addr + 0x100;
    let cmdline = "retrowin32.exe\0".as_bytes();
    x86.mem[cmdline_addr as usize..cmdline_addr as usize + cmdline.len()].copy_from_slice(cmdline);
    x86.state.kernel32.cmdline = cmdline_addr;

    let env_addr = cmdline_addr + 0x20;
    let env = "\0\0".as_bytes();
    x86.mem[env_addr as usize..env_addr as usize + env.len()].copy_from_slice(env);
    x86.state.kernel32.env = env_addr;

    // PEB
    x86.write_u32(peb_addr + 0x10, params_addr);

    // RTL_USER_PROCESS_PARAMETERS
    // x86.write_u32(params_addr + 0x10, console_handle);
    // x86.write_u32(params_addr + 0x14, console_flags);
    // x86.write_u32(params_addr + 0x18, stdin);
    x86.write_u32(params_addr + 0x1c, STDOUT_HFILE);

    // TEB
    x86.write_u32(teb_addr + 0x18, teb_addr); // Confusing: it points to itself.
    x86.write_u32(teb_addr + 0x30, peb_addr);

    x86.state.kernel32.teb = teb_addr;
}

fn ExitProcess(x86: &mut X86, uExitCode: u32) -> u32 {
    x86.host.exit(uExitCode);
    0
}

fn GetACP(_x86: &mut X86) -> u32 {
    1252 // windows-1252
}

fn GetCommandLineA(x86: &mut X86) -> u32 {
    // TODO: possibly this should come from PEB->ProcessParameters->CommandLine.
    x86.state.kernel32.cmdline
}

fn GetCPInfo(_x86: &mut X86, _CodePage: u32, _lpCPInfo: u32) -> u32 {
    0 // fail
}

fn GetEnvironmentStrings(x86: &mut X86) -> u32 {
    x86.state.kernel32.env
}

fn FreeEnvironmentStringsA(_x86: &mut X86, _penv: u32) -> u32 {
    1 // success
}

fn GetEnvironmentStringsW(_x86: &mut X86) -> u32 {
    // CRT startup appears to fallback on non-W version of this if it returns null.
    0
}

fn GetEnvironmentVariableA(_x86: &mut X86, _lpName: u32, _lpBuffer: u32, _nSize: u32) -> u32 {
    // Fail for now.
    0
}

fn GetFileType(_x86: &mut X86, hFile: u32) -> u32 {
    let FILE_TYPE_CHAR = 0x2;
    let FILE_TYPE_UNKNOWN = 0x8;
    match hFile {
        STDIN_HFILE | STDOUT_HFILE | STDERR_HFILE => FILE_TYPE_CHAR,
        _ => {
            log::error!("GetFileType({hFile:x}) unknown handle");
            FILE_TYPE_UNKNOWN
        }
    }
}

fn GetModuleFileNameA(x86: &mut X86, hModule: u32, lpFilename: u32, nSize: u32) -> u32 {
    if hModule != 0 {
        log::warn!("GetModuleFileNameA({hModule:x})");
        return 0;
    }
    match (x86.mem[lpFilename as usize..(lpFilename + nSize) as usize].as_mut())
        .write(b"TODO.exe\0")
    {
        Ok(n) => n as u32,
        Err(err) => {
            log::warn!("GetModuleFileNameA(): {}", err);
            0
        }
    }
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
        -10 => STDIN_HFILE,
        -11 => STDOUT_HFILE,
        -12 => STDERR_HFILE,
        _ => (-1i32) as u32,
    }
}

fn GetTickCount(x86: &mut X86) -> u32 {
    x86.host.time()
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
    heap.alloc(&mut x86.mem, dwBytes)
}

fn HeapFree(_x86: &mut X86, _hHeap: u32, dwFlags: u32, _lpMem: u32) -> u32 {
    if dwFlags != 0 {
        log::warn!("HeapFree flags {dwFlags:x}");
    }
    1 // success
}

fn HeapSize(x86: &mut X86, hHeap: u32, dwFlags: u32, lpMem: u32) -> u32 {
    if dwFlags != 0 {
        log::warn!("HeapSize flags {dwFlags:x}");
    }
    let heap = match x86.state.kernel32.heaps.get(&hHeap) {
        None => {
            log::error!("HeapSize({hHeap:x}): no such heap");
            return 0;
        }
        Some(heap) => heap,
    };
    heap.size(&x86.mem, lpMem)
}

fn HeapReAlloc(x86: &mut X86, hHeap: u32, dwFlags: u32, lpMem: u32, dwBytes: u32) -> u32 {
    if dwFlags != 0 {
        log::warn!("HeapReAlloc flags: {:x}", dwFlags);
    }
    let heap = match x86.state.kernel32.heaps.get_mut(&hHeap) {
        None => {
            log::error!("HeapSize({hHeap:x}): no such heap");
            return 0;
        }
        Some(heap) => heap,
    };
    let old_size = heap.size(&x86.mem, lpMem);
    let new_addr = heap.alloc(&mut x86.mem, dwBytes);
    log::info!("realloc {lpMem:x}/{old_size:x} => {new_addr:x}/{dwBytes:x}");
    x86.mem.copy_within(
        lpMem as usize..(lpMem + old_size) as usize,
        new_addr as usize,
    );
    new_addr
}

fn HeapCreate(x86: &mut X86, flOptions: u32, dwInitialSize: u32, dwMaximumSize: u32) -> u32 {
    log::warn!("HeapCreate({flOptions:x}, {dwInitialSize:x}, {dwMaximumSize:x})");
    x86.state
        .kernel32
        .new_heap(&mut x86.mem, dwInitialSize as usize, "HeapCreate".into())
}

fn HeapDestroy(_x86: &mut X86, hHeap: u32) -> u32 {
    log::warn!("HeapDestroy({hHeap:x})");
    1 // success
}

fn LoadLibraryA(x86: &mut X86, lpLibFileName: u32) -> u32 {
    let filename = &x86.mem[lpLibFileName as usize..].read_strz();
    log::error!("LoadLibrary({filename:?})");
    0 // fail
}

fn SetHandleCount(_x86: &mut X86, uNumber: u32) -> u32 {
    // "For Windows Win32 systems, this API has no effect."
    uNumber
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
    fn GetACP();
    fn GetCommandLineA();
    fn GetCPInfo(CodePage: u32, lpCPInfo: u32);
    fn GetEnvironmentVariableA(lpName: u32, lpBuffer: u32, nSize: u32);
    fn GetEnvironmentStrings();
    fn FreeEnvironmentStringsA(penv: u32);
    fn GetEnvironmentStringsW();
    fn GetFileType(hFile: u32);
    fn GetModuleFileNameA(hModule: u32, lpFilename: u32, nSize: u32);
    fn GetModuleHandleA(lpModuleName: u32);
    fn GetStartupInfoA(lpStartupInfo: u32);
    fn GetStdHandle(nStdHandle: u32);
    fn GetTickCount();
    fn GetVersion();
    fn GetVersionExA(lpVersionInformation: u32);
    fn HeapAlloc(hHeap: u32, dwFlags: u32, dwBytes: u32);
    fn HeapCreate(flOptions: u32, dwInitialSize: u32, dwMaximumSize: u32);
    fn HeapDestroy(hHeap: u32);
    fn HeapFree(hHeap: u32, dwFlags: u32, lpMem: u32);
    fn HeapSize(hHeap: u32, dwFlags: u32, lpMem: u32);
    fn HeapReAlloc(hHeap: u32, dwFlags: u32, lpMem: u32, dwBytes: u32);

    fn LoadLibraryA(lpLibFileName: u32);
    fn WriteFile(
        hFile: u32,
        lpBuffer: u32,
        nNumberOfBytesToWrite: u32,
        lpNumberOfBytesWritten: u32,
        lpOverlapped: u32,
    );
    fn SetHandleCount(uNumber: u32);
    fn VirtualAlloc(lpAddress: u32, dwSize: u32, _flAllocationType: u32, _flProtec: u32);
    fn VirtualFree(lpAddress: u32, dwSize: u32, dwFreeType: u32);
);
