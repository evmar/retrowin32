use std::mem::size_of;

use anyhow::{anyhow, bail};
use x86::{Mem, Memory};

pub struct Reader<'a> {
    pub buf: &'a Mem,
    pub pos: usize,
}

impl<'a> Reader<'a> {
    pub fn new(buf: &'a Mem) -> Self {
        Reader { buf, pos: 0 }
    }

    pub fn done(&self) -> bool {
        self.pos == self.buf.len()
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
        self.buf.0.get(self.pos..self.pos + n)
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

    pub fn read_n<T: x86::Pod>(&mut self, count: usize) -> &'a [T] {
        let slice = self.buf.view_n::<T>(self.pos, count);
        self.pos += size_of::<T>() * count;
        slice
    }
}
