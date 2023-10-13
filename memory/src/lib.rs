mod mem;
mod pod;

#[cfg(feature = "x86-emu")]
mod memory_emu;
#[cfg(feature = "x86-64")]
mod memory_raw;

pub use mem::Mem;
pub use pod::Pod;

#[cfg(feature = "x86-emu")]
pub type MemImpl = memory_emu::VecMem;

#[cfg(feature = "x86-64")]
pub type MemImpl = memory_raw::RawMem;
