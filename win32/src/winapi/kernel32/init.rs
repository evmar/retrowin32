//! Process initialization and startup.

use super::{
    EventObject, FindHandle, Mappings, ResourceHandle, Thread, DLL, HMODULE, STDERR_HFILE,
    STDOUT_HFILE,
};
use crate::{
    machine::MemImpl,
    pe,
    segments::SegmentDescriptor,
    winapi::{alloc::Arena, handle::Handles, heap::Heap, types::*},
    Machine,
};
use ::memory::Mem;
use memory::{Extensions, ExtensionsMut};
use std::collections::HashMap;

/// Process command line, as exposed in GetCommandLine() and also TEB.
/// Gross: GetCommandLineA() needs to return a pointer that's never freed,
/// so we need to hang on to both versions of the command line.
pub struct CommandLine {
    /// Command line, split args.
    pub args: Vec<String>,
    /// Command line, ASCII.
    pub cmdline: u32,
    /// Command line, UTF16.
    pub cmdline16: u32,
    /// Length without trailing nul.
    pub len: usize,
}

impl CommandLine {
    fn new(mut cmdline: String, arena: &mut Arena, mem: Mem) -> Self {
        let args = split_cmdline(&cmdline);

        log::debug!("CommandLine: {}", cmdline);
        let len = cmdline.len();
        cmdline.push(0 as char); // nul terminator

        let cmdline8_ptr = arena.alloc(cmdline.len() as u32, 1);
        mem.sub32_mut(cmdline8_ptr, cmdline.len() as u32)
            .copy_from_slice(cmdline.as_bytes());

        let cmdline16 = String16::from(&cmdline);
        let cmdline16_ptr = arena.alloc(cmdline16.byte_size() as u32, 2);
        let mem16: &mut [u16] =
            unsafe { std::mem::transmute(mem.sub32_mut(cmdline16_ptr, cmdline16.0.len() as u32)) };
        mem16.copy_from_slice(&cmdline16.0);

        CommandLine {
            args,
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

// TODO: follow the logic for CommandLineToArgvW
// https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-commandlinetoargvw
fn split_cmdline(cmdline: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut arg = String::new();
    let mut in_quote = false;
    for c in cmdline.chars() {
        match c {
            ' ' if !in_quote => {
                if !arg.is_empty() {
                    args.push(arg);
                    arg = String::new();
                }
            }
            '"' => {
                in_quote = !in_quote;
            }
            _ => {
                arg.push(c);
            }
        }
    }
    if !arg.is_empty() {
        args.push(arg);
    }
    args
}

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
    let params = mem.get_aligned_ref_mut::<RTL_USER_PROCESS_PARAMETERS>(params_addr);
    // x86.put::<u32>(params_addr + 0x10, console_handle);
    // x86.put::<u32>(params_addr + 0x14, console_flags);
    // x86.put::<u32>(params_addr + 0x18, stdin);
    params.hStdOutput = STDOUT_HFILE;
    params.hStdError = STDERR_HFILE;
    params.ImagePathName.clear();
    params.CommandLine = cmdline.as_unicode_string();

    // PEB
    let peb_addr = arena.alloc(std::cmp::max(std::mem::size_of::<PEB>() as u32, 0x100), 4);
    let peb = mem.get_aligned_ref_mut::<PEB>(peb_addr);
    peb.ProcessParameters = params_addr;
    peb.ProcessHeap = 0; // TODO: we use state.process_heap instead
    peb.TlsCount = 0;

    // SEH chain
    let seh_addr = arena.alloc(
        std::mem::size_of::<_EXCEPTION_REGISTRATION_RECORD>() as u32,
        4,
    );
    let seh = mem.get_aligned_ref_mut::<_EXCEPTION_REGISTRATION_RECORD>(seh_addr);
    seh.Prev = 0xFFFF_FFFF;
    seh.Handler = 0xFF5E_5EFF; // Hopefully easier to spot.

    // TEB
    let teb_addr = arena.alloc(std::cmp::max(std::mem::size_of::<TEB>() as u32, 0x100), 4);
    let teb = mem.get_aligned_ref_mut::<TEB>(teb_addr);
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

/// Objects identified by kernel handles, all of which can be passed to Wait* functions.
pub enum KernelObject {
    Event(EventObject),
    Thread(Thread),
}

type KernelObjects = Handles<HANDLE<()>, KernelObject>;
impl KernelObjects {
    pub fn get_event(&self, handle: HEVENT) -> Option<&EventObject> {
        match self.get_raw(handle.to_raw()) {
            Some(KernelObject::Event(ev)) => Some(ev),
            _ => None,
        }
    }
}

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
    pub process_heap: u32,

    pub dlls: HashMap<HMODULE, DLL>,

    pub resources: pe::IMAGE_DATA_DIRECTORY,
    pub resource_handles: Handles<HRSRC, ResourceHandle>,

    // There is a collection of handle types that are all from the same key space,
    // because they can be passed to the various Wait functions.
    pub objects: Handles<HANDLE<()>, KernelObject>,

    pub files: Handles<HFILE, Box<dyn crate::host::File>>,

    pub find_handles: Handles<HFIND, FindHandle>,

    pub(super) env: u32,

    pub cmdline: CommandLine,
}

impl State {
    pub fn new(mem: &mut MemImpl, cmdline: String, retrowin32_syscall: &[u8]) -> Self {
        let mut mappings = Mappings::new();
        let mapping = mappings.alloc(0x1000, "kernel32 data".into(), mem);
        let mut arena = Arena::new(mapping.addr, mapping.size);

        let mut dlls = HashMap::new();
        let dll = {
            let addr = arena.alloc(retrowin32_syscall.len() as u32, 8);
            mem.mem()
                .sub32_mut(addr, retrowin32_syscall.len() as u32)
                .copy_from_slice(retrowin32_syscall);
            let mut names = HashMap::new();
            names.insert("retrowin32_syscall".into(), addr);
            DLL {
                name: "retrowin32.dll".into(),
                dll: pe::DLL {
                    base: 0, // unused
                    names,
                    ordinal_base: 0,         // unused
                    fns: Default::default(), // unused
                    resources: None,
                    entry_point: None,
                },
            }
        };
        dlls.insert(HMODULE::from_raw(dll.dll.base), dll);

        let env = b"WINDIR=C:\\Windows\0\0";
        let env_addr = arena.alloc(env.len() as u32, 1);
        mem.mem()
            .sub32_mut(env_addr, env.len() as u32)
            .copy_from_slice(env);

        let cmdline = CommandLine::new(cmdline, &mut arena, mem.mem());

        let teb = init_teb(&cmdline, &mut arena, mem.mem());

        State {
            arena,
            image_base: 0,
            teb,
            process_heap: 0,
            mappings,
            heaps: HashMap::new(),
            dlls,
            objects: Default::default(),
            files: Default::default(),
            find_handles: Default::default(),
            env: env_addr,
            cmdline,
            resources: Default::default(),
            resource_handles: Default::default(),
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

    pub fn get_process_heap<'a>(&'a mut self, memory: &mut MemImpl) -> &mut Heap {
        if self.process_heap == 0 {
            let size = 24 << 20;
            let heap = self.new_heap(memory, size, "process heap".into());
            self.process_heap = heap;
        }
        self.get_heap(self.process_heap).unwrap()
    }

    pub fn create_gdt(&mut self, mem: Mem) -> GDTEntries {
        const COUNT: usize = 5;
        let addr = self.arena.alloc(COUNT as u32 * 8, 8);
        let gdt: &mut [u64; COUNT] = unsafe { &mut *(mem.get_ptr_mut::<u64>(addr) as *mut _) };

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

pub fn teb(machine: &Machine) -> &TEB {
    machine
        .mem()
        .get_aligned_ref::<TEB>(machine.state.kernel32.teb)
}
pub fn teb_mut(machine: &mut Machine) -> &mut TEB {
    machine
        .mem()
        .get_aligned_ref_mut::<TEB>(machine.state.kernel32.teb)
}
pub fn peb_mut(machine: &mut Machine) -> &mut PEB {
    let peb_addr = teb(machine).Peb;
    machine.mem().get_aligned_ref_mut::<PEB>(peb_addr)
}

#[repr(C)]
pub struct PEB {
    pub InheritedAddressSpace: u8,
    pub ReadImageFileExecOptions: u8,
    pub BeingDebugged: u8,
    pub SpareBool: u8,
    pub Mutant: DWORD,
    pub ImageBaseAddress: DWORD,
    pub LdrData: DWORD,
    /* 0x10 */
    pub ProcessParameters: DWORD,
    pub SubSystemData: DWORD,
    pub ProcessHeap: DWORD,
    // TODO: more fields

    // This is at the wrong offset, but it shouldn't matter.
    // TODO: this should be TlsBitmap.
    pub TlsCount: DWORD,
}
unsafe impl ::memory::Pod for PEB {}

#[repr(C)]
pub struct NT_TIB {
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
pub struct TEB {
    pub Tib: NT_TIB,
    pub EnvironmentPointer: DWORD,
    pub ClientId_UniqueProcess: DWORD,
    pub ClientId_UniqueThread: DWORD,
    pub ActiveRpcHandle: DWORD,
    pub ThreadLocalStoragePointer: DWORD,
    pub Peb: DWORD,
    pub LastErrorValue: DWORD,
    pub CountOfOwnedCriticalSections: DWORD,
    pub CsrClientThread: DWORD,
    pub Win32ThreadInfo: DWORD,
    pub User32Reserved: [DWORD; 26],
    pub UserReserved: [DWORD; 5],
    pub WOW32Reserved: DWORD,
    pub CurrentLocale: DWORD,
    // TODO: ... there are many more fields here

    // This is at the wrong offset, but it shouldn't matter.
    pub TlsSlots: [DWORD; 64],
}
unsafe impl ::memory::Pod for TEB {}

#[win32_derive::dllexport]
pub fn GetCommandLineA(machine: &mut Machine) -> u32 {
    machine.state.kernel32.cmdline.cmdline
}

#[win32_derive::dllexport]
pub fn GetCommandLineW(machine: &mut Machine) -> u32 {
    machine.state.kernel32.cmdline.cmdline16
}

/// This function is not part of the Windows API, but is rather just the entry
/// point for when retrowin32 starts/stops a process, initializing DLLs and calling main.
/// It probably has some better name within ntdll.dll.
#[win32_derive::dllexport]
pub async fn retrowin32_main(machine: &mut Machine, entry_point: u32) {
    struct DllData {
        base: u32,
        entry_point: u32,
    }

    let dlls = machine
        .state
        .kernel32
        .dlls
        .iter()
        .filter(|(_, dll)| dll.dll.entry_point.is_some())
        .map(|(_, dll)| DllData {
            base: dll.dll.base,
            entry_point: dll.dll.entry_point.unwrap(),
        })
        .collect::<Vec<_>>();
    // TODO: invoking dllmains can load more dlls.
    for dll in dlls {
        let hInstance = dll.base;
        let fdwReason = 1u32; // DLL_PROCESS_ATTACH
        let lpvReserved = 0u32;
        machine
            .call_x86(dll.entry_point, vec![hInstance, fdwReason, lpvReserved])
            .await;
    }

    machine.call_x86(entry_point, vec![]).await;
    // TODO: if the entry point returns, the Windows behavior is to wait for any
    // spawned threads before exiting.
    machine.exit(0);
}

#[win32_derive::dllexport]
pub async fn retrowin32_thread_main(machine: &mut Machine, entry_point: u32, param: u32) {
    machine.call_x86(entry_point, vec![param]).await;
    machine.emu.x86.cpu_mut().state = x86::CPUState::Free;
}
