//! Process initialization and startup.

use super::file::{STDERR_HFILE, STDIN_HFILE, STDOUT_HFILE};
use super::{State, command_line, file::HFILE, get_state};
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

pub fn init_peb(state: &mut State, memory: &Memory, mut cmdline: command_line::State) {
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
            CommandLine: cmdline.as_unicode_string(memory),
        },
    });
    state.peb = user_data;
    state.cmdline = cmdline;
    let peb = memory.mem().get_aligned_ref_mut::<PEB>(state.peb);
    peb.ProcessParameters = user_data;
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
pub async fn retrowin32_main(sys: &mut dyn System, entry_point: u32) {
    if get_state(sys).break_on_startup {
        sys.debug_break();
    }
    sys.call_x86(entry_point, vec![]).await;
    // TODO: if the entry point returns, the Windows behavior is to wait for any
    // spawned threads before exiting.
    sys.exit(0);
}

#[win32_derive::dllexport]
pub async fn retrowin32_thread_main(sys: &mut dyn System, entry_point: u32, param: u32) {
    let status = sys.call_x86(entry_point, vec![param]).await;
    sys.exit_thread(status);
}
