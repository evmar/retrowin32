mod builtin_dlls;
mod machine;
mod segments;
pub mod shims;

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

#[cfg(feature = "x86-unicorn")]
mod gdt;

pub use builtin_ddraw as ddraw; // exposed so debugger can poke at internal state
pub use builtin_kernel32 as kernel32;
pub use kernel32::loader::Module;
pub use machine::{Machine, Status};
pub use win32_system::{host, trace};
pub use win32_winapi::{ERROR, RECT, UnixPath, WindowsPath, WindowsPathBuf};
