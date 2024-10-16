use super::peb_mut;
use crate::{
    machine::Machine,
    winapi::{
        self,
        alloc::Arena,
        types::{Str16, HANDLE},
    },
};
use memory::{Extensions, Mem, Pod};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct HTHREADT;
/// state.objects[HTHREAD] maps to a Thread object.
pub type HTHREAD = HANDLE<HTHREADT>;

#[repr(C)]
struct _EXCEPTION_REGISTRATION_RECORD {
    Prev: u32,
    Handler: u32,
}
unsafe impl ::memory::Pod for _EXCEPTION_REGISTRATION_RECORD {}

#[repr(C)]
pub struct NT_TIB {
    ExceptionList: u32,
    StackBase: u32,
    StackLimit: u32,
    SubSystemTib: u32,
    FiberData: u32,
    ArbitraryUserPointer: u32,
    _Self: u32,
}
unsafe impl ::memory::Pod for NT_TIB {}

#[repr(C)]
pub struct TEB {
    pub Tib: NT_TIB,
    pub EnvironmentPointer: u32,
    pub ClientId_UniqueProcess: u32,
    pub ClientId_UniqueThread: u32,
    pub ActiveRpcHandle: u32,
    pub ThreadLocalStoragePointer: u32,
    pub Peb: u32,
    pub LastErrorValue: u32,
    pub CountOfOwnedCriticalSections: u32,
    pub CsrClientThread: u32,
    pub Win32ThreadInfo: u32,
    pub User32Reserved: [u32; 26],
    pub UserReserved: [u32; 5],
    pub WOW32Reserved: u32,
    pub CurrentLocale: u32,
    // TODO: ... there are many more fields here

    // This is at the wrong offset, but it shouldn't matter.
    pub TlsSlots: [u32; 64],
}
unsafe impl ::memory::Pod for TEB {}

pub struct Thread {
    pub index: usize,

    /// address of TEB
    pub teb: u32,
}

/// Set up TEB, PEB, and other process info.
/// The FS register points at the TEB (thread info), which points at the PEB (process info).
fn init_teb(peb_addr: u32, arena: &mut Arena, mem: Mem) -> u32 {
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
}

/// Information about a newly-created thread; info that persists after the thread is created
/// is kept in Thread.
pub struct NewThread {
    pub thread: Thread,
    /// initial esp
    pub stack_pointer: u32,
}

pub fn create_thread(machine: &mut Machine, stack_size: u32) -> NewThread {
    let index = machine.emu.x86.new_cpu();

    let stack = machine.state.kernel32.mappings.alloc(
        stack_size,
        format!("thread {index} stack"),
        &mut machine.emu.memory,
    );
    let stack_pointer = stack.addr + stack.size - 4;

    let mem = machine.emu.memory.mem();
    let teb = init_teb(
        machine.state.kernel32.peb,
        &mut machine.state.kernel32.arena,
        mem,
    );

    NewThread {
        thread: Thread { index, teb },
        stack_pointer,
    }
}

#[win32_derive::dllexport]
pub fn GetCurrentThread(machine: &mut Machine) -> HTHREAD {
    HTHREAD::from_raw(GetCurrentThreadId(machine))
}

#[win32_derive::dllexport]
pub fn GetCurrentThreadId(machine: &mut Machine) -> u32 {
    #[cfg(feature = "x86-emu")]
    {
        machine.emu.x86.cur_cpu as u32 + 1
    }

    #[cfg(not(feature = "x86-emu"))]
    {
        _ = machine;
        1
    }
}

#[win32_derive::dllexport]
pub fn NtCurrentTeb(machine: &mut Machine) -> u32 {
    machine.emu.x86.cpu().regs.fs_addr
}

