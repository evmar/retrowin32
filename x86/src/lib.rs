pub mod debug;
mod memory;
pub mod ops;
mod registers;
mod x86;

use std::fmt::Display;

pub use memory::{Memory, Pod};
pub use x86::{InstrCache, Snapshot, NULL_POINTER_REGION_SIZE, X86};

#[derive(Debug)]
pub enum Error {
    Interrupt,
    Error(String),
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;
