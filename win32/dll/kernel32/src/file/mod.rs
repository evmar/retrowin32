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
use win32_system::{System, generic_get_state, host};
use win32_winapi::Handles;

#[derive(Default)]
pub struct State {
    pub files: Handles<HFILE, Box<dyn host::File>>,
}

#[inline]
pub fn get_state(sys: &dyn System) -> std::cell::RefMut<'_, State> {
    generic_get_state::<State>(sys)
}
