use crate::host;
use memory::Mem;

pub trait System {
    fn mem(&self) -> Mem;
    /// Escape hatch for users that haven't yet moved to this interface.
    fn machine(&mut self) -> *mut ();
    fn host(&mut self) -> &mut dyn host::Host;

    // TODO: I'd like this to just return a future, but because we make System a trait object
    // we use the workaround from https://github.com/dtolnay/async-trait.
    fn call_x86(
        &mut self,
        func: u32,
        args: Vec<u32>,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32> + '_>>;

    fn get_symbol(&mut self, dll: &str, name: &str) -> u32;
}
