pub mod loader;
mod machine;
mod segments;
pub mod shims;
pub mod winapi;

#[cfg(feature = "x86-emu")]
mod machine_emu;

#[cfg(feature = "x86-emu")]
pub use x86;

#[cfg(feature = "x86-64")]
mod ldt;
#[cfg(feature = "x86-64")]
mod machine_raw;
#[cfg(feature = "x86-64")]
mod shims_raw;

#[cfg(feature = "x86-unicorn")]
mod machine_unicorn;

pub use machine::{Machine, Status};
pub use win32_system::{System, host, trace};
pub use win32_winapi::{ERROR, RECT, UnixPath, WindowsPath, WindowsPathBuf, calling_convention};
