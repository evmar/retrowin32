mod mem;
mod pod;

#[cfg(feature = "x86-emu")]
mod memory_emu;
#[cfg(not(feature = "x86-emu"))]
mod memory_raw;

pub use mem::Mem;
pub use pod::Pod;

#[cfg(feature = "x86-emu")]
pub type MemImpl = memory_emu::VecMem;

#[cfg(not(feature = "x86-emu"))]
pub type MemImpl = memory_raw::RawMem;
