#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

mod dll;
mod file;
mod memory;
mod thread;

use super::{
    alloc::Arena,
    heap::Heap,
    stack_args::{ArrayWithSize, ArrayWithSizeMut},
    types::*,
};
use crate::{
    machine::{Machine, MemImpl},
    segments::SegmentDescriptor,
};
use ::memory::{Mem, Pod};
use num_traits::FromPrimitive;
use std::{collections::HashMap, io::Write};

pub use self::memory::*;
pub use dll::*;
pub use file::*;
pub use thread::*;

const TRACE_CONTEXT: &'static str = "kernel32";

/// Process command line, as exposed in GetCommandLine() and also TEB.
/// Gross: GetCommandLineA() needs to return a pointer that's never freed,
/// so we need to hang on to both versions of the command line.
#[derive(serde::Serialize, serde::Deserialize)]
struct CommandLine {
    /// Command line, ASCII.
    cmdline: u32,
    /// Command line, UTF16.
    cmdline16: u32,
    /// Length without trailing nul.
    len: usize,
}

impl CommandLine {
    fn new(mut cmdline: String, arena: &mut Arena, mem: Mem) -> Self {
        let len = cmdline.len();

        cmdline.push(0 as char); // nul terminator

        let cmdline8_ptr = arena.alloc(cmdline.len() as u32, 1);
        mem.sub(cmdline8_ptr, cmdline.len() as u32)
            .as_mut_slice_todo()
            .copy_from_slice(cmdline.as_bytes());

        let cmdline16 = String16::from(&cmdline);
        let cmdline16_ptr = arena.alloc(cmdline16.byte_size() as u32, 2);
        let mem16: &mut [u16] = unsafe {
            std::mem::transmute(
                mem.sub(cmdline16_ptr, cmdline16.0.len() as u32)
                    .as_mut_slice_todo(),
            )
        };
        mem16.copy_from_slice(&cmdline16.0);

        CommandLine {
            cmdline: cmdline8_ptr,
            cmdline16: cmdline16_ptr,
            len,
        }
    }

    fn as_unicode_string(&self) -> UNICODE_STRING {
        UNICODE_STRING {
            Length: self.len as u16,
            MaximumLength: self.len as u16,
            Buffer: self.cmdline16,
        }
    }
}

/// Set up TEB, PEB, and other process info.
/// The FS register points at the TEB (thread info), which points at the PEB (process info).
fn init_teb(cmdline: &CommandLine, arena: &mut Arena, mem: Mem) -> u32 {
    // RTL_USER_PROCESS_PARAMETERS
    let params_addr = arena.alloc(
        std::cmp::max(
            std::mem::size_of::<RTL_USER_PROCESS_PARAMETERS>() as u32,
            0x100,
        ),
        4,
    );
    let params = mem.view_mut::<RTL_USER_PROCESS_PARAMETERS>(params_addr);
    // x86.put::<u32>(params_addr + 0x10, console_handle);
    // x86.put::<u32>(params_addr + 0x14, console_flags);
    // x86.put::<u32>(params_addr + 0x18, stdin);
    params.hStdOutput = STDOUT_HFILE;
    params.hStdError = STDERR_HFILE;
    params.ImagePathName.clear();
    params.CommandLine = cmdline.as_unicode_string();

    // PEB
    let peb_addr = arena.alloc(std::cmp::max(std::mem::size_of::<PEB>() as u32, 0x100), 4);
    let peb = mem.view_mut::<PEB>(peb_addr);
    peb.ProcessParameters = params_addr;
    peb.ProcessHeap = 0; // Initialized lazily.
    peb.TlsCount = 0;

    // SEH chain
    let seh_addr = arena.alloc(
        std::mem::size_of::<_EXCEPTION_REGISTRATION_RECORD>() as u32,
        4,
    );
    let seh = mem.view_mut::<_EXCEPTION_REGISTRATION_RECORD>(seh_addr);
    seh.Prev = 0xFFFF_FFFF;
    seh.Handler = 0xFF5E_5EFF; // Hopefully easier to spot.

    // TEB
    let teb_addr = arena.alloc(std::cmp::max(std::mem::size_of::<TEB>() as u32, 0x100), 4);
    let teb = mem.view_mut::<TEB>(teb_addr);
    teb.Tib.ExceptionList = seh_addr;
    teb.Tib._Self = teb_addr; // Confusing: it points to itself.
    teb.Peb = peb_addr;

    teb_addr
    // log::info!("params {params_addr:x} peb {peb_addr:x} teb {teb_addr:x}");
}

