#[cfg(target_family = "unix")]
fn main() {
    println!("cargo:rerun-if-env-changed=XWIN");
    let xwin = std::env::var("XWIN").unwrap();
    for dir in ["crt/lib/x86", "sdk/lib/ucrt/x86", "sdk/lib/um/x86"] {
        println!(r"cargo:rustc-link-search={xwin}/{dir}");
    }
}

#[cfg(target_family = "windows")]
fn main() {}
