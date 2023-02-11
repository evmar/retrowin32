use std::mem::size_of;

use anyhow::{anyhow, bail};
use x86::Memory;

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

    pub fn read<T: x86::Pod>(&mut self) -> &'a T {
        let t = self.buf.view::<T>(self.pos as u32);
        self.pos += size_of::<T>();
        t
    }
}