/// Result of setting up the GDT, with initial values for all the relevant segment registers.
pub struct GDTEntries {
    /// Address of GDT itself.
    pub addr: u32,
    pub cs: u16,
    pub ds: u16,
    pub fs: u16,
    pub ss: u16,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct State {
    /// Memory for kernel32 data structures.
    arena: Arena,
    /// Address image was loaded at.
    pub image_base: u32,
    /// Address of TEB (what FS register-relative addresses refer to).
    pub teb: u32,
    pub mappings: Mappings,
    /// Heaps created by HeapAlloc().
    heaps: HashMap<u32, Heap>,

    #[serde(skip)] // TODO
    pub dlls: Vec<DLL>,

    #[serde(skip)] // TODO
    files: HashMap<HFILE, Box<dyn crate::host::File>>,

    #[serde(skip)]
    #[cfg(feature = "x86-64")]
    pub ldt: crate::ldt::LDT,

    env: u32,

    cmdline: CommandLine,
}

impl State {
    pub fn new(mem: &mut MemImpl, cmdline: String) -> Self {
        let mut mappings = Mappings::new();
        let mapping = mappings.alloc(0x1000, "kernel32 data".into(), mem);
        let mut arena = Arena::new(mapping.addr, mapping.size);

        let env = "\0\0".as_bytes();
        let env_addr = arena.alloc(env.len() as u32, 1);
        mem.mem()
            .sub(env_addr, env.len() as u32)
            .as_mut_slice_todo()
            .copy_from_slice(env);

        let cmdline = CommandLine::new(cmdline, &mut arena, mem.mem());

        let teb = init_teb(&cmdline, &mut arena, mem.mem());

        #[cfg(feature = "x86-64")]
        let ldt = {
            let mut ldt = crate::ldt::LDT::default();

            // NOTE: OSX seems extremely sensitive to the values used here, where like
            // using a span size that is not exactly 0xFFF causes the entry to be rejected.
            let fs_sel = ldt.add_entry(teb, 0xFFF, false);
            unsafe {
                std::arch::asm!(
                    "mov fs,{fs_sel:x}",
                    fs_sel = in(reg) fs_sel
                );
            }

            ldt
        };

        State {
            arena,
            image_base: 0,
            teb,
            mappings,
            heaps: HashMap::new(),
            dlls: Vec::new(),
            files: HashMap::new(),
            env: env_addr,
            cmdline,
            #[cfg(feature = "x86-64")]
            ldt,
        }
    }

    pub fn new_private_heap(&mut self, mem: &mut MemImpl, size: usize, desc: String) -> Heap {
        let mapping = self.mappings.alloc(size as u32, desc, mem);
        Heap::new(mapping.addr, mapping.size)
    }

    pub fn new_heap(&mut self, mem: &mut MemImpl, size: usize, desc: String) -> u32 {
        let heap = self.new_private_heap(mem, size, desc);
        let addr = heap.addr;
        self.heaps.insert(addr, heap);
        addr
    }

    pub fn get_heap<'a>(&'a mut self, addr: u32) -> Option<&mut Heap> {
        self.heaps.get_mut(&addr)
    }

    pub fn create_gdt(&mut self, mem: Mem) -> GDTEntries {
        const COUNT: usize = 5;
        let addr = self.arena.alloc(COUNT as u32 * 8, 8);
        let gdt: &mut [u64; COUNT] = unsafe { &mut *(mem.ptr_mut::<u64>(addr) as *mut _) };

        gdt[0] = 0;

        let cs = (1 << 3) | 0b011;
        gdt[1] = SegmentDescriptor {
            base: 0x0000_0000,
            limit: 0xFFFF_FFFF,
            granularity: true,
            db: true, // 32 bit
            long: false,
            available: false,
            present: true,
            dpl: 3,
            system: true,  // code/data
            type_: 0b1011, // code, execute/read, accessed
        }
        .encode();

        let ds = (2 << 3) | 0b011;
        gdt[2] = SegmentDescriptor {
            base: 0x0000_0000,
            limit: 0xFFFF_FFFF,
            granularity: true,
            db: true, // 32 bit
            long: false,
            available: false,
            present: true,
            dpl: 3,
            system: true,  // code/data
            type_: 0b0011, // data, read/write, accessed
        }
        .encode();

        let fs = (3 << 3) | 0b011;
        gdt[3] = SegmentDescriptor {
            base: self.teb,
            limit: 0x1000,
            granularity: false,
            db: true, // 32 bit
            long: false,
            available: false,
            present: true,
            dpl: 3,
            system: true,  // code/data
            type_: 0b0011, // data, read/write, accessed
        }
        .encode();

        // unicorn test says: "when setting SS, need rpl == cpl && dpl == cpl",
        // which is to say because the system is level 0 (cpl) we need the descriptor
        // to also be zero (dpl) and the selector to also be zero (rpl, the 0b000 here).
        let ss = (4 << 3) | 0b000;
        gdt[4] = SegmentDescriptor {
            base: 0x0000_0000,
            limit: 0xFFFF_FFFF,
            granularity: true,
            db: true, // 32 bit
            long: false,
            available: false,
            present: true,
            dpl: 0,        // NOTE: this is different from others
            system: true,  // code/data
            type_: 0b0011, // data, read/write, accessed
        }
        .encode();

        GDTEntries {
            addr,
            cs,
            ds,
            fs,
            ss,
        }
    }
}

