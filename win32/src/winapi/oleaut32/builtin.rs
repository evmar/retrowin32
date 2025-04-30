#![doc = r" Generated code, do not edit.  See winapi/builtin.rs for an overview."]
#![allow(unused_imports)]
#![allow(unused_variables)]
use crate::{
    shims::{Handler, Shim},
    winapi::builtin::BuiltinDLL,
};
mod wrappers {
    use crate::{
        System,
        calling_convention::*,
        machine::Machine,
        winapi::{self, *},
    };
    use ::memory::Extensions;
    use winapi::oleaut32::*;
}
const SHIMS: [Shim; 0usize] = [];
pub const DLL: BuiltinDLL = BuiltinDLL {
    file_name: "oleaut32.dll",
    shims: &SHIMS,
    raw: std::include_bytes!("../../../dll/oleaut32.dll"),
};
