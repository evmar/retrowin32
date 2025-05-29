//! Process initialization and startup.

use super::{file::HFILE, get_state, state::State2};
use crate::Machine; // TODO(Machine): break_on_startup, exit_thread, file
use crate::winapi::kernel32; // TODO: until we are in our own crate
use kernel32::file::{STDERR_HFILE, STDIN_HFILE, STDOUT_HFILE};
use memory::Extensions;
use win32_system::{System, memory::Memory};
use win32_winapi::{DWORD, WORD};

#[repr(C)]
pub struct UNICODE_STRING {
    pub Length: WORD,
    pub MaximumLength: WORD,
    pub Buffer: DWORD,
}
unsafe impl ::memory::Pod for UNICODE_STRING {}

#[repr(C)]
struct CURDIR {
    DosPath: UNICODE_STRING,
    Handle: DWORD,
}
unsafe impl ::memory::Pod for CURDIR {}

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

/// We stamp one of these into a process's memory to expose all the data structures
/// that the kernel32 APIs need to access.
#[repr(C)]
struct UserspaceData {
    peb: PEB,
    params: RTL_USER_PROCESS_PARAMETERS,
}
unsafe impl ::memory::Pod for UserspaceData {}

pub fn init_peb(state: &mut State2, memory: &Memory, cmdline: &str) {
    let user_data = memory.store(UserspaceData {
        peb: PEB {
            InheritedAddressSpace: 0,
            ReadImageFileExecOptions: 0,
            BeingDebugged: 0,
            SpareBool: 0,
            Mutant: 0,
            ImageBaseAddress: state.image_base,
            LdrData: 0,
            ProcessParameters: 0, // set below
            SubSystemData: 0,
            ProcessHeap: memory.process_heap.addr,
            TlsCount: 0, // will be set later
        },
        params: RTL_USER_PROCESS_PARAMETERS {
            AllocationSize: 0,
            Size: 0,
            Flags: 0,
            DebugFlags: 0,
            ConsoleHandle: 0,
            ConsoleFlags: 0,
            hStdInput: STDIN_HFILE,
            hStdOutput: STDOUT_HFILE,
            hStdError: STDERR_HFILE,
            CurrentDirectory: memory::Pod::zeroed(),
            DllPath: memory::Pod::zeroed(),
            ImagePathName: memory::Pod::zeroed(),
            CommandLine: state.cmdline.as_unicode_string(&cmdline, memory),
        },
    });
    state.peb = user_data;
    let peb = memory.mem().get_aligned_ref_mut::<PEB>(state.peb);
    peb.ProcessParameters = user_data;
}

/// Result of setting up the GDT, with initial values for all the relevant segment registers.
#[cfg(feature = "x86-unicorn")]
pub struct GDTEntries {
    /// Address of GDT itself.
    pub addr: u32,
    pub cs: u16,
    pub ds: u16,
    pub fs: u16,
    pub ss: u16,
}

#[cfg(feature = "x86-unicorn")]
pub fn create_gdt(&mut self, mem: Mem) -> GDTEntries {
    use segments::SegmentDescriptor;
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

pub fn peb_mut(sys: &dyn System) -> &mut PEB {
    let state = get_state(sys);
    sys.mem().get_aligned_ref_mut::<PEB>(state.peb)
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
        #[cfg(feature = "x86-emu")]
        {
            machine.emu.x86.cpu_mut().state = x86::CPUState::DebugBreak;
        }
        #[cfg(not(feature = "x86-emu"))]
        {
            todo!();
        }
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
