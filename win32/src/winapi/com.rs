#[allow(non_snake_case)]
#[repr(C)]
#[derive(PartialEq)]
pub struct GUID(pub (u32, u16, u16, [u8; 8]));
unsafe impl memory::Pod for GUID {}

impl std::fmt::Debug for GUID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:08x}-{:04x}-{:04x}-{:04x}-",
            self.0 .0,
            self.0 .1,
            self.0 .2,
            u16::from_le_bytes(self.0 .3[..2].try_into().unwrap())
        )?;
        for b in &self.0 .3[2..] {
            write!(f, "{:02x}", b)?;
        }
        Ok(())
    }
}

macro_rules! vtable {
    ($($fn:ident: $impl:tt,)*) => {
        // macro is parsed by win32-derive codegen
    };
}
pub(crate) use vtable;
