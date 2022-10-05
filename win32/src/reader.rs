use std::mem::size_of;

use anyhow::{anyhow, bail};

use crate::memory::{Memory, Pod};

pub struct Reader<'a> {
    pub buf: &'a [u8],
    pub pos: usize,
}

impl<'a> Reader<'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        Reader { buf, pos: 0 }
    }

    fn check_eof(&self) -> anyhow::Result<()> {
        if self.pos > self.buf.len() {
            bail!("EOF");
        }
        Ok(())
    }

    pub fn seek(&mut self, ofs: usize) -> anyhow::Result<()> {
        self.pos = ofs;
        self.check_eof()
    }

    pub fn skip(&mut self, n: usize) -> anyhow::Result<()> {
        self.pos += n;
        self.check_eof()
    }

    fn peek(&self, n: usize) -> Option<&'a [u8]> {
        self.buf.get(self.pos..self.pos + n)
    }

    pub fn expect(&mut self, s: &str) -> anyhow::Result<()> {
        let p = self.peek(s.len()).ok_or(anyhow!("EOF"))?;
        if p != s.as_bytes() {
            bail!("expected {:?}, got {:?}", s, p);
        }
        self.pos += s.len();
        Ok(())
    }

    pub fn u32(&mut self) -> anyhow::Result<u32> {
        let buf = self.peek(4).ok_or(anyhow!("EOF"))?;
        let val = (buf[0] as u32)
            | ((buf[1] as u32) << 8)
            | ((buf[2] as u32) << 16)
            | ((buf[3] as u32) << 24);
        self.pos += 4;
        Ok(val)
    }

    pub fn view<T: Pod>(&mut self) -> &'a T {
        let t = self.buf.view::<T>(self.pos as u32);
        self.pos += size_of::<T>();
        t
    }
}

pub fn read_strz(buf: &[u8]) -> String {
    let nul = buf.iter().position(|&c| c == 0).unwrap();
    String::from_utf8_lossy(&buf[0..nul]).to_string()
}
