use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn dllexport(_attr: TokenStream, item: TokenStream) -> TokenStream {
    //let input = parse_macro_input!(item as syn::Item);
    let tokens = item;
    println!("dllexport tok:: {} ::tok", tokens);
    tokens
}
