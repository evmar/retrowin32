use super::{peb_mut, teb_mut};
use crate::{
    machine::Machine,
    winapi,
    winapi::types::{Str16, HANDLE},
};
use memory::Pod;

const TRACE_CONTEXT: &'static str = "kernel32/thread";

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct HTHREADT;
pub type HTHREAD = HANDLE<HTHREADT>;

#[win32_derive::dllexport]
pub fn GetCurrentThread(machine: &mut Machine) -> HTHREAD {
    HTHREAD::from_raw(GetCurrentThreadId(machine))
}

#[win32_derive::dllexport]
pub fn GetCurrentThreadId(machine: &mut Machine) -> u32 {
    #[cfg(feature = "x86-emu")]
    {
        machine.emu.x86.cur_cpu as u32
    }

    #[cfg(not(feature = "x86-emu"))]
    {
        _ = machine;
        1
    }
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
        let id = 1; // TODO
        let stack_pointer = machine.create_stack(format!("thread{id} stack"), dwStackSize);
        let cpu = machine.emu.x86.new_cpu();
        cpu.regs.set32(x86::Register::ESP, stack_pointer);
        cpu.regs.set32(x86::Register::EBP, stack_pointer);
        let mem = machine.emu.memory.mem();
        x86::ops::push(cpu, mem, lpParameter);
        x86::ops::push(cpu, mem, lpStartAddress);
        x86::ops::push(cpu, mem, 0);
        cpu.regs.eip = retrowin32_thread_main;

        HTHREAD::from_raw(id)
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
