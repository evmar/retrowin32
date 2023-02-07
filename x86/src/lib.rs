pub mod debug;
mod memory;
pub mod ops;
mod registers;
mod x86;

pub use memory::{Memory, Pod};
pub use x86::{InstrCache, Snapshot, NULL_POINTER_REGION_SIZE, X86};

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
