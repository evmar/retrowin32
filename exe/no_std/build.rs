#[cfg(target_family = "unix")]
fn main() {
    println!("cargo:rerun-if-env-changed=XWIN");
    let xwin = std::env::var("XWIN")
        .unwrap_or_else(|_| std::env::var("HOME").unwrap() + "/.xwin-cache/splat");
    for dir in ["crt/lib/x86", "sdk/lib/ucrt/x86", "sdk/lib/um/x86"] {
        println!(r"cargo:rustc-link-search={xwin}/{dir}");
    }

    // These libraries provide builtin symbols like memcpy,
    // which are needed by the Rust standard library.
    // We don't actually want to use memset etc. so we define our own in lib.rs,
    // and the rest get dead code stripped.
    println!("cargo::rustc-link-arg=/DEFAULTLIB:vcruntime.lib");
    println!("cargo::rustc-link-arg=/DEFAULTLIB:msvcrt.lib");

    // Pass a version number here for the subsystem to make the executable
    // compatible with win2k.
    // https://waleedassar.blogspot.com/2012/08/major-minorsubsystemversion.html
    println!("cargo::rustc-link-arg=/subsystem:console,4.0");
}

#[cfg(target_family = "windows")]
fn main() {}
