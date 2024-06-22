fn hex(b: u8) -> u8 {
    if b < 10 {
        b'0' + b
    } else {
        b'a' + (b - 10)
    }
}

fn dec(b: u8) -> u8 {
    b'0' + b
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

    pub fn bstr(&mut self, str: &[u8]) -> &mut Self {
        self.buf[self.len..][..str.len()].copy_from_slice(str);
        self.len += str.len();
        self
    }

    pub fn str(&mut self, str: &str) -> &mut Self {
        self.bstr(str.as_bytes())
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

    pub fn f64(&mut self, valf: f64) -> &mut Self {
        // `val as u32` generates 40 lines of asm in Rust, I think due to handling
        // corner cases.  This dumb impl is simpler.
        let mut val: i32 = 1000;
        unsafe {
            // val = (valf * 1000.0) as i32
            core::arch::asm!(
                "fld qword ptr [{valf}]",
                "fimul dword ptr [{val}]",
                "fistp dword ptr [{val}]",
                valf = in(reg) &valf,
                val = in(reg) &mut val,
            );
        }

        let mut neg = false;
        if val < 0 {
            val = -val;
            neg = true;
        }

        let mut buf = [0u8; 64];
        let mut out = buf.len() - 1;

        for _ in 0..3 {
            let v = (val % 10) as u8;
            val /= 10;
            buf[out] = dec(v);
            out -= 1;
        }

        buf[out] = b'.';
        out -= 1;

        if val == 0 {
            buf[out] = b'0';
            out -= 1;
        } else {
            while val > 0 {
                let v = (val % 10) as u8;
                val /= 10;
                buf[out] = dec(v);
                out -= 1;
            }
        }

        if neg {
            buf[out] = b'-';
            out -= 1;
        }

        self.bstr(&buf[out + 1..])
    }

    pub fn buf(&self) -> &[u8] {
        &self.buf[..self.len]
    }
}
