pub(crate) mod file;
pub(crate) mod file16;
pub(crate) mod find;
pub(crate) mod fs;
pub(crate) mod mapping;
pub(crate) mod metadata;
pub(crate) mod misc;
pub(crate) mod path;
pub(crate) mod stdio;

pub use file::{HFILE, write_file};
pub use metadata::FileAttribute;
pub use stdio::{STDERR_HFILE, STDIN_HFILE, STDOUT_HFILE};
use win32_system::{System, host};
use win32_winapi::Handles;

#[derive(Default)]
pub struct State {
    pub files: Handles<HFILE, Box<dyn host::File>>,
}

pub fn get_state(sys: &dyn System) -> ::std::cell::RefMut<State> {
    type SysState = ::std::cell::RefCell<State>;
    sys.state2(&::std::any::TypeId::of::<SysState>(), || {
        Box::new(::std::cell::RefCell::new(State::default()))
    })
    .downcast_ref::<SysState>()
    .unwrap()
    .borrow_mut()
}
