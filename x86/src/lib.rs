pub mod debug;
mod memory;
pub mod ops;
mod registers;
mod x86;

pub use memory::{Mem, Memory, Pod, VecMem};
pub use x86::{InstrCache, CPU, NULL_POINTER_REGION_SIZE};

#[derive(Debug)]
pub enum StepError {
    Interrupt,
    Error(String),
}
impl std::fmt::Display for StepError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl std::error::Error for StepError {}

pub type StepResult<T> = std::result::Result<T, StepError>;
