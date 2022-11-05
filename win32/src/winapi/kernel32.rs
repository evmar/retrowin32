#![allow(non_snake_case)]

use std::collections::HashMap;

use crate::{
    memory::{Memory, Pod, DWORD},
    pe::{self, ImageSectionFlags},
    x86::{self, X86},
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
    pub flags: pe::ImageSectionFlags,
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
    /// Memory for kernel32 data structures.
    heap: Heap,
    /// Address image was loaded at.
    pub image_base: u32,
    /// Address of TEB (FS register).
    pub teb: u32,
    pub mappings: Vec<Mapping>,
    /// Heaps created by HeapAlloc().
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
            flags: ImageSectionFlags::empty(),
        }];
        State {
            heap: Heap::new(0, 0),
            image_base: 0,
            teb: 0,
            mappings,
            heaps: HashMap::new(),
            cmdline: 0,
            env: 0,
        }
    }

    pub fn init(&mut self, mem: &mut Vec<u8>) {
        self.heap = self.new_private_heap(mem, 0x1000, "kernel32 data".into());
        // Fill region with garbage so it's clearer when we access something we don't intend to.
        mem[self.heap.addr as usize..(self.heap.addr + self.heap.size) as usize].fill(0xde);

        self.init_teb_peb(mem);

        let cmdline = "retrowin32.exe\0".as_bytes();
        let cmdline_addr = self.heap.alloc(mem, cmdline.len() as u32);
        mem[cmdline_addr as usize..cmdline_addr as usize + cmdline.len()].copy_from_slice(cmdline);
        self.cmdline = cmdline_addr;

        let env = "\0\0".as_bytes();
        let env_addr = self.heap.alloc(mem, env.len() as u32);
        mem[env_addr as usize..env_addr as usize + env.len()].copy_from_slice(env);
        self.env = env_addr;
    }

    /// Set up TEB, PEB, and other process info.
    /// The FS register points at the TEB (thread info), which points at the PEB (process info).
    fn init_teb_peb(&mut self, mem: &mut [u8]) {
        let params_addr = self.heap.alloc(mem, 0x100);
        let peb_addr = self.heap.alloc(mem, 0x100);
        let teb_addr = self.heap.alloc(mem, 0x100);

        // PEB
        mem.write_u32(peb_addr + 0x10, params_addr);

        // RTL_USER_PROCESS_PARAMETERS
        // x86.write_u32(params_addr + 0x10, console_handle);
        // x86.write_u32(params_addr + 0x14, console_flags);
        // x86.write_u32(params_addr + 0x18, stdin);
        mem.write_u32(params_addr + 0x1c, STDOUT_HFILE);

        // TEB
        mem.write_u32(teb_addr + 0x18, teb_addr); // Confusing: it points to itself.
        mem.write_u32(teb_addr + 0x30, peb_addr);

        self.teb = teb_addr;
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

    pub fn new_mapping(&mut self, size: u32, desc: String, mem: &mut Vec<u8>) -> &Mapping {
        if size > 1 << 20 {
            log::error!("new mapping {:?} {size:x} bytes", desc);
            assert!(size <= 1 << 20);
        }
        let mut prev_end = 0;
        let pos = self
            .mappings
            .iter()
            .position(|mapping| {
                let space = mapping.addr - prev_end;
                if space > size {
                    return true;
                }
                prev_end = mapping.addr + mapping.size + (0x1000 - 1) & !(0x1000 - 1);
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
                self.mappings.len()
            });

        self.mappings.insert(
            pos,
            Mapping {
                addr: prev_end,
                size,
                desc,
                flags: ImageSectionFlags::empty(),
            },
        );
        &self.mappings[pos]
    }

    pub fn new_private_heap(&mut self, mem: &mut Vec<u8>, size: usize, desc: String) -> Heap {
        let mapping = self.new_mapping(size as u32, desc, mem);
        Heap::new(mapping.addr, mapping.size)
    }

    pub fn new_heap(&mut self, mem: &mut Vec<u8>, size: usize, desc: String) -> u32 {
        let heap = self.new_private_heap(mem, size, desc);
        let addr = heap.addr;
        self.heaps.insert(addr, heap);
        addr
    }
}

