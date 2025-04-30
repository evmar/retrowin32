use crate::Machine;
use memory::Mem;

pub trait System {
    fn mem(&self) -> Mem;
    fn machine(&mut self) -> &mut Machine;
    fn host(&mut self) -> &mut dyn crate::Host;
}
