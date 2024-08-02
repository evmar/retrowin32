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

macro_rules! vtable_fwd {
    (($iface:ident :: $fn:ident)) => {
        $iface::shims::$fn
    };
}
pub(crate) use vtable_fwd;

macro_rules! vtable_entry {
    ($iface:ident $shims:ident $fn:ident todo) => {
        Err(format!(
            "{} vtable::{}",
            stringify!($iface),
            stringify!($fn)
        ))
    };
    ($iface:ident $shims:ident $fn:ident ok) => {
        Ok(&$shims::$fn)
    };
    ($iface:ident $shims:ident $fn:ident $target:tt) => {
        Ok(&$crate::winapi::com::vtable_fwd!($target))
    };
}
pub(crate) use vtable_entry;

macro_rules! vtable {
    ($iface:ident $shims:ident $($fn:ident: $impl:tt,)*) => {
        #[repr(C)]
        struct Vtable {
            $($fn: u32),*
        }
        unsafe impl memory::Pod for Vtable {}
        impl Vtable {
            fn new(mut register: impl FnMut(Result<&'static $crate::shims::Shim, String>) -> u32) -> Self {
                Vtable {
                    $($fn: register($crate::winapi::com::vtable_entry!($iface $shims $fn $impl))),*
                }
            }
        }

        pub fn vtable(mem: memory::Mem, heap: &mut $crate::winapi::heap::Heap, register: impl FnMut(Result<&'static $crate::shims::Shim, String>) -> u32) -> u32 {
            let addr = heap.alloc(mem, std::mem::size_of::<Vtable>() as u32);
            let vtable = Vtable::new(register);
            *mem.view_mut::<Vtable>(addr) = vtable;
            addr
        }
    };
}
pub(crate) use vtable;
