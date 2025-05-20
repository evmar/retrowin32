use crate::System;
use win32_winapi::calling_convention::ABIReturn;

pub type SyncHandler = unsafe fn(&mut dyn System, u32) -> ABIReturn;
pub type AsyncHandler =
    unsafe fn(
        &mut dyn System,
        u32,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ABIReturn> + '_>>;

#[derive(Debug, Clone, Copy)]
pub enum Handler {
    Sync(SyncHandler),
    Async(AsyncHandler),
}

#[derive(Debug)]
pub struct Shim {
    pub name: &'static str,
    pub func: Handler,
}

pub struct BuiltinDLL {
    pub file_name: &'static str,
    /// The xth function in the DLL represents a call to shims[x].
    pub shims: &'static [Shim],
    /// Raw bytes of generated .dll.
    pub raw: &'static [u8],
}
