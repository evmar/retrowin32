use super::shims::FromX86;

pub type WORD = u16;
pub type DWORD = u32;

macro_rules! declare_handle {
    ($name:ident) => {
        #[derive(Debug, Eq, PartialEq)]
        #[repr(transparent)]
        pub struct $name(pub u32);
        impl FromX86 for $name {
            fn from_raw(raw: u32) -> Self {
                $name(raw)
            }
        }
    };
}

declare_handle!(HFILE);
declare_handle!(HMODULE);
