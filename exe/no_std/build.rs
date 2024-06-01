#[cfg(target_family = "unix")]
fn main() {
    println!("cargo:rerun-if-env-changed=XWIN");
    if let Ok(xwin) = std::env::var("XWIN") {
        for dir in ["crt/lib/x86", "sdk/lib/ucrt/x86", "sdk/lib/um/x86"] {
            println!(r"cargo:rustc-link-search={xwin}/{dir}");
        }
    }

    // These libraries provide builtin symbols like memcpy.
    println!("cargo::rustc-link-arg=/DEFAULTLIB:vcruntime.lib");
    println!("cargo::rustc-link-arg=/DEFAULTLIB:msvcrt.lib");
}

#[cfg(target_family = "windows")]
fn main() {}
