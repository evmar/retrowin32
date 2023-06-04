use std::mem::size_of;

use anyhow::bail;
use x86::Mem;

pub struct Reader<'a, 'm> {
    pub buf: &'a Mem<'m>,
    pub pos: u32,
}

impl<'a, 'm> Reader<'a, 'm> {
    pub fn new(buf: &'a Mem<'m>) -> Self {
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

    pub fn seek(&mut self, ofs: u32) -> anyhow::Result<()> {
        self.pos = ofs;
        self.check_eof()
    }

    pub fn skip(&mut self, n: u32) -> anyhow::Result<()> {
        self.pos += n;
        self.check_eof()
    }

    pub fn expect(&mut self, s: &str) -> anyhow::Result<()> {
        let got = self.read_n::<u8>(s.len() as u32)?;
        if got != s.as_bytes() {
            bail!("expected {:?}, got {:?}", s, got);
        }
        Ok(())
    }

    // TODO: Result<> here.
    pub fn read<T: x86::Pod>(&mut self) -> &'m T {
        let t = self.buf.view::<T>(self.pos as u32);
        self.pos += size_of::<T>() as u32;
        t
    }

    pub fn read_n<T: x86::Pod>(&mut self, count: u32) -> anyhow::Result<&'m [T]> {
        let len = size_of::<T>() as u32 * count;
        if self.pos + len > self.buf.len() {
            bail!("EOF");
        }
        let slice = self.buf.view_n::<T>(self.pos as u32, count as u32);
        self.pos += len;
        Ok(slice)
    }
}
