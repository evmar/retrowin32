use crate::{Machine, host};
use memory::Mem;

pub trait System {
    fn mem(&self) -> Mem;
    fn machine(&mut self) -> &mut Machine;
    fn host(&mut self) -> &mut dyn host::Host;

    // TODO: I'd like this to just return a future, but because we make System a trait object
    // we use the workaround from https://github.com/dtolnay/async-trait.
    fn call_x86(
        &mut self,
        func: u32,
        args: Vec<u32>,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32> + '_>>;
}
