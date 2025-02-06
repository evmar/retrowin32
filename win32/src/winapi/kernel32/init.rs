//! Process initialization and startup.

use super::{
    command_line::CommandLine, EventObject, FindHandle, Mappings, ResourceHandle, Thread, DLL,
    HEVENT, HMODULE, STDERR_HFILE, STDOUT_HFILE,
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
use std::{collections::HashMap, rc::Rc};

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
    Event(EventObject),
    Thread(Rc<Thread>),
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
    pub arena: Arena,
    /// Address image was loaded at.
    pub image_base: u32,
    /// Address of PEB (process information exposed to executable).
    pub peb: u32,
    pub mappings: Mappings,

    /// Heaps created by HeapAlloc().  Note: doesn't include the process heap.
    heaps: HashMap<u32, Heap>,

    /// The "process heap" is a per-process default heap exposed via GetProcessHeap and used
    /// by default.
    /// We also use it for our own random allocations, e.g. buffers allocated by other APIs.
    pub process_heap: Heap,

    /// Unlike other heaps, the process heap is not stored in the `heaps` map, so we stash
    /// its address separately, and need to remember to check it when doing operations
    /// over heaps in general.
    pub process_heap_addr: u32,

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
    pub fn new(mem: &mut MemImpl, retrowin32_syscall: &[u8]) -> Self {
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

        State {
            arena,
            image_base: 0,
            peb: 0,
            process_heap: Heap::default(),
            process_heap_addr: 0,
            mappings,
            heaps: HashMap::new(),
            dlls,
            objects: Default::default(),
            files: Default::default(),
            find_handles: Default::default(),
            env: env_addr,
            cmdline: CommandLine::default(),
            resources: Default::default(),
            resource_handles: Default::default(),
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
        if addr == self.process_heap_addr {
            return Some(&mut self.process_heap);
        }
        self.heaps.get_mut(&addr)
    }

    pub fn init_process_heap<'a>(&'a mut self, memory: &mut MemImpl) {
        assert!(self.process_heap_addr == 0);
        let size = 24 << 20;
        let addr = self.new_heap(memory, size, "process heap".into());
        self.process_heap = self.heaps.remove(&addr).unwrap();
        self.process_heap_addr = addr;
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
    struct DllData {
        base: u32,
        entry_point: u32,
    }

    let dlls = machine
        .state
        .kernel32
        .dlls
        .iter()
        .filter_map(|(_, dll)| {
            Some(DllData {
                base: dll.dll.base,
                entry_point: dll.dll.entry_point?,
            })
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
