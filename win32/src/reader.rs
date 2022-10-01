use anyhow::{anyhow, bail};

pub struct Reader<'a> {
    buf: &'a [u8],
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

    pub fn u8(&mut self) -> anyhow::Result<u8> {
        let val = self.buf[self.pos];
        self.pos += 1;
        Ok(val)
    }

    pub fn u16(&mut self) -> anyhow::Result<u16> {
        let buf = self.peek(4).ok_or(anyhow!("EOF"))?;
        let val = (buf[0] as u16) | ((buf[1] as u16) << 8);
        self.pos += 2;
        Ok(val)
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

    pub fn str(&mut self, n: usize) -> anyhow::Result<String> {
        let mut buf = self.peek(n).ok_or(anyhow!("EOF"))?;
        self.pos += n;
        if let Some(nul) = buf.iter().position(|&c| c == 0) {
            buf = &buf[0..nul];
        }
        Ok(String::from_utf8_lossy(buf).to_string())
    }
}
