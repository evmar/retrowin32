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
    // Skip first arg, the &Machine.
    for arg in func.sig.inputs.iter().skip(1) {
        let arg = match arg {
            syn::FnArg::Typed(arg) => arg,
            _ => unimplemented!(),
        };
        let name = match arg.pat.as_ref() {
            syn::Pat::Ident(ident) => &ident.ident,
            _ => unimplemented!(),
        };

        args.push(quote!(#name));
        let ty = &arg.ty;
        body.push(quote! {
            let #name = unsafe { <#ty>::from_stack(&mut machine.x86.mem, machine.x86.regs.esp + stack_offset) };
            stack_offset += <#ty>::stack_consumed();
        });
    }
    quote!(pub fn #name(machine: &mut Machine) {
        // We expect all the stack_offset math to be inlined by the compiler into plain constants.
        // TODO: reading the args in reverse would produce fewer bounds checks...
        let mut stack_offset = 4u32;
        #(#body)*
        machine.x86.regs.eax = #module::#name(machine, #(#args),*).to_raw();
        machine.x86.regs.eip = machine.x86.mem.read_u32(machine.x86.regs.esp);
        machine.x86.regs.esp += stack_offset;
    })
}

// TODO: this fn is used by main.rs, but not lib.rs.
#[allow(dead_code)]
pub fn resolve_fn(fn_names: Vec<&syn::Ident>) -> TokenStream {
    if fn_names.is_empty() {
        return quote! {
            fn resolve(_sym: &winapi::ImportSymbol) -> Option<fn(&mut Machine)> {
                None
            }
        };
    };
    let matches = fn_names.into_iter().map(|sym| {
        let quoted = sym.to_string();
        quote!(#quoted => #sym)
    });
    quote! {
        fn resolve(sym: &winapi::ImportSymbol) -> Option<fn(&mut Machine)> {
            Some(match *sym {
                winapi::ImportSymbol::Name(name) => match name {
                    #(#matches,)*
                    _ => return None,
                }
                _ => return None, // TODO: ordinal
            })
        }
    }
}
