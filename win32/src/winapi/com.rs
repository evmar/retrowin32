pub const E_NOINTERFACE: u32 = 0x80004002;

#[allow(non_snake_case)]
#[repr(C)]
#[derive(PartialEq)]
pub struct GUID {
    pub Data1: u32,
    pub Data2: u16,
    pub Data3: u16,
    pub Data4: [u8; 8],
}
unsafe impl memory::Pod for GUID {}

impl std::fmt::Debug for GUID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:08x}-{:04x}-{:04x}-{:04x}-",
            self.Data1,
            self.Data2,
            self.Data3,
            u16::from_le_bytes(self.Data4[..2].try_into().unwrap())
        )?;
        for b in &self.Data4[2..] {
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
