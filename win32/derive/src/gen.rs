use proc_macro2::TokenStream;
use quote::quote;

/// Generate the wrapper function that calls a win32api function by taking arguments using from_x86.
///
/// winapi is stdcall, which means args are pushed right to left.
/// They are callee-cleaned which means they are popped left to right, as done here.
/// The caller of winapi functions is responsible for pushing/popping the
/// return address, because some callers actually 'jmp' directly.
///
/// This macro generates shim wrappers of functions, taking their
/// input args off the stack and forwarding their return values via eax.
pub fn fn_wrapper(module: TokenStream, func: &syn::ItemFn) -> TokenStream {
    let name = &func.sig.ident;
    let mut args: Vec<TokenStream> = Vec::new();
    let mut body: Vec<TokenStream> = Vec::new();
    for (i, arg) in func.sig.inputs.iter().enumerate() {
        let arg = match arg {
            syn::FnArg::Typed(arg) => arg,
            _ => unimplemented!(),
        };

        let name = match arg.pat.as_ref() {
            syn::Pat::Ident(ident) => &ident.ident,
            _ => unimplemented!(),
        };
        if i == 0 {
            // first param, the machine
            args.push(quote!(machine));
        } else {
            args.push(quote!(#name));
            let ty = &arg.ty;
            body.push(quote!(let #name: #ty = unsafe { from_x86(&mut machine.x86) };));
        }
    }
    quote!(pub fn #name(machine: &mut Machine) {
        #(#body)*
        machine.x86.regs.eax = #module::#name(#(#args),*).to_raw();
    })
}
