#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

mod dll;
mod file;
mod memory;
mod thread;

use super::{
    alloc::Alloc,
    alloc::ArenaInfo,
    alloc::{Heap, HeapInfo},
    types::*,
};
use crate::machine::Machine;
use num_traits::FromPrimitive;
use std::{collections::HashMap, io::Write};
use x86::{
    Mem, Pod, {Memory, VecMem},
};

pub use dll::*;
pub use file::*;
pub use memory::*;
pub use thread::*;

const TRACE_CONTEXT: &'static str = "kernel32";

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

    #[serde(skip)] // TODO
    pub dlls: Vec<DLL>,

    #[serde(skip)] // TODO
    files: HashMap<HFILE, Box<dyn crate::host::File>>,

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
            dlls: Vec::new(),
            files: HashMap::new(),
            env: 0,
            cmdline: 0,
            cmdline16: 0,
        }
    }

    pub fn init(&mut self, mem: &mut VecMem) {
        let mapping = self.mappings.alloc(0x1000, "kernel32 data".into(), mem);
        self.arena = ArenaInfo::new(mapping.addr, mapping.size);

        let env = "\0\0".as_bytes();
        let env_addr = self.arena.get(mem).alloc(env.len() as u32);
        mem.slice_mut(env_addr..)
            .slice_mut(..env.len() as u32)
            .as_mut_slice_todo()
            .copy_from_slice(env);
        self.env = env_addr;
    }

    fn init_cmdline(&mut self, mem: &mut Mem, mut cmdline: String) {
        // Gross: GetCommandLineA() needs to return a pointer that's never freed,
        // so we need to hang on to both versions of the command line.

        cmdline.push(0 as char); // nul terminator

        self.cmdline = self.arena.get(mem).alloc(cmdline.len() as u32);
        mem.slice_mut(self.cmdline..)
            .slice_mut(..cmdline.len() as u32)
            .as_mut_slice_todo()
            .copy_from_slice(cmdline.as_bytes());

        let cmdline16 = String16::from(&cmdline);
        self.cmdline16 = self.arena.get(mem).alloc(cmdline16.byte_size() as u32);
        let mem16: &mut [u16] = unsafe {
            std::mem::transmute(
                mem.slice_mut(self.cmdline16..)
                    .slice_mut(..cmdline16.0.len() as u32)
                    .as_mut_slice_todo(),
            )
        };
        mem16.copy_from_slice(&cmdline16.0);
    }

    /// Set up TEB, PEB, and other process info.
    /// The FS register points at the TEB (thread info), which points at the PEB (process info).
    pub fn init_process(&mut self, mem: &mut Mem, cmdline: String) {
        let cmdline_len = cmdline.len();

        self.init_cmdline(mem, cmdline);

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

    pub fn new_private_heap(&mut self, mem: &mut VecMem, size: usize, desc: String) -> HeapInfo {
        let mapping = self.mappings.alloc(size as u32, desc, mem);
        HeapInfo::new(mem, mapping.addr, mapping.size)
    }

    pub fn new_heap(&mut self, mem: &mut VecMem, size: usize, desc: String) -> u32 {
        let heap = self.new_private_heap(mem, size, desc);
        let addr = heap.addr;
        self.heaps.insert(addr, heap);
        addr
    }

    pub fn get_heap<'a>(&'a mut self, mem: &'a mut Mem, addr: u32) -> Option<Heap<'a>> {
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
        log::error!("unimplemented: GetModuleFileNameW(non-null)")
    }
    0 // fail
}

#[repr(C)]
#[derive(Debug)]
pub struct STARTUPINFOA {
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
pub fn GetStartupInfoA(_machine: &mut Machine, lpStartupInfo: Option<&mut STARTUPINFOA>) -> u32 {
    // MSVC runtime library passes in uninitialized memory for lpStartupInfo, so don't trust info.cb.
    let info = lpStartupInfo.unwrap();
    let len = std::cmp::min(info.cb, std::mem::size_of::<STARTUPINFOA>() as u32);
    unsafe { info.clear_memory(len) };
    0
}

#[win32_derive::dllexport]
pub fn GetStartupInfoW(machine: &mut Machine, lpStartupInfo: Option<&mut STARTUPINFOA>) -> u32 {
    // STARTUPINFOA is the same shape as the W one, just the strings are different...
    GetStartupInfoA(machine, lpStartupInfo)
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
pub fn GetCurrentProcessId(_machine: &mut Machine) -> u32 {
    1
}

#[win32_derive::dllexport]
pub fn GetTickCount(machine: &mut Machine) -> u32 {
    machine.host.time()
}

// The number of "counts" per second, where counts are the units returned by
// QueryPerformanceCounter.  On my Windows machine this value was 10m, which
// is to say a count is 0.1us.
const QUERY_PERFORMANCE_FREQ: u32 = 10_000_000;

#[win32_derive::dllexport]
pub fn QueryPerformanceCounter(
    machine: &mut Machine,
    lpPerformanceCount: Option<&mut u64>,
) -> bool {
    let ms = machine.host.time();
    *lpPerformanceCount.unwrap() = ms as u64 * (QUERY_PERFORMANCE_FREQ as u64 / 1000);
    true // success
}

#[win32_derive::dllexport]
pub fn QueryPerformanceFrequency(machine: &mut Machine, lpFrequency: u32) -> bool {
    // 64-bit write
    machine
        .x86
        .mem
        .write_u32(lpFrequency, QUERY_PERFORMANCE_FREQ);
    machine.x86.mem.write_u32(lpFrequency + 4, 0);
    true
}

#[repr(C)]
#[derive(Debug)]
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
#[derive(Debug)]
pub struct OSVERSIONINFO {
    dwOSVersionInfoSize: DWORD,
    dwMajorVersion: DWORD,
    dwMinorVersion: DWORD,
    dwBuildNumber: DWORD,
    dwPlatformId: DWORD,
    //szCSDVersion: [u8; 128],
}
unsafe impl Pod for OSVERSIONINFO {}

#[win32_derive::dllexport]
pub fn GetVersionExA(
    _machine: &mut Machine,
    lpVersionInformation: Option<&mut OSVERSIONINFO>,
) -> u32 {
    let info = lpVersionInformation.unwrap();
    if info.dwOSVersionInfoSize < std::mem::size_of::<OSVERSIONINFO>() as u32 {
        log::error!("GetVersionExA undersized buffer");
        return 0;
    }
    unsafe { info.clear_memory(info.dwOSVersionInfoSize) };

    info.dwMajorVersion = 6; // ? pulled from debugger
    info.dwPlatformId = 2 /* VER_PLATFORM_WIN32_NT */;

    1
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
pub fn SetHandleCount(_machine: &mut Machine, uNumber: u32) -> u32 {
    // "For Windows Win32 systems, this API has no effect."
    uNumber
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

// TODO: this has a bunch of synchronization magic that I haven't implemented,
// but I did at least make this struct the right size (128 bits).
#[repr(C)]
#[derive(Debug)]
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
        -1 => machine.x86.mem.slice(lpMultiByteStr..).read_strz_with_nul(),
        len => std::str::from_utf8(
            &machine
                .x86
                .mem
                .slice(lpMultiByteStr..)
                .slice(..len as u32)
                .as_slice_todo(),
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