fn teb(machine: &Machine) -> &TEB {
    machine.mem().view::<TEB>(machine.state.kernel32.teb)
}
fn teb_mut(machine: &mut Machine) -> &mut TEB {
    machine.mem().view_mut::<TEB>(machine.state.kernel32.teb)
}
pub fn peb_mut(machine: &mut Machine) -> &mut PEB {
    let peb_addr = teb(machine).Peb;
    machine.mem().view_mut::<PEB>(peb_addr)
}

#[repr(C)]
pub struct PEB {
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
unsafe impl ::memory::Pod for PEB {}

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
unsafe impl ::memory::Pod for NT_TIB {}

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
unsafe impl ::memory::Pod for TEB {}

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
unsafe impl ::memory::Pod for RTL_USER_PROCESS_PARAMETERS {}

#[repr(C)]
struct _EXCEPTION_REGISTRATION_RECORD {
    Prev: DWORD,
    Handler: DWORD,
}
unsafe impl ::memory::Pod for _EXCEPTION_REGISTRATION_RECORD {}

/// This function is not part of the Windows API, but is rather just how retrowin32
/// starts/stops a process.  It probably has some better name within ntdll.dll.
#[win32_derive::dllexport]
pub async fn retrowin32_main(machine: &mut Machine, entry_point: u32) -> u32 {
    // Iterate list of dlls by index, because in principle invoking dllmains might load more
    // dlls(?).
    for i in 0.. {
        let dllmain = match machine.state.kernel32.dlls.get(i) {
            None => break,
            Some(m) if m.dll.entry_point == 0 => continue,
            Some(m) => m.dll.entry_point,
        };
        log::info!("invoking dllmain {:x}", dllmain);
        let hInstance = 0u32; // TODO
        let fdwReason = 1u32; // DLL_PROCESS_ATTACH
        let lpvReserved = 0u32;
        machine
            .call_x86(dllmain, vec![hInstance, fdwReason, lpvReserved])
            .await;
    }

    machine.call_x86(entry_point, vec![]).await;
    // TODO: if the entry point returns, the Windows behavior is to wait for any
    // spawned threads before exiting.
    ExitProcess(machine, 0);
    0
}

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
    #[cfg(feature = "x86-emu")]
    machine.emu.x86.cpu.err("ExitProcess".into());
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
    machine.state.kernel32.cmdline.cmdline
}

#[win32_derive::dllexport]
pub fn GetCommandLineW(machine: &mut Machine) -> u32 {
    machine.state.kernel32.cmdline.cmdline16
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
    buf: ArrayWithSize<u8>,
) -> u32 {
    println!("name {:?} buf {:?}", name, buf);
    0
}

