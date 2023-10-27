mod mem;
mod pod;

#[cfg(feature = "x86-emu")]
mod memory_emu;
#[cfg(feature = "x86-64")]
mod memory_raw;
#[cfg(feature = "x86-unicorn")]
mod memory_unicorn;

pub use mem::Mem;
pub use pod::Pod;

#[cfg(feature = "x86-emu")]
pub type MemImpl = memory_emu::VecMem;

#[cfg(feature = "x86-64")]
pub type MemImpl = memory_raw::RawMem;

#[cfg(feature = "x86-unicorn")]
pub type MemImpl = memory_unicorn::MemImpl;
