//! Process initialization and startup.

use super::{
    FindHandle, HEVENT, HFILE, HFIND, HMODULE, ResourceHandle, STDERR_HFILE, STDOUT_HFILE, Thread,
    command_line::CommandLine,
};
use crate::{
    Machine,
    loader::{self, Module},
    segments::SegmentDescriptor,
    winapi::{arena::Arena, *},
};
use ::memory::Mem;
use memory::{Extensions, ExtensionsMut};
use std::{collections::HashMap, rc::Rc, sync::Arc};
use win32_system::{Event, host, memory::Memory};

#[repr(C)]
pub struct UNICODE_STRING {
    pub Length: WORD,
    pub MaximumLength: WORD,
    pub Buffer: DWORD,
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
    Event(Arc<Event>),
    Thread(Rc<Thread>),
}

impl Clone for KernelObject {
    fn clone(&self) -> Self {
        match self {
            KernelObject::Event(ev) => KernelObject::Event(ev.clone()),
            KernelObject::Thread(th) => KernelObject::Thread(th.clone()),
        }
    }
}

type KernelObjects = Handles<HANDLE<()>, KernelObject>;
pub trait KernelObjectsMethods {
    fn get_event(&self, handle: HEVENT) -> Option<&Event>;
}
impl KernelObjectsMethods for KernelObjects {
    fn get_event(&self, handle: HEVENT) -> Option<&Event> {
        match self.get_raw(handle.to_raw()) {
            Some(KernelObject::Event(ev)) => Some(ev),
            _ => None,
        }
    }
}

pub struct ImportedSymbol {
    pub module: String,
    pub name: Box<[u8]>,
    pub addr: u32,
}

pub struct State {
    /// Memory for kernel32 data structures.
    pub arena: Arena,
    /// Address image was loaded at.
    pub image_base: u32,
    /// Address of PEB (process information exposed to executable).
    pub peb: u32,

    /// Loaded PE modules: the exe and all DLLs.
    pub modules: HashMap<HMODULE, Module>,

    pub resource_handles: Handles<HRSRC, ResourceHandle>,

    // There is a collection of handle types that are all from the same key space,
    // because they can be passed to the various Wait functions.
    pub objects: Handles<HANDLE<()>, KernelObject>,

    pub files: Handles<HFILE, Box<dyn host::File>>,

    pub find_handles: Handles<HFIND, FindHandle>,

    pub env: Vec<(String, String)>,

    pub cmdline: CommandLine,

    /// If true, debug break when entering the exe entry point.
    pub break_on_startup: bool,
}

impl State {
    pub fn new(memory: &mut Memory, retrowin32_syscall: &[u8]) -> Self {
        let mapping = memory
            .mappings
            .alloc(memory.imp.mem(), 0x1000, "kernel32 data".into());
        let mut arena = Arena::new(mapping.addr, mapping.size);

        let mut dlls = HashMap::new();
        let module = {
            let addr = arena.alloc(retrowin32_syscall.len() as u32, 8);
            memory
                .mem()
                .sub32_mut(addr, retrowin32_syscall.len() as u32)
                .copy_from_slice(retrowin32_syscall);
            let mut names = HashMap::new();
            names.insert("retrowin32_syscall".into(), addr);
            let exports = loader::Exports {
                names,
                ..Default::default()
            };
            loader::Module {
                name: "retrowin32.dll".into(),
                // use a non-zero base address so it doesn't register as the null HMODULE
                image_base: 1,
                exports,
                ..Default::default() // rest of fields unused
            }
        };
        dlls.insert(HMODULE::from_raw(module.image_base), module);

        let env = vec![(String::from("WINDIR"), String::from("C:\\Windows"))];

        State {
            arena,
            image_base: 0,
            peb: 0,
            modules: dlls,
            objects: Default::default(),
            files: Default::default(),
            find_handles: Default::default(),
            env,
            cmdline: CommandLine::default(),
            resource_handles: Default::default(),
            break_on_startup: false,
        }
    }

    pub fn init_process(&mut self, mem: Mem, cmdline: CommandLine) {
        self.cmdline = cmdline;

        // RTL_USER_PROCESS_PARAMETERS
        let params_addr = self.arena.alloc(
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
        params.CommandLine = self.cmdline.as_unicode_string(&mut self.arena, mem);

        self.peb = self
            .arena
            .alloc(std::cmp::max(std::mem::size_of::<PEB>() as u32, 0x100), 4);
        let peb = mem.get_aligned_ref_mut::<PEB>(self.peb);
        peb.ProcessParameters = params_addr;
        peb.ProcessHeap = 0; // TODO: use state.process_heap_addr instead
        peb.TlsCount = 0;
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
            base: 0, // TODO: get teb into here
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

pub fn peb_mut(machine: &mut Machine) -> &mut PEB {
    machine
        .mem()
        .get_aligned_ref_mut::<PEB>(machine.state.kernel32.peb)
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

/// This function is not part of the Windows API, but is rather just the entry
/// point for when retrowin32 starts/stops a process, initializing DLLs and calling main.
/// It probably has some better name within ntdll.dll.
#[win32_derive::dllexport]
pub async fn retrowin32_main(machine: &mut Machine, entry_point: u32) {
    if machine.state.kernel32.break_on_startup {
        machine.emu.x86.cpu_mut().state = x86::CPUState::DebugBreak;
    }
    machine.call_x86(entry_point, vec![]).await;
    // TODO: if the entry point returns, the Windows behavior is to wait for any
    // spawned threads before exiting.
    machine.exit(0);
}

#[win32_derive::dllexport]
pub async fn retrowin32_thread_main(machine: &mut Machine, entry_point: u32, param: u32) {
    machine.call_x86(entry_point, vec![param]).await;
    machine.exit_thread();
}
