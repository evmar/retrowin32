#![doc = r" Generated code, do not edit.  See winapi/builtin.rs for an overview."]
#![allow(unused_imports)]
#![allow(unused_variables)]
use win32_system::dll::*;
mod wrappers {
    use crate as oleaut32;
    use crate::*;
    use ::memory::Extensions;
    use win32_system::{System, trace};
    use win32_winapi::{calling_convention::*, *};
}
const SHIMS: [Shim; 0usize] = [];
pub const DLL: BuiltinDLL = BuiltinDLL {
    file_name: "oleaut32.dll",
    shims: &SHIMS,
    raw: std::include_bytes!("../oleaut32.dll"),
};
