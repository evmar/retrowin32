mod mem;
mod pod;
pub mod str16;

pub use mem::{Extensions, ExtensionsMut, Iterator, Mem, check_aligned};
pub use pod::Pod;

#[cfg(feature = "mem-box")]
mod boxmem;
#[cfg(feature = "mem-box")]
pub use boxmem::BoxMem as MemImpl;

#[cfg(feature = "mem-raw")]
mod rawmem;
#[cfg(feature = "mem-raw")]
pub use rawmem::RawMem as MemImpl;
