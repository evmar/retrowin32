//! Windows string serialization.
//!
//! Internally we use Rust String, UTF-8.
//! When passing to Windows we convert to either ANSI or wide, depending on the API.
//! Because we are passing to Windows, we cannot assume the output buffer is aligned,
//! which means we always write to &mut [u8].

use memory::{ExtensionsMut, Mem};

/// Write a nul-terminated string to a buffer as ANSI.
/// Returns number of chars if it fits, or Err(n) if it doesn't.
fn write_ansi(buf: &mut [u8], str: &str) -> Result<u32, u32> {
    let cap = buf.len();
    let mut ofs = 0;
    for c in str.chars() {
        if c > 0x7f as char {
            todo!("non-ascii");
        }
        if ofs < cap {
            buf[ofs] = c as u8;
        }
        ofs += 1;
    }
    if ofs < cap {
        buf[ofs] = 0;
    }
    ofs += 1;

    if ofs > buf.len() {
        return Err(ofs as u32);
    }
    Ok(ofs as u32)
}

/// Write a nul-terminated string to a buffer as wide.
/// Returns number of chars if it fits, or Err(n) if it doesn't.
fn write_wide(buf: &mut [u8], str: &str) -> Result<u32, u32> {
    let cap = buf.len();
    let mut ofs = 0;
    for c in str.chars() {
        if c > 0x7f as char {
            todo!("non-ascii");
        }
        if ofs + 1 < cap {
            buf[ofs..][..2].copy_from_slice(&(c as u16).to_le_bytes());
        }
        ofs += 2;
    }
    if ofs + 1 < cap {
        buf[ofs..][..2].copy_from_slice(&[0, 0]);
    }
    ofs += 2;

    if ofs > cap {
        return Err(ofs as u32 / 2);
    }
    Ok(ofs as u32 / 2)
}

/// BufWrite and its impls let code be generic over the output buffer type.
pub trait BufWrite<'a> {
    /// Get the size of the buffer, in characters (not the number of bytes written).
    fn capacity(&self) -> u32;
    /// Return the number of characters written.
    /// Err(n) is the number of characters needed when buffer is too small.
    fn write(&mut self, str: &str) -> Result<u32, u32>;
}

pub struct BufWriteAnsi<'a>(&'a mut [u8]); // TODO: codepage
impl<'a> BufWriteAnsi<'a> {
    pub fn new(buf: &'a mut [u8]) -> Self {
        Self(buf)
    }
    pub fn from_mem(mem: Mem<'a>, ofs: u32, count: u32) -> Self {
        Self(mem.sub32_mut(ofs, count))
    }
}
impl<'a> BufWrite<'a> for BufWriteAnsi<'a> {
    fn capacity(&self) -> u32 {
        self.0.len() as u32
    }
    fn write(&mut self, str: &str) -> Result<u32, u32> {
        write_ansi(self.0, str)
    }
}

pub struct BufWriteWide<'a>(&'a mut [u8]);
impl<'a> BufWriteWide<'a> {
    pub fn new(buf: &'a mut [u8]) -> Self {
        Self(buf)
    }
    pub fn from_mem(mem: Mem<'a>, ofs: u32, count: u32) -> Self {
        Self::new(mem.sub32_mut(ofs, count * 2))
    }
}
impl<'a> BufWrite<'a> for BufWriteWide<'a> {
    fn capacity(&self) -> u32 {
        self.0.len() as u32 / 2
    }
    fn write(&mut self, str: &str) -> Result<u32, u32> {
        write_wide(self.0, str)
    }
}
