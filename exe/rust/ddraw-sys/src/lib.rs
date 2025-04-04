//! Bindings to the DirectDraw API.

// Wrap and re-export the bindgen module so we can suppress warnings.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]
// Help rust-analyzer ignore that we're using stdcall:
#![allow(unsupported_fn_ptr_calling_conventions)]

mod bindgen;
pub use bindgen::*;