pub fn teb_mut(machine: &mut Machine) -> &mut TEB {
    // TODO: read directly from local Thread, don't believe exe's fs address.
    let fs = machine.emu.x86.cpu().regs.fs_addr;
    machine.emu.memory.mem().get_aligned_ref_mut::<TEB>(fs)
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

#[win32_derive::dllexport]
pub async fn CreateThread(
    machine: &mut Machine,
    lpThreadAttributes: u32,
    dwStackSize: u32,
    lpStartAddress: u32,
    lpParameter: u32,
    dwCreationFlags: u32,
    lpThreadId: u32,
) -> HTHREAD {
    let retrowin32_thread_main =
        winapi::kernel32::get_kernel32_builtin(machine, "retrowin32_thread_main");

    #[cfg(feature = "x86-emu")]
    {
        // TODO: should reuse a CPU from a previous thread that has exited
        let NewThread {
            thread,
            stack_pointer,
        } = create_thread(machine, dwStackSize);
        let cpu = &mut machine.emu.x86.cpus[thread.index];
        cpu.regs.set32(x86::Register::ESP, stack_pointer);
        cpu.regs.set32(x86::Register::EBP, stack_pointer);
        cpu.regs.fs_addr = thread.teb;
        let mem = machine.emu.memory.mem();
        x86::ops::push(cpu, mem, lpParameter);
        x86::ops::push(cpu, mem, lpStartAddress);
        x86::ops::push(cpu, mem, 0);
        cpu.regs.eip = retrowin32_thread_main;

        HTHREAD::from_raw(thread.index as u32 + 1)
    }

    #[cfg(not(feature = "x86-emu"))]
    {
        _ = retrowin32_thread_main;
        log::warn!("CreateThread running thread synchronously");
        machine.call_x86(lpStartAddress, vec![lpParameter]).await;
        HTHREAD::null()
    }
}

#[win32_derive::dllexport]
pub fn ExitThread(machine: &mut Machine, dwExitCode: u32) {
    if machine.emu.x86.cur_cpu == 0 {
        panic!("ExitThread called on main thread");
    }

    log::warn!(
        "thread on cpu {id} exiting with code {code}",
        code = dwExitCode,
        id = machine.emu.x86.cur_cpu
    );
    // TODO: free stack, other thread cleanup, set event to signal waiters, etc.

    machine.emu.x86.cpu_mut().state = x86::CPUState::Free;
}

#[win32_derive::dllexport]
pub fn ResumeThread(_machine: &mut Machine, hThread: HTHREAD) -> u32 {
    1
}

#[win32_derive::dllexport]
pub fn SetThreadDescription(
    _machine: &mut Machine,
    hThread: HTHREAD,
    lpThreadDescription: Option<&Str16>,
) -> bool {
    true // success
}

#[win32_derive::dllexport]
pub fn SetThreadPriority(_machine: &mut Machine, hThread: HTHREAD, nPriority: u32) -> bool {
    true // success
}

#[win32_derive::dllexport]
pub fn GetThreadPriority(_machine: &mut Machine, hThread: HTHREAD) -> i32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn SetThreadStackGuarantee(_machine: &mut Machine, StackSizeInBytes: Option<&mut u32>) -> bool {
    // ignore
    true // success
}

#[win32_derive::dllexport]
pub fn InterlockedIncrement(_machine: &mut Machine, addend: Option<&mut u32>) -> u32 {
    let addend = addend.unwrap();
    *addend += 1;
    *addend
}

#[win32_derive::dllexport]
pub fn InterlockedDecrement(_machine: &mut Machine, addend: Option<&mut u32>) -> u32 {
    todo!()
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

#[win32_derive::dllexport]
pub fn InitializeCriticalSection(_machine: &mut Machine, lpCriticalSection: u32) -> bool {
    true
}

#[win32_derive::dllexport]
pub fn InitializeCriticalSectionEx(
    _machine: &mut Machine,
    lpCriticalSection: u32,
    dwSpinCount: u32,
    flags: u32,
) -> bool {
    true
}

#[win32_derive::dllexport]
pub fn InitializeCriticalSectionAndSpinCount(
    _machine: &mut Machine,
    lpCriticalSection: u32,
    dwSpinCount: u32,
) -> bool {
    // "On single-processor systems, the spin count is ignored and the critical section spin count is set to 0 (zero)."
    // "This function always succeeds and returns a nonzero value."
    true
}

#[win32_derive::dllexport]
pub fn DeleteCriticalSection(_machine: &mut Machine, lpCriticalSection: u32) -> u32 {
    0
}

#[win32_derive::dllexport]
pub fn EnterCriticalSection(_machine: &mut Machine, lpCriticalSection: u32) -> u32 {
    0
}

#[win32_derive::dllexport]
pub fn LeaveCriticalSection(_machine: &mut Machine, lpCriticalSection: u32) -> u32 {
    0
}

#[repr(C)]
#[derive(Debug)]
pub struct SRWLOCK {
    ptr: u32,
}
unsafe impl Pod for SRWLOCK {}

#[win32_derive::dllexport]
pub fn AcquireSRWLockShared(_machine: &mut Machine, SRWLock: Option<&mut SRWLOCK>) -> u32 {
    0
}

#[win32_derive::dllexport]
pub fn ReleaseSRWLockShared(_machine: &mut Machine, SRWLock: Option<&mut SRWLOCK>) -> u32 {
    0
}

#[win32_derive::dllexport]
pub fn AcquireSRWLockExclusive(_machine: &mut Machine, SRWLock: Option<&mut SRWLOCK>) -> u32 {
    0
}

#[win32_derive::dllexport]
pub fn TryAcquireSRWLockExclusive(_machine: &mut Machine, SRWLock: Option<&mut SRWLOCK>) -> u32 {
    0
}

#[win32_derive::dllexport]
pub fn ReleaseSRWLockExclusive(_machine: &mut Machine, SRWLock: Option<&mut SRWLOCK>) -> u32 {
    0
}