pub fn GetLastError(_x86: &mut X86) -> u32 {
    0x1c // printer out of paper
}

/// A magic address that we jump to to end the process.
// TODO: this is pretty unsatisfying, we register this as a break point?
// Maybe better is to generate a hlt instruction somewhere and jump to it?
pub const MAGIC_EXIT_ADDRESS: u32 = 0xFFFF_FFFF;

pub fn ExitProcess(x86: &mut X86, uExitCode: u32) -> u32 {
    x86.host.exit(uExitCode);
    x86.regs.eip = MAGIC_EXIT_ADDRESS;
    0
}

pub fn GetACP(_x86: &mut X86) -> u32 {
    1252 // windows-1252
}

pub fn GetCommandLineA(x86: &mut X86) -> u32 {
    // TODO: possibly this should come from PEB->ProcessParameters->CommandLine.
    x86.state.kernel32.cmdline
}

pub fn GetCPInfo(_x86: &mut X86, _CodePage: u32, _lpCPInfo: u32) -> u32 {
    0 // fail
}

pub fn GetEnvironmentStrings(x86: &mut X86) -> u32 {
    x86.state.kernel32.env
}

pub fn FreeEnvironmentStringsA(_x86: &mut X86, _penv: u32) -> u32 {
    1 // success
}

pub fn GetEnvironmentStringsW(_x86: &mut X86) -> u32 {
    // CRT startup appears to fallback on non-W version of this if it returns null.
    0
}

pub fn GetEnvironmentVariableA(_x86: &mut X86, name: &str, buf: &mut [u8]) -> usize {
    println!("name {:?} buf {:?}", name, buf);
    0
}

