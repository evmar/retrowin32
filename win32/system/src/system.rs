use crate::{
    event::ArcEvent,
    host,
    wait::{Wait, WaitResult},
};
use memory::Mem;
use std::pin::Pin;
use win32_winapi::HANDLE;

/// The interface for the system beneath all the Windows DLLs, providing the lowest-level
/// functionality that the DLLs cannot implement themselves.  This functionality is still
/// implemented in the win32 crate, but the trait separation breaks the circular depedency
/// between the DLLs and the system.
///
/// - Machine manages loaded DLLs
/// - DLLs get passed a dyn System,
/// - System implemented by Machine
///
/// See also the Host interface, which is the interface to the outside world.
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

    fn new_thread(&mut self, stack_size: u32, start_addr: u32, args: &[u32]);

    // TODO: remove blocking API in favor of event objects.
    fn block(&mut self, wait: Option<u32>) -> Pin<Box<dyn Future<Output = ()> + '_>>;
    fn unblock(&mut self);

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

    fn get_symbol(&self, dll: &str, name: &str) -> u32;
    fn get_resources(&self, module: u32) -> Option<&[u8]>;
    fn get_thread_id(&self) -> u32;

    fn exit(&mut self, status: u32);

    /// Get a per-subcomponent state object.  The idea is each library (e.g. gdi32)
    /// can store its own state in the system, without this API needing to depend
    /// on the gdi32 library.
    fn state(&self, id: &std::any::TypeId) -> &dyn std::any::Any;
}
