fn hex(b: u8) -> u8 {
    if b < 0x10 {
        b'0' + b
    } else {
        b'a' + (b - 0x10)
    }
}

pub struct Fmt {
    buf: [u8; 80],
    len: usize,
}

#[allow(dead_code)]
impl Fmt {
    pub fn new() -> Self {
        Fmt {
            buf: [0; 80],
            len: 0,
        }
    }

    pub fn char(&mut self, char: u8) -> &mut Self {
        self.buf[self.len] = char;
        self.len += 1;
        self
    }

    pub fn str(&mut self, str: &str) -> &mut Self {
        self.buf[self.len..][..str.len()].copy_from_slice(str.as_bytes());
        self.len += str.len();
        self
    }

    pub fn bin_u8(&mut self, val: u8) -> &mut Self {
        for i in 0..8 {
            let bit = (val >> (7 - i)) & 1;
            self.char(if bit == 1 { b'1' } else { b'0' });
        }
        self
    }

    pub fn hex_u8(&mut self, val: u8) -> &mut Self {
        self.char(hex(val >> 4));
        self.char(hex(val & 0xF));
        self
    }

    pub fn hex_u32(&mut self, val: u32) -> &mut Self {
        for i in 0..4 {
            let b = (val >> ((3 - i) * 8)) as u8;
            self.hex_u8(b);
        }
        self
    }

    pub fn buf(&self) -> &[u8] {
        &self.buf[..self.len]
    }
}
