use std::mem::size_of;

use anyhow::bail;
use memory::Extensions;

pub struct Reader<'m> {
    pub buf: &'m [u8],
    pub pos: usize,
}

impl<'m> Reader<'m> {
    pub fn new(buf: &'m [u8]) -> Self {
        Reader { buf, pos: 0 }
    }

    fn check_eof(&self) -> anyhow::Result<()> {
        if self.pos > self.buf.len() {
            bail!("EOF");
        }
        Ok(())
    }

    pub fn seek(&mut self, ofs: u32) -> anyhow::Result<()> {
        self.pos = ofs as usize;
        self.check_eof()
    }

    pub fn skip(&mut self, n: usize) -> anyhow::Result<()> {
        self.pos += n;
        self.check_eof()
    }

    pub fn expect(&mut self, s: &str) -> anyhow::Result<()> {
        let got = &self.buf[self.pos..][..s.len()];
        if got != s.as_bytes() {
            bail!("expected {:?}, got {:?}", s, got);
        }
        self.pos += s.len();
        Ok(())
    }

    pub fn read<T: memory::Pod + Clone>(&mut self) -> T {
        let t = self.buf.get_pod::<T>(self.pos as u32);
        self.pos += size_of::<T>();
        t
    }

    pub fn read_n<T: memory::Pod + Clone>(&mut self, count: u32) -> anyhow::Result<Box<[T]>> {
        let len = size_of::<T>() * count as usize;
        if self.pos + len > self.buf.len() {
            bail!("EOF");
        }
        let values = self
            .buf
            .iter_pod::<T>(self.pos as u32, count)
            .collect::<Vec<_>>()
            .into_boxed_slice();
        self.pos += len;
        Ok(values)
    }
}
