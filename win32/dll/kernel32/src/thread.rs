use super::{KernelObject, get_state, peb_mut};
use memory::Extensions;
use std::{rc::Rc, sync::Arc};
use win32_system::{Event, System, memory::Memory};
use win32_winapi::{ERROR, HANDLE, Str16};

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
    pub padding: [u32; 20],

    // This is at the wrong offset, but it shouldn't matter.
    pub TlsSlots: [u32; 64],
}
unsafe impl ::memory::Pod for TEB {}

pub struct Thread {
    /// Entry in kernel32.objects.
    pub handle: HTHREAD,

    /// address of TEB
    pub teb: u32,

    pub terminated: Arc<Event>,
}

/// Set up TEB, PEB, and other process info.
/// The FS register points at the TEB (thread info), which points at the PEB (process info).
fn init_teb(peb_addr: u32, thread_id: u32, memory: &Memory) -> u32 {
    // SEH chain
    let seh_addr = memory.store(_EXCEPTION_REGISTRATION_RECORD {
        Prev: 0xFFFF_FFFF,
        Handler: 0xFF5E_5EFF, // Hopefully easier to spot.
    });

    // TEB
    let teb_addr = memory.store(TEB {
        Tib: NT_TIB {
            ExceptionList: seh_addr,
            // _Self: circular
            ..memory::Pod::zeroed()
        },
        Peb: peb_addr,
        ClientId_UniqueThread: thread_id,
        ..memory::Pod::zeroed()
    });

    let teb = memory.mem().get_aligned_ref_mut::<TEB>(teb_addr);
    teb.Tib._Self = teb_addr; // circular reference

    teb_addr
}

/// Information about a newly-created thread.
/// Info that persists after the thread is created is kept in Thread.
pub struct NewThread {
    pub thread: Rc<Thread>,
    /// initial esp
    pub stack_pointer: u32,
}

pub fn create_thread(sys: &mut dyn System, stack_size: u32) -> NewThread {
    // TODO: 2x get_state calls here is gross, need to figure out mappings mutability.
    let handle = get_state(sys).objects.reserve();
    let memory = sys.memory_mut();
    let stack = memory.mappings.alloc(
        memory.imp.mem(),
        stack_size,
        format!("thread {handle:x} stack", handle = handle.to_raw()),
    );
    let stack_pointer = stack.addr + stack.size - 4;

    let mut state = get_state(sys);
    let teb = init_teb(state.peb, handle.to_raw(), &sys.memory());

    let thread = Rc::new(Thread {
        handle: HTHREAD::from_raw(handle.to_raw()),
        teb,
        terminated: Event::new(None, false, false),
    });
    state
        .objects
        .set(handle, KernelObject::Thread(thread.clone()));

    NewThread {
        thread,
        stack_pointer,
    }
}

pub fn current_thread(sys: &dyn System) -> HTHREAD {
    HTHREAD::from_raw(teb(sys).ClientId_UniqueThread)
}

#[win32_derive::dllexport]
pub fn GetCurrentThread(sys: &dyn System) -> HTHREAD {
    current_thread(sys)
}

#[win32_derive::dllexport]
pub fn GetCurrentThreadId(sys: &dyn System) -> u32 {
    current_thread(sys).to_raw()
}

pub fn teb(sys: &dyn System) -> &TEB {
    sys.mem().get_aligned_ref::<TEB>(sys.teb_addr())
}

pub fn teb_mut(sys: &dyn System) -> &mut TEB {
    sys.mem().get_aligned_ref_mut::<TEB>(sys.teb_addr())
}

#[win32_derive::dllexport]
pub fn TlsAlloc(sys: &dyn System) -> u32 {
    let peb = peb_mut(sys);
    let slot = peb.TlsCount;
    peb.TlsCount = slot + 1;
    slot
}

#[win32_derive::dllexport]
pub fn TlsFree(sys: &dyn System, dwTlsIndex: u32) -> bool {
    let peb = peb_mut(sys);
    if dwTlsIndex >= peb.TlsCount {
        log::warn!("TlsFree of unknown slot {dwTlsIndex}");
        return false;
    }
    // TODO
    true
}

#[win32_derive::dllexport]
pub fn TlsSetValue(sys: &mut dyn System, dwTlsIndex: u32, lpTlsValue: u32) -> bool {
    let teb = teb_mut(sys);
    teb.TlsSlots[dwTlsIndex as usize] = lpTlsValue;
    true
}

#[win32_derive::dllexport]
pub fn TlsGetValue(sys: &mut dyn System, dwTlsIndex: u32) -> u32 {
    let teb = teb_mut(sys);
    if dwTlsIndex as usize >= teb.TlsSlots.len() {
        log::warn!("TlsGetValue of unknown slot {dwTlsIndex}");
        sys.set_last_error(ERROR::INVALID_PARAMETER);
        return 0;
    }
    teb.TlsSlots[dwTlsIndex as usize]
}

#[win32_derive::dllexport]
pub async fn CreateThread(
    sys: &mut dyn System,
    lpThreadAttributes: u32,
    dwStackSize: u32,
    lpStartAddress: u32,
    lpParameter: u32,
    dwCreationFlags: u32,
    lpThreadId: u32,
) -> HTHREAD {
    let retrowin32_thread_main = sys.get_symbol("kernel32.dll", "retrowin32_thread_main");

    let stack_size = if dwStackSize > 0 {
        dwStackSize
    } else {
        // TODO: in theory this is configured by exe header, but in practice probably doesn't matter.
        64 << 10
    };

    #[cfg(feature = "x86-emu")]
    {
        // TODO: should reuse a CPU from a previous thread that has exited
        let handle = sys.new_thread(
            true,
            stack_size,
            retrowin32_thread_main,
            &[
                lpStartAddress, // entry point
                lpParameter,    // parameter
            ],
        );
        HTHREAD::from_raw(handle)
    }

    #[cfg(not(feature = "x86-emu"))]
    {
        _ = retrowin32_thread_main;
        log::warn!("CreateThread running thread synchronously");
        sys.call_x86(lpStartAddress, vec![lpParameter]).await;
        HTHREAD::null()
    }
}

#[win32_derive::dllexport]
pub fn ExitThread(sys: &mut dyn System, dwExitCode: u32) {
    // TODO: free stack, other thread cleanup, set event to signal waiters, etc.
    sys.exit_thread(dwExitCode);
}

#[win32_derive::dllexport]
pub fn ResumeThread(sys: &dyn System, hThread: HTHREAD) -> u32 {
    1
}

#[win32_derive::dllexport]
pub fn TerminateThread(sys: &dyn System, hThread: HTHREAD, dwExitCode: u32) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn SetThreadDescription(
    sys: &dyn System,
    hThread: HTHREAD,
    lpThreadDescription: Option<&Str16>,
) -> bool {
    true // success
}

#[win32_derive::dllexport]
pub fn SetThreadPriority(sys: &dyn System, hThread: HTHREAD, nPriority: u32) -> bool {
    true // success
}

#[win32_derive::dllexport]
pub fn GetThreadPriority(sys: &dyn System, hThread: HTHREAD) -> i32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn SetThreadStackGuarantee(sys: &dyn System, StackSizeInBytes: Option<&mut u32>) -> bool {
    // ignore
    true // success
}
