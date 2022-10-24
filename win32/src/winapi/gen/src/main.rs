//! Code generator for winapi functions.
//! Generates functions that pop arguments off the x86 stack.

use std::io::Write;

use proc_macro2::TokenStream;
use quote::quote;

/// Process one function, generating the wrapper function that calls it.
fn process_fn(module: &syn::Ident, func: &syn::ItemFn) -> TokenStream {
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
            // first param, the x86
            args.push(quote!(x86));
        } else {
            args.push(quote!(#name));
            let ty = &arg.ty;
            body.push(quote!(let #name: #ty = from_x86(x86);));
        }
    }
    quote!(fn #name(x86: &mut X86) {
        #(#body)*
        x86.regs.eax = winapi::#module::#name(#(#args),*) as u32;
    })
}

/// Process one module, generating the wrapper functions and resolve helper.
fn process_mod(module: &syn::Ident, path: &str) -> anyhow::Result<TokenStream> {
    let buf = std::fs::read_to_string(path)?;
    let file = syn::parse_file(&buf)?;
    let mut fns = Vec::new();
    let mut matches = Vec::new();
    for item in &file.items {
        match item {
            syn::Item::Fn(func) => {
                // if func.attrs.iter().any(|attr| attr.path.is_ident("winapi")) {

                let is_public = match func.vis {
                    syn::Visibility::Public(_) => true,
                    _ => false,
                };
                let name = func.sig.ident.to_string();
                let is_upper = name.chars().next().unwrap().is_uppercase();
                if is_public && is_upper {
                    fns.push(process_fn(&module, func));
                    let ident = &func.sig.ident;
                    let quoted = ident.to_string();
                    matches.push(quote!(#quoted => #ident));
                }
            }
            // syn::Item::Struct(_) => todo!(),
            _ => {}
        }
    }
    if fns.is_empty() {
        return Ok(quote!());
    }
    Ok(quote! {
        pub mod #module {
            use super::*;
            use winapi::#module::*;

            #(#fns)*
            pub fn resolve(name: &str) -> Option<fn(&mut X86)> {
                Some(match name {
                    #(#matches,)*
                    _ => return None,
                })
            }
        }
    })
}

/// Process multiple files, generating a single Rust output file.
fn process(args: std::env::Args) -> anyhow::Result<TokenStream> {
    let mut names = Vec::new();
    let mut mods = Vec::new();
    for path in args {
        let module = std::path::Path::new(&path)
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap();
        let module = quote::format_ident!("{}", module);
        mods.push(process_mod(&module, &path)?);
        names.push(module);
    }
    Ok(quote! {
        /// Generated code, do not edit.

        use crate::{memory::Memory, winapi, x86::X86};

        unsafe fn smuggle<T: ?Sized>(x: &T) -> &'static T {
            std::mem::transmute(x)
        }
        unsafe fn smuggle_mut<T: ?Sized>(x: &mut T) -> &'static mut T {
            std::mem::transmute(x)
        }

        unsafe trait FromX86 {
            fn from_x86(x86: &mut X86) -> Self;
        }
        unsafe impl FromX86 for u32 {
            fn from_x86(x86: &mut X86) -> Self {
                x86.pop()
            }
        }
        unsafe impl<T: From<u32>> FromX86 for Option<T> {
            fn from_x86(x86: &mut X86) -> Self {
                let val = x86.pop();
                if val == 0 {
                    None
                } else {
                    Some(T::from(val))
                }
            }
        }
        unsafe impl FromX86 for &mut [u8] {
            fn from_x86(x86: &mut X86) -> Self {
                let ofs = x86.pop() as usize;
                let len = x86.pop() as usize;
                unsafe { smuggle_mut(&mut x86.mem[ofs..ofs + len]) }
            }
        }
        unsafe impl FromX86 for &str {
            fn from_x86(x86: &mut X86) -> Self {
                let ofs = x86.pop() as usize;
                let strz = x86.mem[ofs..].read_strz();
                unsafe { smuggle(strz) }
            }
        }

        // This is used by the macro but rust-analyzer gets confused and thinks it's dead.
        #[allow(dead_code)]
        pub fn from_x86<T: FromX86>(x86: &mut X86) -> T {
            T::from_x86(x86)
        }

        #(#mods)*
    })
}

fn rustfmt(tokens: &mut String) -> anyhow::Result<()> {
    // Stolen from https://github.com/microsoft/windows-rs/blob/master/crates/tools/lib/src/lib.rs
    let mut child = std::process::Command::new("rustfmt")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()?;
    let mut stdin = child.stdin.take().ok_or(anyhow::anyhow!("no stdin"))?;
    stdin.write_all(tokens.as_bytes())?;
    drop(stdin);
    let output = child.wait_with_output()?;

    if !output.status.success() {
        anyhow::bail!("rustfmt failed: {}", std::str::from_utf8(&output.stderr)?);
    }
    *tokens = String::from_utf8(output.stdout)?;
    Ok(())
}

fn print(tokens: TokenStream) -> anyhow::Result<()> {
    //println!("{}", tokens);
    let file = syn::parse2::<syn::File>(tokens)?;
    println!("#![allow(non_snake_case)]"); // parse2 seems to fail if it sees this.
    println!("#![allow(unused_imports)]");
    let mut text = prettyplease::unparse(&file);
    rustfmt(&mut text)?;
    println!("{}", text);
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let mut args = std::env::args();
    args.next();
    let tokens = process(args)?;
    print(tokens)?;
    Ok(())
}
