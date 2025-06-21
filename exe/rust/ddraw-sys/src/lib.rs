//! Bindings to the DirectDraw API.

// Wrap and re-export the bindgen module so we can suppress warnings.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]

mod bindgen;
pub use bindgen::*;
