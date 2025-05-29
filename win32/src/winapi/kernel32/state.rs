use super::{HEVENT, Thread, command_line, init::init_peb};
use std::{rc::Rc, sync::Arc};
use win32_system::{Event, System, memory::Memory};
use win32_winapi::{HANDLE, Handles};

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

#[derive(Default)]
pub struct State {
    /// If true, debug break when entering the exe entry point.
    pub break_on_startup: bool,
}

/// State held via the sys.get_state() interface.
// TODO: move all State to State2, rename to State.
#[derive(Default)]
pub struct State2 {
    /// Address image was loaded at.
    pub image_base: u32,

    /// Address of PEB (process information exposed to executable).
    pub peb: u32,

    /// State for command line APIs.
    /// The actual process command line is held in Machine, this is just to stash some pointers.
    pub(crate) cmdline: command_line::State,

    // There is a collection of handle types that are all from the same key space,
    // because they can be passed to the various Wait functions.
    pub objects: Handles<HANDLE<()>, KernelObject>,
}

impl State2 {
    pub fn init_process(&mut self, memory: &Memory, cmdline: &str) {
        init_peb(self, memory, cmdline);
    }
}

pub fn get_state(sys: &dyn System) -> std::cell::RefMut<State2> {
    type SysState = std::cell::RefCell<State2>;
    sys.state2(&std::any::TypeId::of::<SysState>(), || {
        Box::new(std::cell::RefCell::new(State2::default()))
    })
    .downcast_ref::<SysState>()
    .unwrap()
    .borrow_mut()
}
