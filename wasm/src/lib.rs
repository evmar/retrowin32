use std::io::Result;
use std::io::Write;

pub struct Writer<'a> {
    w: &'a mut dyn Write,
}

impl<'a> Writer<'a> {
    pub fn new(w: &'a mut dyn Write) -> Writer<'a> {
        Writer { w }
    }

    pub fn write_byte(&mut self, b: u8) -> Result<()> {
        self.w.write_all(&[b])
    }

    pub fn write_uint(&mut self, n: impl Into<u32>) -> Result<()> {
        let mut n: u32 = n.into();
        while n >= 1 << 7 {
            self.write_byte((n as u8) | 0x80)?;
            n >>= 7;
        }
        self.write_byte(n as u8)
    }

    pub fn header(&mut self) -> Result<()> {
        self.w
            .write_all(&[0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00])
    }

    pub fn section(&mut self, id: u8, f: impl FnOnce(Writer) -> Result<()>) -> Result<()> {
        let mut buf = Vec::new();
        f(Writer::new(&mut buf))?;

        self.write_byte(id)?;
        self.write_uint(buf.len() as u32)?;
        self.w.write_all(&buf)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() -> Result<()> {
        let mut buf: Vec<u8> = Vec::new();
        let mut w = Writer::new(&mut buf);
        w.header()?;
        Ok(())
    }

    #[test]
    fn leb() -> Result<()> {
        let mut buf: Vec<u8> = Vec::new();
        let mut w = Writer::new(&mut buf);
        w.write_uint(624485u32)?;
        assert!(buf == &[0xE5, 0x8E, 0x26]);
        Ok(())
    }
}
