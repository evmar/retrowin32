//! Windows string encoding.
//!
//! Internally we use Rust String, UTF-8.
//! When passing to Windows we convert to either ANSI or wide, depending on the API.
//! Because we are passing to Windows, we cannot assume the output buffer is aligned,
//! which means we always write to &mut [u8].

use memory::{ExtensionsMut, Mem};

/// Convert a string to a buffer as ANSI.
/// Returns number of bytes it would write; silently drops bytes that don't fit.
fn convert_ansi(buf: &mut [u8], str: &str) -> usize {
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
    ofs
}

/// Convert a string to a buffer as a wide string.
/// Returns number of bytes it would write; silently drops bytes that don't fit.
fn convert_wide(buf: &mut [u8], str: &str) -> usize {
    let cap = buf.len();
    let mut ofs = 0;
    for c in str.chars() {
        if c > 0x7f as char {
            todo!("non-ascii");
        }
        if ofs + 2 <= cap {
            buf[ofs..][..2].copy_from_slice(&(c as u16).to_le_bytes());
        }
        ofs += 2;
    }
    ofs
}

/// Encoder and its impls let code be generic over the output buffer type.
pub trait Encoder<'a> {
    /// Get the size of the buffer, in characters (not the number of bytes written).
    fn capacity(&self) -> u32;
    fn write(&mut self, str: &str);
    fn write_nul(&mut self, str: &str) {
        self.write(str);
        self.write("\0");
    }
    /// Returns the number of code units written, or Err(n) if not enough space.
    fn status(&self) -> Result<u32, u32>;
}

pub struct EncoderAnsi<'a> {
    // TODO: codepage
    buf: &'a mut [u8],
    ofs: usize,
}

impl<'a> EncoderAnsi<'a> {
    pub fn new(buf: &'a mut [u8]) -> Self {
        Self { buf, ofs: 0 }
    }
    pub fn from_mem(mem: Mem<'a>, ofs: u32, count: u32) -> Self {
        Self::new(mem.sub32_mut(ofs, count))
    }
}

impl<'a> Encoder<'a> for EncoderAnsi<'a> {
    fn capacity(&self) -> u32 {
        self.buf.len() as u32
    }
    fn write(&mut self, str: &str) {
        let buf = if self.ofs < self.buf.len() {
            &mut self.buf[self.ofs..]
        } else {
            &mut []
        };
        self.ofs += convert_ansi(buf, str);
    }
    fn status(&self) -> Result<u32, u32> {
        if self.ofs <= self.buf.len() {
            Ok(self.ofs as u32)
        } else {
            Err(self.ofs as u32)
        }
    }
}

pub struct EncoderWide<'a> {
    buf: &'a mut [u8],
    ofs: usize,
}

impl<'a> EncoderWide<'a> {
    pub fn new(buf: &'a mut [u8]) -> Self {
        Self { buf, ofs: 0 }
    }
    pub fn from_mem(mem: Mem<'a>, ofs: u32, count: u32) -> Self {
        Self::new(mem.sub32_mut(ofs, count * 2))
    }
}

impl<'a> Encoder<'a> for EncoderWide<'a> {
    fn capacity(&self) -> u32 {
        self.buf.len() as u32 / 2
    }
    fn write(&mut self, str: &str) {
        let buf = if self.ofs < self.buf.len() {
            &mut self.buf[self.ofs..]
        } else {
            &mut []
        };
        self.ofs += convert_wide(buf, str);
    }
    fn status(&self) -> Result<u32, u32> {
        if self.ofs <= self.buf.len() {
            Ok(self.ofs as u32 / 2)
        } else {
            Err(self.ofs as u32 / 2)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn measure() {
        let mut enc = EncoderAnsi::new(&mut []);
        enc.write("he");
        enc.write("llo");
        assert_eq!(enc.status(), Err(5));

        let mut enc = EncoderWide::new(&mut []);
        enc.write("he");
        enc.write("llo");
        assert_eq!(enc.status(), Err(5));
    }

    #[test]
    fn fits() {
        let mut buf: [u8; 5] = [0; 5];
        let mut enc = EncoderAnsi::new(&mut buf[..]);
        assert_eq!(enc.capacity(), 5);
        enc.write("he");
        enc.write("llo");
        assert_eq!(enc.status(), Ok(5));
        assert_eq!(&buf[..], b"hello");

        let mut buf: [u8; 10] = [0; 10];
        let mut enc = EncoderWide::new(&mut buf[..]);
        assert_eq!(enc.capacity(), 5);
        enc.write("he");
        enc.write("llo");
        assert_eq!(enc.status(), Ok(5));
        assert_eq!(&buf[..], b"h\0e\0l\0l\0o\0");
    }
}
