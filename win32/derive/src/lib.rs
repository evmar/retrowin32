//! TODO: see TODO in main.rs.

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn dllexport(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
