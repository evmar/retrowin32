#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use crate::{
    machine::Machine,
    pe::ImageSectionFlags,
    winapi::{self, shims::ToX86},
};
use bitflags::bitflags;
use num_traits::FromPrimitive;
use std::collections::HashMap;
use std::io::Write;
use tsify::Tsify;
use x86::Memory;

use super::{
    alloc::Alloc,
    alloc::ArenaInfo,
    alloc::{Heap, HeapInfo},
    types::{Str16, String16, DWORD, HFILE, HMODULE, WORD},
};

// For now, a magic variable that makes it easier to spot.
pub const STDIN_HFILE: HFILE = HFILE::from_raw(0xF11E_0100);
pub const STDOUT_HFILE: HFILE = HFILE::from_raw(0xF11E_0101);
pub const STDERR_HFILE: HFILE = HFILE::from_raw(0xF11E_0102);

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
    fn new() -> Self {
        Mappings(vec![Mapping {
            addr: 0,
            size: x86::NULL_POINTER_REGION_SIZE,
            desc: "avoid null pointers".into(),
            flags: ImageSectionFlags::empty(),
        }])
    }

    pub fn add(&mut self, mut mapping: Mapping) {
        mapping.size = round_up_to_page_granularity(mapping.size);
        let pos = self
            .0
            .iter()
            .position(|m| m.addr > mapping.addr)
            .unwrap_or(self.0.len());
        if pos > 0 {
            let prev = &self.0[pos - 1];
            assert!(prev.addr + prev.size <= mapping.addr);
        }
        if pos < self.0.len() {
            let next = &self.0[pos];
            assert!(mapping.addr + mapping.size <= next.addr);
        }
        self.0.insert(pos, mapping);
    }

    pub fn alloc(&mut self, size: u32, desc: String, mem: &mut Vec<u8>) -> &Mapping {
        let size = round_up_to_page_granularity(size);
        if size > 1 << 20 {
            log::error!("new mapping {:?} {size:x} bytes", desc);
            assert!(size <= 1 << 20);
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

    pub fn grow(&mut self, addr: u32) -> u32 {
        let pos = self.0.iter().position(|m| m.addr == addr).unwrap();
        let mapping = &self.0[pos];
        let new_size = mapping.size * 2;
        if pos + 1 < self.0.len() {
            let next = &self.0[pos + 1];
            if mapping.addr + new_size > next.addr {
                panic!("cannot grow {:?}", mapping);
            }
        }

        let mapping = &mut self.0[pos];
        let growth = new_size - mapping.size;
        mapping.size = new_size;
        log::info!("grew mapping {:?} by {:#x}", mapping.desc, growth);
        growth
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct State {
    /// Memory for kernel32 data structures.
    arena: ArenaInfo,
    /// Address image was loaded at.
    pub image_base: u32,
    /// Address of TEB (what FS register-relative addresses refer to).
    pub teb: u32,
    pub mappings: Mappings,
    /// Heaps created by HeapAlloc().
    heaps: HashMap<u32, HeapInfo>,

    env: u32,

    /// Command line, ASCII.
    cmdline: u32,
    /// Command line, UTF16.
    cmdline16: u32,
}
impl State {
    pub fn new() -> Self {
        State {
            arena: ArenaInfo::new(0, 0),
            image_base: 0,
            teb: 0,
            mappings: Mappings::new(),
            heaps: HashMap::new(),
            env: 0,
            cmdline: 0,
            cmdline16: 0,
        }
    }

    pub fn init(&mut self, mem: &mut Vec<u8>, cmdline: String) {
        let mapping = self.mappings.alloc(0x1000, "kernel32 data".into(), mem);
        self.arena = ArenaInfo::new(mapping.addr, mapping.size);

        let cmdline_len = cmdline.len();
        self.init_cmdline(mem, cmdline);
        self.init_teb_peb(mem, cmdline_len);

        let env = "\0\0".as_bytes();
        let env_addr = self.arena.get(mem).alloc(env.len() as u32);
        mem[env_addr as usize..env_addr as usize + env.len()].copy_from_slice(env);
        self.env = env_addr;
    }

    fn init_cmdline(&mut self, mem: &mut [u8], mut cmdline: String) {
        // Gross: GetCommandLineA() needs to return a pointer that's never freed,
        // so we need to hang on to both versions of the command line.

        cmdline.push(0 as char); // nul terminator

        self.cmdline = self.arena.get(mem).alloc(cmdline.len() as u32);
        mem[self.cmdline as usize..self.cmdline as usize + cmdline.len()]
            .copy_from_slice(cmdline.as_bytes());

        let cmdline16 = String16::from(&cmdline);
        self.cmdline16 = self.arena.get(mem).alloc(cmdline16.byte_size() as u32);
        let mem16: &mut [u16] = unsafe {
            std::mem::transmute(
                &mut mem[self.cmdline16 as usize..self.cmdline16 as usize + cmdline16.0.len()],
            )
        };
        mem16.copy_from_slice(&cmdline16.0);
    }

    /// Set up TEB, PEB, and other process info.
    /// The FS register points at the TEB (thread info), which points at the PEB (process info).
    fn init_teb_peb(&mut self, mem: &mut [u8], cmdline_len: usize) {
        // RTL_USER_PROCESS_PARAMETERS
        let params_addr = self.arena.get(mem).alloc(std::cmp::max(
            std::mem::size_of::<RTL_USER_PROCESS_PARAMETERS>() as u32,
            0x100,
        ));
        let params = mem.view_mut::<RTL_USER_PROCESS_PARAMETERS>(params_addr);
        // x86.write_u32(params_addr + 0x10, console_handle);
        // x86.write_u32(params_addr + 0x14, console_flags);
        // x86.write_u32(params_addr + 0x18, stdin);
        params.hStdOutput = STDOUT_HFILE;
        params.hStdError = STDERR_HFILE;
        params.ImagePathName.clear();
        params.CommandLine = UNICODE_STRING {
            Length: cmdline_len as u16,
            MaximumLength: cmdline_len as u16,
            Buffer: self.cmdline16,
        };

        // PEB
        let peb_addr = self
            .arena
            .get(mem)
            .alloc(std::cmp::max(std::mem::size_of::<PEB>() as u32, 0x100));
        let peb = mem.view_mut::<PEB>(peb_addr);
        peb.ProcessParameters = params_addr;
        peb.ProcessHeap = 0; // Initialized lazily.
        peb.TlsCount = 0;

        // SEH chain
        let seh_addr = self
            .arena
            .get(mem)
            .alloc(std::mem::size_of::<_EXCEPTION_REGISTRATION_RECORD>() as u32);
        let seh = mem.view_mut::<_EXCEPTION_REGISTRATION_RECORD>(seh_addr);
        seh.Prev = 0xFFFF_FFFF;
        seh.Handler = 0xFF5E_5EFF; // Hopefully easier to spot.

        // TEB
        let teb_addr = self
            .arena
            .get(mem)
            .alloc(std::cmp::max(std::mem::size_of::<TEB>() as u32, 0x100));
        let teb = mem.view_mut::<TEB>(teb_addr);
        teb.Tib.ExceptionList = seh_addr;
        teb.Tib._Self = teb_addr; // Confusing: it points to itself.
        teb.Peb = peb_addr;

        self.teb = teb_addr;
        // log::info!("params {params_addr:x} peb {peb_addr:x} teb {teb_addr:x}");
    }

    pub fn new_private_heap(&mut self, mem: &mut Vec<u8>, size: usize, desc: String) -> HeapInfo {
        let mapping = self.mappings.alloc(size as u32, desc, mem);
        HeapInfo::new(mem, mapping.addr, mapping.size)
    }

    pub fn new_heap(&mut self, mem: &mut Vec<u8>, size: usize, desc: String) -> u32 {
        let heap = self.new_private_heap(mem, size, desc);
        let addr = heap.addr;
        self.heaps.insert(addr, heap);
        addr
    }

    pub fn get_heap<'a>(&'a mut self, mem: &'a mut [u8], addr: u32) -> Option<Heap<'a>> {
        self.heaps
            .get_mut(&addr)
            .map(|h| h.get_heap(mem, &mut self.mappings))
    }
}

fn teb(machine: &Machine) -> &TEB {
    machine.x86.mem.view::<TEB>(machine.state.kernel32.teb)
}
fn teb_mut(machine: &mut Machine) -> &mut TEB {
    machine.x86.mem.view_mut::<TEB>(machine.state.kernel32.teb)
}
fn peb_mut(machine: &mut Machine) -> &mut PEB {
    let peb_addr = teb(machine).Peb;
    machine.x86.mem.view_mut::<PEB>(peb_addr)
}

#[repr(C)]
struct PEB {
    InheritedAddressSpace: u8,
    ReadImageFileExecOptions: u8,
    BeingDebugged: u8,
    SpareBool: u8,
    Mutant: DWORD,
    ImageBaseAddress: DWORD,
    LdrData: DWORD,
    /* 0x10 */
    ProcessParameters: DWORD,
    SubSystemData: DWORD,
    ProcessHeap: DWORD,
    // TODO: more fields

    // This is at the wrong offset, but it shouldn't matter.
    // TODO: this should be TlsBitmap.
    TlsCount: DWORD,
}
unsafe impl x86::Pod for PEB {}

#[repr(C)]
struct NT_TIB {
    ExceptionList: DWORD,
    StackBase: DWORD,
    StackLimit: DWORD,
    SubSystemTib: DWORD,
    FiberData: DWORD,
    ArbitraryUserPointer: DWORD,
    _Self: DWORD,
}
unsafe impl x86::Pod for NT_TIB {}

#[repr(C)]
struct TEB {
    Tib: NT_TIB,
    EnvironmentPointer: DWORD,
    ClientId_UniqueProcess: DWORD,
    ClientId_UniqueThread: DWORD,
    ActiveRpcHandle: DWORD,
    ThreadLocalStoragePointer: DWORD,
    Peb: DWORD,
    LastErrorValue: DWORD,
    CountOfOwnedCriticalSections: DWORD,
    CsrClientThread: DWORD,
    Win32ThreadInfo: DWORD,
    User32Reserved: [DWORD; 26],
    UserReserved: [DWORD; 5],
    WOW32Reserved: DWORD,
    CurrentLocale: DWORD,
    // TODO: ... there are many more fields here

    // This is at the wrong offset, but it shouldn't matter.
    TlsSlots: [DWORD; 64],
}
unsafe impl x86::Pod for TEB {}

#[repr(C)]
struct UNICODE_STRING {
    Length: WORD,
    MaximumLength: WORD,
    Buffer: DWORD,
}

impl UNICODE_STRING {
    fn clear(&mut self) {
        self.Length = 0;
        self.MaximumLength = 0;
        self.Buffer = 0;
    }
}

#[repr(C)]
struct CURDIR {
    DosPath: UNICODE_STRING,
    Handle: DWORD,
}

#[repr(C)]
struct RTL_USER_PROCESS_PARAMETERS {
    AllocationSize: DWORD,
    Size: DWORD,
    Flags: DWORD,
    DebugFlags: DWORD,
    ConsoleHandle: DWORD,
    ConsoleFlags: DWORD,
    hStdInput: HFILE,
    hStdOutput: HFILE,
    hStdError: HFILE,
    CurrentDirectory: CURDIR,
    DllPath: UNICODE_STRING,
    ImagePathName: UNICODE_STRING,
    CommandLine: UNICODE_STRING,
}
unsafe impl x86::Pod for RTL_USER_PROCESS_PARAMETERS {}

#[repr(C)]
struct _EXCEPTION_REGISTRATION_RECORD {
    Prev: DWORD,
    Handler: DWORD,
}
unsafe impl x86::Pod for _EXCEPTION_REGISTRATION_RECORD {}

#[win32_derive::dllexport]
pub fn SetLastError(machine: &mut Machine, dwErrCode: u32) -> u32 {
    teb_mut(machine).LastErrorValue = dwErrCode;
    0 // unused
}

#[win32_derive::dllexport]
pub fn GetLastError(_machine: &mut Machine) -> u32 {
    // TODO: should we start calling SetLastError when appropriate?
    0x1c // printer out of paper
}

#[win32_derive::dllexport]
pub fn ExitProcess(machine: &mut Machine, uExitCode: u32) -> u32 {
    machine.host.exit(uExitCode);
    // TODO: this is unsatisfying.
    // Maybe better is to generate a hlt instruction somewhere and jump to it?
    machine.x86.stop();
    0
}

#[win32_derive::dllexport]
pub fn GetACP(_machine: &mut Machine) -> u32 {
    1252 // windows-1252
}

#[win32_derive::dllexport]
pub fn IsValidCodePage(_machine: &mut Machine, CodePage: u32) -> bool {
    CodePage == 1252
}

#[win32_derive::dllexport]
pub fn GetCPInfo(_machine: &mut Machine, _CodePage: u32, _lpCPInfo: u32) -> u32 {
    0 // fail
}

#[win32_derive::dllexport]
pub fn GetCommandLineA(machine: &mut Machine) -> u32 {
    machine.state.kernel32.cmdline
}

#[win32_derive::dllexport]
pub fn GetCommandLineW(machine: &mut Machine) -> u32 {
    machine.state.kernel32.cmdline16
}

#[win32_derive::dllexport]
pub fn GetEnvironmentStrings(machine: &mut Machine) -> u32 {
    machine.state.kernel32.env
}

#[win32_derive::dllexport]
pub fn FreeEnvironmentStringsA(_machine: &mut Machine, _penv: u32) -> u32 {
    1 // success
}

#[win32_derive::dllexport]
pub fn GetEnvironmentStringsW(_machine: &mut Machine) -> u32 {
    // CRT startup appears to fallback on non-W version of this if it returns null.
    0
}

#[win32_derive::dllexport]
pub fn GetEnvironmentVariableA(
    _machine: &mut Machine,
    name: Option<&str>,
    buf: Option<&mut [u8]>,
) -> usize {
    println!("name {:?} buf {:?}", name, buf);
    0
}

#[win32_derive::dllexport]
pub fn GetFileType(_machine: &mut Machine, hFile: HFILE) -> u32 {
    let FILE_TYPE_CHAR = 0x2;
    let FILE_TYPE_UNKNOWN = 0x8;
    match hFile {
        STDIN_HFILE | STDOUT_HFILE | STDERR_HFILE => FILE_TYPE_CHAR,
        _ => {
            log::error!("GetFileType({hFile:?}) unknown handle");
            FILE_TYPE_UNKNOWN
        }
    }
}

#[win32_derive::dllexport]
pub fn GetModuleFileNameA(
    _machine: &mut Machine,
    hModule: HMODULE,
    filename: Option<&mut [u8]>,
) -> usize {
    assert!(hModule.is_null());
    match filename.unwrap().write(b"TODO.exe\0") {
        Ok(n) => n,
        Err(err) => {
            log::warn!("GetModuleFileNameA(): {}", err);
            0
        }
    }
}

#[win32_derive::dllexport]
pub fn GetModuleFileNameW(
    _machine: &mut Machine,
    hModule: HMODULE,
    _lpFilename: u32,
    _nSize: u32,
) -> u32 {
    if !hModule.is_null() {
        log::error!("unimplemented: GetModuleHandleW(non-null)")
    }
    0 // fail
}

#[win32_derive::dllexport]
pub fn GetModuleHandleA(machine: &mut Machine, lpModuleName: Option<&str>) -> HMODULE {
    if let Some(name) = lpModuleName {
        log::error!("unimplemented: GetModuleHandle({name:?})");
        return HMODULE::null();
    }
    // HMODULE is base address of current module.
    HMODULE::from_raw(machine.state.kernel32.image_base)
}

#[win32_derive::dllexport]
pub fn GetModuleHandleW(machine: &mut Machine, lpModuleName: Option<Str16>) -> HMODULE {
    let ascii = lpModuleName.map(|str| str.to_string());
    GetModuleHandleA(machine, ascii.as_deref())
}

#[win32_derive::dllexport]
pub fn GetModuleHandleExW(
    machine: &mut Machine,
    dwFlags: u32,
    lpModuleName: Option<Str16>,
    hModule: Option<&mut HMODULE>,
) -> bool {
    if dwFlags != 0 {
        unimplemented!("GetModuleHandleExW flags {dwFlags:x}");
    }
    let hMod = GetModuleHandleW(machine, lpModuleName);
    if let Some(out) = hModule {
        *out = hMod;
    }
    return !hMod.is_null();
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
unsafe impl x86::Pod for STARTUPINFOA {}

#[win32_derive::dllexport]
pub fn GetStartupInfoA(machine: &mut Machine, lpStartupInfo: u32) -> u32 {
    let ofs = lpStartupInfo as usize;
    let size = std::mem::size_of::<STARTUPINFOA>();
    machine.x86.mem[ofs..ofs + size].fill(0);

    let info = machine.x86.mem.view_mut::<STARTUPINFOA>(ofs as u32);
    info.cb = size as u32;
    0
}

#[win32_derive::dllexport]
pub fn GetStartupInfoW(machine: &mut Machine, lpStartupInfo: u32) -> u32 {
    let ofs = lpStartupInfo as usize;
    // STARTUPINFOA is the same shape as the W one, just the strings are different...
    let size = std::mem::size_of::<STARTUPINFOA>();
    machine.x86.mem[ofs..ofs + size].fill(0);

    let info = machine.x86.mem.view_mut::<STARTUPINFOA>(ofs as u32);
    info.cb = size as u32;
    0
}

#[derive(Debug, FromPrimitive)]
pub enum ProcessorFeature {
    FLOATING_POINT_PRECISION_ERRATA = 0,
    FLOATING_POINT_EMULATED = 1,
    COMPARE_EXCHANGE_DOUBLE = 2,
    MMX_INSTRUCTIONS_AVAILABLE = 3,
    PPC_MOVEMEM_64BIT_OK = 4,
    ALPHA_BYTE_INSTRUCTIONS = 5,
    XMMI_INSTRUCTIONS_AVAILABLE = 6,
    _3DNOW_INSTRUCTIONS_AVAILABLE = 7,
    RDTSC_INSTRUCTION_AVAILABLE = 8,
    PAE_ENABLED = 9,
    XMMI64_INSTRUCTIONS_AVAILABLE = 10,
    SSE_DAZ_MODE_AVAILABLE = 11,
    NX_ENABLED = 12,
    SSE3_INSTRUCTIONS_AVAILABLE = 13,
    COMPARE_EXCHANGE128 = 14,
    COMPARE64_EXCHANGE128 = 15,
    CHANNELS_ENABLED = 16,
    XSAVE_ENABLED = 17,
    ARM_VFP_32_REGISTERS_AVAILABLE = 18,
    ARM_NEON_INSTRUCTIONS_AVAILABLE = 19,
    SECOND_LEVEL_ADDRESS_TRANSLATION = 20,
    VIRT_FIRMWARE_ENABLED = 21,
    RDWRFSGSBASE_AVAILABLE = 22,
    FASTFAIL_AVAILABLE = 23,
    ARM_DIVIDE_INSTRUCTION_AVAILABLE = 24,
    ARM_64BIT_LOADSTORE_ATOMIC = 25,
    ARM_EXTERNAL_CACHE_AVAILABLE = 26,
    ARM_FMAC_INSTRUCTIONS_AVAILABLE = 27,
    RDRAND_INSTRUCTION_AVAILABLE = 28,
    ARM_V8_INSTRUCTIONS_AVAILABLE = 29,
    ARM_V8_CRYPTO_INSTRUCTIONS_AVAILABLE = 30,
    ARM_V8_CRC32_INSTRUCTIONS_AVAILABLE = 31,
    RDTSCP_INSTRUCTION_AVAILABLE = 32,
}

#[win32_derive::dllexport]
pub fn IsProcessorFeaturePresent(_machine: &mut Machine, feature: u32) -> bool {
    let feature = ProcessorFeature::from_u32(feature).unwrap();
    log::warn!("IsProcessorFeaturePresent({feature:?}) => false");
    false
}

#[win32_derive::dllexport]
pub fn IsDebuggerPresent(_machine: &mut Machine) -> bool {
    true // Might cause a binary to log info via the debug API? Not sure.
}

#[win32_derive::dllexport]
pub fn GetCurrentThreadId(_machine: &mut Machine) -> u32 {
    1
}

#[win32_derive::dllexport]
pub fn GetCurrentProcessId(_machine: &mut Machine) -> u32 {
    1
}

#[win32_derive::dllexport]
pub fn GetStdHandle(_machine: &mut Machine, nStdHandle: u32) -> HFILE {
    match nStdHandle as i32 {
        -10 => STDIN_HFILE,
        -11 => STDOUT_HFILE,
        -12 => STDERR_HFILE,
        _ => HFILE::invalid(),
    }
}

#[win32_derive::dllexport]
pub fn GetTickCount(machine: &mut Machine) -> u32 {
    machine.host.time()
}

#[win32_derive::dllexport]
pub fn QueryPerformanceCounter(_machine: &mut Machine, _ptr: u32) -> bool {
    true // success
}

#[repr(C)]
pub struct FILETIME {
    dwLowDateTime: DWORD,
    dwHighDateTime: DWORD,
}
unsafe impl x86::Pod for FILETIME {}
#[win32_derive::dllexport]
pub fn GetSystemTimeAsFileTime(_machine: &mut Machine, _time: Option<&mut FILETIME>) -> u32 {
    0
}

#[win32_derive::dllexport]
pub fn GetVersion(_machine: &mut Machine) -> u32 {
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

#[win32_derive::dllexport]
pub fn GetVersionExA(machine: &mut Machine, lpVersionInformation: u32) -> u32 {
    let ofs = lpVersionInformation as usize;
    let size = machine.x86.read_u32(lpVersionInformation) as usize;
    if size < std::mem::size_of::<OSVERSIONINFO>() {
        log::error!("GetVersionExA undersized buffer");
        return 0;
    }
    machine.x86.mem[ofs..ofs + size].fill(0);

    let buf = &mut machine.x86.mem[ofs..ofs + std::mem::size_of::<OSVERSIONINFO>()];
    let info: &mut OSVERSIONINFO =
        unsafe { (buf.as_mut_ptr() as *mut OSVERSIONINFO).as_mut().unwrap() };

    info.dwOSVersionInfoSize = size as u32;
    info.dwMajorVersion = 6; // ? pulled from debugger
    info.dwPlatformId = 2 /* VER_PLATFORM_WIN32_NT */;

    1
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

#[win32_derive::dllexport]
pub fn HeapCreate(
    machine: &mut Machine,
    flOptions: u32,
    dwInitialSize: u32,
    _dwMaximumSize: u32,
) -> u32 {
    let _flags = HeapCreateFlags::from_bits(flOptions).unwrap();
    // Currently none of the flags will affect behavior, but we might need to revisit this
    // with exceptions or threads support...
    //log::info!("HeapCreate({flags:x}, {dwInitialSize:x}, {dwMaximumSize:x})");
    machine.state.kernel32.new_heap(
        &mut machine.x86.mem,
        dwInitialSize as usize,
        "HeapCreate".into(),
    )
}

#[win32_derive::dllexport]
pub fn HeapDestroy(_machine: &mut Machine, hHeap: u32) -> u32 {
    log::warn!("HeapDestroy({hHeap:x})");
    1 // success
}

#[win32_derive::dllexport]
pub fn GetProcessHeap(machine: &mut Machine) -> u32 {
    let heap = peb_mut(machine).ProcessHeap;
    if heap != 0 {
        return heap;
    }
    let size = 1 << 20;
    let heap = machine
        .state
        .kernel32
        .new_heap(&mut machine.x86.mem, size, "process heap".into());
    peb_mut(machine).ProcessHeap = heap;
    heap
}

#[win32_derive::dllexport]
pub fn LoadLibraryA(machine: &mut Machine, filename: Option<&str>) -> u32 {
    let filename = filename.unwrap();
    let filename = filename.to_ascii_lowercase();

    if let Some(index) = crate::winapi::DLLS
        .iter()
        .position(|dll| dll.file_name == filename)
    {
        return (index + 1) as u32;
    }

    log::error!(
        "LoadLibrary({filename:?}) => {:x}",
        machine.x86.mem.read_u32(machine.x86.regs.esp - 4)
    );
    0 // fail
}

#[win32_derive::dllexport]
pub fn LoadLibraryExW(
    _machine: &mut Machine,
    lpLibFileName: Option<Str16>,
    hFile: HFILE,
    dwFlags: u32,
) -> u32 {
    log::error!("LoadLibraryExW({lpLibFileName:?}, {hFile:x?}, {dwFlags:x})");
    0 // fail
}

#[win32_derive::dllexport]
pub fn GetProcAddress(machine: &mut Machine, hModule: HMODULE, lpProcName: Option<&str>) -> u32 {
    let proc_name = lpProcName.unwrap();
    if let Some(dll) = winapi::DLLS.get(hModule.to_raw() as usize - 1) {
        // See if the symbol was already imported.
        let full_name = format!("{}!{}", dll.file_name, proc_name);
        if let Some(addr) = machine.shims.lookup(&full_name) {
            return addr;
        }

        let handler = (dll.resolve)(&winapi::ImportSymbol::Name(proc_name));
        let addr = machine.shims.add(full_name, handler);
        return addr;
    }
    log::error!("GetProcAddress({:x?}, {:?})", hModule, lpProcName);
    0 // fail
}

#[win32_derive::dllexport]
pub fn SetHandleCount(_machine: &mut Machine, uNumber: u32) -> u32 {
    // "For Windows Win32 systems, this API has no effect."
    uNumber
}

const GENERIC_READ: u32 = 0x8000_0000;

#[derive(Debug)]
pub enum CreationDisposition {
    OPEN_EXISTING = 3,
}
impl TryFrom<u32> for CreationDisposition {
    type Error = u32;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(match value {
            x if x == CreationDisposition::OPEN_EXISTING as u32 => {
                CreationDisposition::OPEN_EXISTING
            }
            _ => return Err(value),
        })
    }
}

pub const FILE_ATTRIBUTE_NORMAL: u32 = 0x80;

#[win32_derive::dllexport]
pub fn CreateFileW(
    _machine: &mut Machine,
    lpFileName: Option<Str16>,
    dwDesiredAccess: u32,
    _dwShareMode: u32,
    _lpSecurityAttributes: u32,
    dwCreationDisposition: Result<CreationDisposition, u32>,
    dwFlagsAndAttributes: u32,
    hTemplateFile: HFILE,
) -> HFILE {
    if dwDesiredAccess != GENERIC_READ {
        unimplemented!("CreateFileW access {:x}", dwDesiredAccess);
    }
    let dwCreationDisposition = match dwCreationDisposition {
        Err(x) => unimplemented!("dwCreationDisposition {x:?}"),
        Ok(disp) => disp,
    };
    if dwFlagsAndAttributes != FILE_ATTRIBUTE_NORMAL {
        unimplemented!("dwFlagsAndAttributes {dwFlagsAndAttributes:x}");
    }
    if !hTemplateFile.is_null() {
        unimplemented!("hTemplateFile {hTemplateFile:?}");
    }
    log::error!("CreateFileW {lpFileName:?} {dwDesiredAccess:x} {dwCreationDisposition:x?} {dwFlagsAndAttributes:x} {hTemplateFile:?}");
    HFILE::invalid()
}

#[win32_derive::dllexport]
pub fn WriteFile(
    machine: &mut Machine,
    hFile: HFILE,
    lpBuffer: Option<&[u8]>,
    lpNumberOfBytesWritten: Option<&mut u32>,
    lpOverlapped: u32,
) -> bool {
    assert!(hFile == STDOUT_HFILE || hFile == STDERR_HFILE);
    assert!(lpOverlapped == 0);

    let n = machine.host.write(lpBuffer.unwrap());

    // The docs say this parameter may not be null, but a test program with the param as null
    // runs fine on real Windows...
    if let Some(written) = lpNumberOfBytesWritten {
        *written = n as u32;
    }
    true
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
pub fn OutputDebugStringA(_machine: &mut Machine, msg: Option<&str>) -> u32 {
    log::warn!("OutputDebugStringA: {:?}", msg);
    0
}

#[win32_derive::dllexport]
pub fn InitializeCriticalSectionAndSpinCount(
    _machine: &mut Machine,
    _lpCriticalSection: u32,
    _dwSpinCount: u32,
) -> bool {
    // "On single-processor systems, the spin count is ignored and the critical section spin count is set to 0 (zero)."
    // "This function always succeeds and returns a nonzero value."
    true
}

#[win32_derive::dllexport]
pub fn DeleteCriticalSection(_machine: &mut Machine, _lpCriticalSection: u32) -> u32 {
    0
}

#[win32_derive::dllexport]
pub fn EnterCriticalSection(_machine: &mut Machine, _lpCriticalSection: u32) -> u32 {
    0
}

#[win32_derive::dllexport]
pub fn LeaveCriticalSection(_machine: &mut Machine, _lpCriticalSection: u32) -> u32 {
    0
}

#[win32_derive::dllexport]
pub fn SetUnhandledExceptionFilter(_machine: &mut Machine, _lpTopLevelExceptionFilter: u32) -> u32 {
    0 // No current handler.
}

#[win32_derive::dllexport]
pub fn UnhandledExceptionFilter(_machine: &mut Machine, _exceptionInfo: u32) -> u32 {
    // "The process is being debugged, so the exception should be passed (as second chance) to the application's debugger."
    0 // EXCEPTION_CONTINUE_SEARCH
}

#[win32_derive::dllexport]
pub fn NtCurrentTeb(machine: &mut Machine) -> u32 {
    machine.state.kernel32.teb
}

#[win32_derive::dllexport]
pub fn TlsAlloc(machine: &mut Machine) -> u32 {
    let peb = peb_mut(machine);
    let slot = peb.TlsCount;
    peb.TlsCount = slot + 1;
    slot
}

#[win32_derive::dllexport]
pub fn TlsFree(machine: &mut Machine, dwTlsIndex: u32) -> bool {
    let peb = peb_mut(machine);
    if dwTlsIndex >= peb.TlsCount {
        log::warn!("TlsFree of unknown slot {dwTlsIndex}");
        return false;
    }
    // TODO
    true
}

#[win32_derive::dllexport]
pub fn TlsSetValue(machine: &mut Machine, dwTlsIndex: u32, lpTlsValue: u32) -> bool {
    let teb = teb_mut(machine);
    teb.TlsSlots[dwTlsIndex as usize] = lpTlsValue;
    true
}

#[win32_derive::dllexport]
pub fn TlsGetValue(machine: &mut Machine, dwTlsIndex: u32) -> u32 {
    let teb = teb_mut(machine);
    teb.TlsSlots[dwTlsIndex as usize]
}

// TODO: this has a bunch of synchronization magic that I haven't implemented,
// but I did at least make this struct the right size (128 bits).
#[repr(C)]
pub struct SLIST_HEADER {
    Next: u32,
    todo: [u32; 3],
}
unsafe impl x86::Pod for SLIST_HEADER {}

#[win32_derive::dllexport]
pub fn InitializeSListHead(_machine: &mut Machine, ListHead: Option<&mut SLIST_HEADER>) -> u32 {
    ListHead.unwrap().Next = 0;
    0
}

/// The system default Windows ANSI code page.
const CP_ACP: u32 = 0;

#[win32_derive::dllexport]
pub fn MultiByteToWideChar(
    machine: &mut Machine,
    CodePage: u32,
    _dwFlags: u32,
    lpMultiByteStr: u32,
    cbMultiByte: i32,
    mut lpWideCharStr: Option<&mut [u16]>,
) -> u32 {
    if CodePage != CP_ACP && CodePage != 1252 {
        unimplemented!("MultiByteToWideChar code page {CodePage}");
    }
    // TODO: dwFlags

    let input = match cbMultiByte {
        0 => return 0, // TODO: invalid param
        -1 => machine.x86.mem[lpMultiByteStr as usize..].read_strz_with_nul(),
        len => std::str::from_utf8(
            &machine.x86.mem[lpMultiByteStr as usize..lpMultiByteStr as usize + len as usize],
        )
        .unwrap(),
    };

    match lpWideCharStr {
        Some(buf) if buf.len() == 0 => lpWideCharStr = None,
        _ => (),
    };

    match lpWideCharStr {
        None => input.len() as u32,
        Some(buf) => {
            let mut len = 0;
            for (c_in, c_out) in std::iter::zip(input.bytes(), buf) {
                if c_in > 0x7f {
                    unimplemented!("unicode");
                }
                *c_out = c_in as u16;
                len += 1;
            }
            len
        }
    }
}

#[win32_derive::dllexport]
pub fn WriteConsoleW(
    machine: &mut Machine,
    hConsoleOutput: HFILE,
    lpBuffer: Option<&[u16]>,
    lpNumberOfCharsWritten: Option<&mut u32>,
    _lpReserved: u32,
) -> bool {
    let buf = Str16::from_buffer(lpBuffer.unwrap()).to_string();
    let mut bytes_written = 0;
    if !WriteFile(
        machine,
        hConsoleOutput,
        Some(buf.as_bytes()),
        Some(&mut bytes_written),
        0,
    ) {
        return false;
    }
    if let Some(chars_written) = lpNumberOfCharsWritten {
        *chars_written = bytes_written;
    }
    return bytes_written == buf.len() as u32;
}

#[win32_derive::dllexport]
pub fn CreateThread(
    _machine: &mut Machine,
    lpThreadAttributes: u32,
    dwStackSize: u32,
    lpStartAddress: u32,
    lpParameter: u32,
    dwCreationFlags: u32,
    lpThreadId: u32,
) -> u32 {
    log::warn!("CreateThread {lpThreadAttributes:x} {dwStackSize:x} {lpStartAddress:x} {lpParameter:x} {dwCreationFlags:x} {lpThreadId:x}");
    0
}

#[win32_derive::dllexport]
pub fn SetThreadPriority(_machine: &mut Machine, _hThread: u32, _nPriority: u32) -> bool {
    true // success
}
