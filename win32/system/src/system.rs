use crate::{
    dll::{DLLResolution, Shim},
    event::ArcEvent,
    host,
    wait::{Wait, WaitResult},
};
use memory::Mem;
use std::pin::Pin;
use win32_winapi::{ERROR, HANDLE, HMODULE};

/// The interface for the system beneath all the Windows DLLs, providing the lowest-level
/// functionality that the DLLs cannot implement themselves.  See discussion in win32/README.md.
// TODO: this interface is a grab bag of goop, mostly whatever was needed to get the
// existing DLLs migrated to it, without much attention paid to the right API.
// TODO: I'd like to make the various Future-returning APIs to just return a future, but because we
// make System a trait object we use the workaround from https://github.com/dtolnay/async-trait.
pub trait System {
    fn mem(&self) -> Mem;
    fn memory(&self) -> &crate::memory::Memory;
    // TODO: this is ugly, and only needed in a few places, revisit.
    fn memory_mut(&mut self) -> &mut crate::memory::Memory;

    fn host(&self) -> &dyn host::Host;

    fn call_x86(&mut self, func: u32, args: Vec<u32>) -> Pin<Box<dyn Future<Output = u32> + '_>>;

    // Thread control functions.
    fn new_thread(&mut self, new_cpu: bool, stack_size: u32, start_addr: u32, args: &[u32]) -> u32;
    fn exit_thread(&mut self, status: u32);
    fn teb_addr(&self) -> u32;

    // Process control functions.
    fn debug_break(&mut self);
    fn exit(&mut self, status: u32);

    // TODO: remove block/unblock API in favor of event objects.
    fn block(&mut self, wait: Option<u32>) -> Pin<Box<dyn Future<Output = ()> + '_>>;
    fn unblock(&mut self);

    // TODO: remove in favor of an Event?
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

    // This is kernel32 API, but it's used so often we put it here so that
    // every other library doesn't need to depend on kernel32.
    fn set_last_error(&self, err: ERROR);

    fn register_shims(&mut self, export_to_shim: &[(u32, &'static Shim)]);

    /// Given an imported DLL name, find the DLL file we'll load for it.
    /// Handles normalizing the name, aliases, and builtins.
    fn resolve_dll(&self, filename: &str) -> DLLResolution;

    /// Look up a symbol from a DLL; DLL must have already been loaded.
    /// (This is a convenience around kernel32 API because it's needed by most DLLs.)
    fn get_symbol(&self, dll: &str, name: &str) -> u32;

    // These two functions are kernel32 API used by user32;
    // maybe we should just have the dependency?
    /// Get the resources section of a given module handle.
    fn get_resources(&self, module: HMODULE) -> Option<&[u8]>;
    fn get_thread_id(&self) -> u32;

    /// Get an arbitrary state object.  The idea is each library (e.g. gdi32)
    /// can store its own state in the system, without this API needing to depend
    /// on the gdi32 library.
    fn state(
        &self,
        id: &std::any::TypeId,
        init: fn() -> Box<dyn std::any::Any>,
    ) -> &dyn std::any::Any;
}

pub fn generic_get_state<T>(sys: &dyn System) -> std::cell::RefMut<T>
where
    T: 'static + Default,
{
    sys.state(&std::any::TypeId::of::<T>(), || {
        Box::new(std::cell::RefCell::new(T::default()))
    })
    .downcast_ref::<std::cell::RefCell<T>>()
    .unwrap()
    .borrow_mut()
}