pub fn GetFileType(_x86: &mut X86, hFile: u32) -> u32 {
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

pub fn GetModuleFileNameA(
    _x86: &mut X86,
    hModule: Option<HMODULE>,
    mut filename: &mut [u8],
) -> usize {
    assert!(hModule.is_none());
    match filename.write(b"TODO.exe\0") {
        Ok(n) => n,
        Err(err) => {
            log::warn!("GetModuleFileNameA(): {}", err);
            0
        }
    }
}

pub fn GetModuleHandleA(x86: &mut X86, lpModuleName: u32) -> u32 {
    if lpModuleName != 0 {
        log::error!("unimplemented: GetModuleHandle(non-null)")
    }
    // HMODULE is base address of current module.
    x86.state.kernel32.image_base
}

pub fn GetModuleHandleW(x86: &mut X86, lpModuleName: u32) -> u32 {
    if lpModuleName != 0 {
        log::error!("unimplemented: GetModuleHandleW(non-null)")
    }
    GetModuleHandleA(x86, 0)
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
unsafe impl Pod for STARTUPINFOA {}

pub fn GetStartupInfoA(x86: &mut X86, lpStartupInfo: u32) -> u32 {
    let ofs = lpStartupInfo as usize;
    let size = std::mem::size_of::<STARTUPINFOA>();
    x86.mem[ofs..ofs + size].fill(0);

    let info = x86.mem.view_mut::<STARTUPINFOA>(ofs as u32);
    info.cb.set(size as u32);
    0
}

pub fn IsProcessorFeaturePresent(_x86: &mut X86, _feature: u32) -> bool {
    false
}

pub fn IsDebuggerPresent(_x86: &mut X86) -> bool {
    true // Might cause a binary to log info via the debug API? Not sure.
}

pub fn GetCurrentThreadId(_x86: &mut X86) -> u32 {
    1
}

pub fn GetCurrentProcessId(_x86: &mut X86) -> u32 {
    1
}

pub fn GetStdHandle(_x86: &mut X86, nStdHandle: u32) -> u32 {
    match nStdHandle as i32 {
        -10 => STDIN_HFILE,
        -11 => STDOUT_HFILE,
        -12 => STDERR_HFILE,
        _ => (-1i32) as u32,
    }
}

pub fn GetTickCount(x86: &mut X86) -> u32 {
    x86.host.time()
}

pub fn QueryPerformanceCounter(_x86: &mut X86, _ptr: u32) -> bool {
    true // success
}

#[repr(C)]
pub struct FILETIME {
    dwLowDateTime: DWORD,
    dwHighDateTime: DWORD,
}
unsafe impl Pod for FILETIME {}
pub fn GetSystemTimeAsFileTime(_x86: &mut X86, _time: &mut FILETIME) -> u32 {
    0
}

pub fn GetVersion(_x86: &mut X86) -> u32 {
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

pub fn GetVersionExA(x86: &mut X86, lpVersionInformation: u32) -> u32 {
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

pub fn HeapAlloc(x86: &mut X86, hHeap: u32, dwFlags: u32, dwBytes: u32) -> u32 {
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

pub fn HeapFree(_x86: &mut X86, _hHeap: u32, dwFlags: u32, _lpMem: u32) -> u32 {
    if dwFlags != 0 {
        log::warn!("HeapFree flags {dwFlags:x}");
    }
    1 // success
}

pub fn HeapSize(x86: &mut X86, hHeap: u32, dwFlags: u32, lpMem: u32) -> u32 {
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

pub fn HeapReAlloc(x86: &mut X86, hHeap: u32, dwFlags: u32, lpMem: u32, dwBytes: u32) -> u32 {
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

pub fn HeapCreate(x86: &mut X86, flOptions: u32, dwInitialSize: u32, dwMaximumSize: u32) -> u32 {
    log::warn!("HeapCreate({flOptions:x}, {dwInitialSize:x}, {dwMaximumSize:x})");
    x86.state
        .kernel32
        .new_heap(&mut x86.mem, dwInitialSize as usize, "HeapCreate".into())
}

pub fn HeapDestroy(_x86: &mut X86, hHeap: u32) -> u32 {
    log::warn!("HeapDestroy({hHeap:x})");
    1 // success
}

pub fn LoadLibraryA(x86: &mut X86, lpLibFileName: u32) -> u32 {
    let filename = &x86.mem[lpLibFileName as usize..].read_strz();
    log::error!("LoadLibrary({filename:?})");
    0 // fail
}

pub fn LoadLibraryExW(_x86: &mut X86, lpLibFileName: u32, hFile: u32, dwFlags: u32) -> u32 {
    log::error!("LoadLibraryExW({lpLibFileName:x}, {hFile:x}, {dwFlags:x})");
    0 // fail
}

pub fn SetHandleCount(_x86: &mut X86, uNumber: u32) -> u32 {
    // "For Windows Win32 systems, this API has no effect."
    uNumber
}

pub fn WriteFile(
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

pub fn VirtualAlloc(
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
        .new_mapping(dwSize, "VirtualAlloc".into(), &mut x86.mem);
    mapping.addr
}

pub fn VirtualFree(_x86: &mut X86, lpAddress: u32, dwSize: u32, dwFreeType: u32) -> u32 {
    log::warn!("VirtualFree({lpAddress:x}, {dwSize:x}, {dwFreeType:x})");
    1 // success
}

#[repr(transparent)]
pub struct HMODULE(u32);
impl From<u32> for HMODULE {
    fn from(x: u32) -> Self {
        Self(x)
    }
}

pub fn OutputDebugStringA(_x86: &mut X86, msg: &str) -> u32 {
    log::warn!("OutputDebugStringA: {:?}", msg);
    0
}

pub fn InitializeCriticalSectionAndSpinCount(
    _x86: &mut X86,
    _lpCriticalSection: u32,
    _dwSpinCount: u32,
) -> bool {
    // On single-processor systems, the spin count is ignored and the critical section spin count is set to 0 (zero).
    false
}

pub fn EnterCriticalSection(_x86: &mut X86, _lpCriticalSection: u32) -> u32 {
    0
}

pub fn SetUnhandledExceptionFilter(_x86: &mut X86, _lpTopLevelExceptionFilter: u32) -> u32 {
    0 // No current handler.
}

pub fn UnhandledExceptionFilter(_x86: &mut X86, _exceptionInfo: u32) -> u32 {
    // "The process is being debugged, so the exception should be passed (as second chance) to the application's debugger."
    0 // EXCEPTION_CONTINUE_SEARCH
}
