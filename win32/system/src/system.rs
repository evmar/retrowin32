use crate::{
    dll::DLLResolution,
    event::ArcEvent,
    host,
    wait::{Wait, WaitResult},
};
use memory::Mem;
use std::pin::Pin;
use win32_winapi::{ERROR, HANDLE, HMODULE};

/// The interface for the system beneath all the Windows DLLs, providing the lowest-level
/// functionality that the DLLs cannot implement themselves.  See discussion in win32/README.md.
pub trait System {
    fn mem(&self) -> Mem;
    fn memory(&self) -> &crate::memory::Memory;
    fn memory_mut(&mut self) -> &mut crate::memory::Memory;

    /// Escape hatch for users that haven't yet moved to this interface.
    fn machine(&mut self) -> *mut ();
    fn host(&self) -> &dyn host::Host;

    // TODO: I'd like this to just return a future, but because we make System a trait object
    // we use the workaround from https://github.com/dtolnay/async-trait.
    fn call_x86(&mut self, func: u32, args: Vec<u32>) -> Pin<Box<dyn Future<Output = u32> + '_>>;

    fn new_thread(&mut self, new_cpu: bool, stack_size: u32, start_addr: u32, args: &[u32]) -> u32;

    // TODO: remove blocking API in favor of event objects.
    fn block(&mut self, wait: Option<u32>) -> Pin<Box<dyn Future<Output = ()> + '_>>;
    fn unblock(&mut self);

    fn sleep(&mut self, ms: u32) -> Pin<Box<dyn Future<Output = ()> + '_>>;

    fn wait_for_events(
        &mut self,
        events: &[ArcEvent],
        wait_all: bool,
        wait: Wait,
    ) -> Pin<Box<dyn Future<Output = WaitResult> + '_>>;
    // TODO: remove me
    fn wait_for_objects(
        &mut self,
        objects: &[HANDLE<()>],
        wait_all: bool,
        wait: Wait,
    ) -> Pin<Box<dyn Future<Output = WaitResult> + '_>>;

    fn set_last_error(&self, err: ERROR);

    /// Get an already-loaded library by name.
    fn get_library(&self, dll: &str) -> HMODULE;
    /// Load a library by name.
    fn load_library(&mut self, dll: &str) -> Pin<Box<dyn Future<Output = HMODULE> + '_>>;

    /// Look up a symbol from a DLL; DLL must have already been loaded.
    fn get_symbol(&self, dll: &str, name: &str) -> u32;

    /// Get the resources section of a given module handle.
    fn get_resources(&self, module: HMODULE) -> Option<&[u8]>;

    fn get_thread_id(&self) -> u32;

    fn exit(&mut self, status: u32);
    fn exit_thread(&mut self, status: u32);

    /// Get a per-subcomponent state object.  The idea is each library (e.g. gdi32)
    /// can store its own state in the system, without this API needing to depend
    /// on the gdi32 library.
    fn state(&self, id: &std::any::TypeId) -> &dyn std::any::Any;
    // TODO: migrate to this new API.
    fn state2(
        &self,
        id: &std::any::TypeId,
        init: fn() -> Box<dyn std::any::Any>,
    ) -> &dyn std::any::Any;

    // TODO: added in kernel32 migration, need a better place.
    fn teb_addr(&self) -> u32;
    fn debug_break(&mut self);

    fn resolve_dll(&self, filename: &str) -> DLLResolution;
}
