mod exports;
mod file;
mod imports;
mod reader;
mod relocations;
mod resources;

pub use exports::*;
pub use file::*;
pub use imports::*;
pub use relocations::*;
pub use resources::*;

/// Read a C-style nul terminated string from a buffer.
/// Various PE structures use these, sometimes with an optional nul.
pub(crate) fn c_str(buf: &[u8]) -> &[u8] {
    let len = buf.iter().position(|b| *b == 0).unwrap_or(buf.len());
    &buf[..len]
}
