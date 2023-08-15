//! Code generator for winapi functions.
//! Generates functions that pop arguments off the x86 stack.
//! TODO: move this code to lib, and switch to using a macro for codegen.

use std::io::Write;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
mod gen;

/// Process one module, generating the wrapper functions and resolve helper.
fn process_mod(module: &syn::Ident, path: &std::path::Path) -> anyhow::Result<TokenStream> {
    let dll_name = format!("{}.dll", module);
    eprintln!("{}", dll_name);

    // path may be a .rs file or a directory (module).
    let mut paths: Vec<std::path::PathBuf> = if path.extension().is_none() {
        std::fs::read_dir(path)?
            .map(|e| e.unwrap().path())
            .collect()
    } else {
        vec![path.to_path_buf()]
    };
    paths.sort();

    let mut fns = Vec::new();
    let mut shims = Vec::new();
    let mut exports = Vec::new();
    for path in paths {
        let buf = std::fs::read_to_string(&path)?;
        let file = match syn::parse_file(&buf) {
            Ok(file) => file,
            Err(err) => return Ok(err.into_compile_error().into()),
        };
        let dllexports = gen::gather_shims(&file.items)?;
        for (func, dllexport) in dllexports {
            let (wrapper, shim) = gen::fn_wrapper(quote! { winapi::#module }, func);
            fns.push(wrapper);
            shims.push(shim);

            let ordinal = match dllexport.ordinal {
                Some(ord) => quote!(Some(#ord)),
                None => quote!(None),
            };
            let name = &func.sig.ident;
            exports.push(quote!(Symbol { ordinal: #ordinal, shim: shims::#name }));
        }
    }

    let exports_count = exports.len();

    Ok(quote! {
        pub mod #module {
            use super::*;

            mod impls {
                use crate::{machine::Machine, winapi::{self, stack_args::*, types::*}};
                use winapi::#module::*;
                #(#fns)*
            }

            mod shims {
                use crate::shims::Shim;
                use super::impls;
                #(#shims)*
            }

            const EXPORTS: [Symbol; #exports_count] = [
                #(#exports),*
            ];

            pub const DLL: BuiltinDLL = BuiltinDLL {
                file_name: #dll_name,
                exports: &EXPORTS,
            };
        }
    })
}

/// Process multiple files, generating a single Rust output file.
fn process(args: std::env::Args) -> anyhow::Result<TokenStream> {
    let mut names = Vec::new();
    let mut mods = Vec::new();
    for path in args {
        let path = std::path::Path::new(&path);
        let module_name = path.file_stem().unwrap().to_string_lossy();
        let module = quote::format_ident!("{}", module_name);
        let gen = match process_mod(&module, &path) {
            Ok(gen) => gen,
            Err(err) => anyhow::bail!("processing module: {}", err),
        };
        mods.push(gen);
        names.push(module);
    }
    Ok(quote! {
        /// Generated code, do not edit.

        use crate::shims;

        pub struct Symbol {
            pub ordinal: Option<usize>,
            pub shim: shims::Shim,
        }

        pub struct BuiltinDLL {
            pub file_name: &'static str,
            pub exports: &'static [Symbol],
        }

        #(#mods)*
    })
}

fn rustfmt(tokens: &mut String) -> anyhow::Result<()> {
    // Stolen from https://github.com/microsoft/windows-rs/blob/master/crates/tools/lib/src/lib.rs
    let mut child = std::process::Command::new("rustfmt")
        .arg("--edition")
        .arg("2018")
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
    // println!("{}", tokens);
    let file = match syn::parse2::<syn::File>(tokens) {
        Ok(file) => file,
        Err(err) => anyhow::bail!("parsing macro-generated code: {}", err),
    };
    // parse2 seems to fail if it sees these, so dump them here.
    println!("#![allow(non_snake_case)]");
    println!("#![allow(non_upper_case_globals)]");
    println!("#![allow(unused_imports)]");
    println!("#![allow(unused_mut)]");
    let mut text = file.to_token_stream().to_string();
    rustfmt(&mut text)?;
    print!("{}", text);
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let mut args = std::env::args();
    args.next();
    let tokens = process(args)?;
    print(tokens)?;
    Ok(())
}