#[win32_derive::dllexport]
pub fn GetModuleFileNameA(
    _machine: &mut Machine,
    hModule: HMODULE,
    filename: ArrayWithSizeMut<u8>,
) -> u32 {
    assert!(hModule.is_null());
    match filename.unwrap().write(b"TODO.exe\0") {
        Ok(n) => n as u32,
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
unsafe impl ::memory::Pod for STARTUPINFOA {}

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

// In principle we could just use an i64 here, but when Windows passes one of these via
// the stack it may align it on a 4-byte address when Rust requires 8-byte alignment for
// 64-bit addresses.  So instead we more closely match the Windows behavior.
#[repr(C)]
#[derive(Debug)]
pub struct LARGE_INTEGER {
    LowPart: u32,
    HighPart: i32,
}
unsafe impl Pod for LARGE_INTEGER {}

#[win32_derive::dllexport]
pub fn QueryPerformanceCounter(
    machine: &mut Machine,
    lpPerformanceCount: Option<&mut LARGE_INTEGER>,
) -> bool {
    let counter = lpPerformanceCount.unwrap();
    let ms = machine.host.time();
    let counts = ms as u64 * (QUERY_PERFORMANCE_FREQ as u64 / 1000);
    counter.LowPart = counts as u32;
    counter.HighPart = (counts >> 32) as u32 as i32;
    true // success
}

#[win32_derive::dllexport]
pub fn QueryPerformanceFrequency(machine: &mut Machine, lpFrequency: u32) -> bool {
    // 64-bit write
    machine
        .mem()
        .put::<u32>(lpFrequency, QUERY_PERFORMANCE_FREQ);
    machine.mem().put::<u32>(lpFrequency + 4, 0);
    true
}

#[repr(C)]
#[derive(Debug)]
pub struct FILETIME {
    dwLowDateTime: DWORD,
    dwHighDateTime: DWORD,
}
unsafe impl ::memory::Pod for FILETIME {}
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
pub fn WriteConsoleA(
    _machine: &mut Machine,
    hConsoleOutput: HANDLE<()>,
    lpBuffer: ArrayWithSize<u8>,
    lpNumberOfCharsWritten: Option<&mut u32>,
    lpReserved: u32,
) -> bool {
    let msg = std::str::from_utf8(lpBuffer.unwrap()).unwrap();
    log::warn!("WriteConsoleA: {:?}", msg);
    if let Some(w) = lpNumberOfCharsWritten {
        *w = msg.len() as u32;
    }
    true // success
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
unsafe impl ::memory::Pod for SLIST_HEADER {}

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
    dwFlags: u32,
    lpMultiByteStr: u32,
    cbMultiByte: i32,
    mut lpWideCharStr: ArrayWithSizeMut<u16>,
) -> u32 {
    if CodePage != CP_ACP && CodePage != 1252 {
        unimplemented!("MultiByteToWideChar code page {CodePage}");
    }
    // TODO: dwFlags

    let input_len = match cbMultiByte {
        0 => return 0,                                                 // TODO: invalid param
        -1 => machine.mem().slicez(lpMultiByteStr).unwrap().len() + 1, // include nul
        len => len as u32,
    };

    match lpWideCharStr {
        Some(buf) if buf.len() == 0 => lpWideCharStr = None,
        _ => (),
    };

    match lpWideCharStr {
        None => input_len,
        Some(buf) => {
            let input = machine.mem().sub(lpMultiByteStr, input_len);
            let mut len = 0;
            for (&c_in, c_out) in std::iter::zip(input.as_slice_todo(), buf) {
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
    lpBuffer: ArrayWithSize<u16>,
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
pub fn WaitForSingleObject(
    _machine: &mut Machine,
    hHandle: HANDLE<()>,
    dwMilliseconds: u32,
) -> u32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn CreateEventA(
    _machine: &mut Machine,
    lpEventAttributes: u32,
    bManualReset: bool,
    bInitialState: bool,
    lpName: Option<&str>,
) -> HANDLE<()> {
    todo!()
}

#[win32_derive::dllexport]
pub fn SetEvent(_machine: &mut Machine, hEvent: HANDLE<()>) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn GetCurrentThread(_machine: &mut Machine) -> HANDLE<()> {
    todo!()
}

#[win32_derive::dllexport]
pub fn SetPriorityClass(
    _machine: &mut Machine,
    hProcess: HANDLE<()>,
    dwPriorityClass: u32,
) -> bool {
    true // success
}

#[win32_derive::dllexport]
pub fn Sleep(_machine: &mut Machine, dwMilliseconds: u32) -> u32 {
    log::warn!("TODO: sleep");
    0
}

#[win32_derive::dllexport]
pub fn AddVectoredExceptionHandler(_machine: &mut Machine, first: u32, handler: u32) -> u32 {
    0
}

#[repr(C)]
#[derive(Debug)]
pub struct INIT_ONCE {
    ptr: u32,
}
unsafe impl Pod for INIT_ONCE {}

#[win32_derive::dllexport]
pub fn InitOnceBeginInitialize(
    _machine: &mut Machine,
    lpInitOnce: Option<&mut INIT_ONCE>,
    dwFlags: u32,
    fPending: Option<&mut u32>,
    lpContext: u32,
) -> bool {
    if dwFlags != 0 {
        todo!();
    }
    *fPending.unwrap() = 1;
    true
}

#[win32_derive::dllexport]
pub fn InitOnceComplete(
    _machine: &mut Machine,
    lpInitOnce: Option<&mut INIT_ONCE>,
    dwFlags: u32,
    lpContext: u32,
) -> bool {
    if dwFlags != 0 {
        todo!();
    }
    lpInitOnce.unwrap().ptr = lpContext;
    true
}
