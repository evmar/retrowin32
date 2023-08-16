mod mem;
mod pod;

#[cfg(feature = "cpuemu")]
mod memory_cpuemu;
#[cfg(not(feature = "cpuemu"))]
mod memory_raw;

pub use mem::Mem;
pub use pod::Pod;

#[cfg(feature = "cpuemu")]
pub type MemImpl = memory_cpuemu::VecMem;

#[cfg(not(feature = "cpuemu"))]
pub type MemImpl = memory_raw::RawMem;
