This directory contains Rust programs that produce win32 .exe files. They can be
used to test various Windows APIs, and even run on native Windows.

After building via `build.sh`, the exes will be found in
`./target/i686-pc-windows-msvc/release/`.

## Rust configuration

We want win32 binaries for win2k era computers, which is gradually falling off
the edge of the Rust-compatible build targets. In particular we want 32-bit x86
and don't yet (ever?) support SSE2. We used to target i586 to get this, but Rust
dropped that in favor of i686.

The trick is to use the `target-cpu=pentium` flag. We also cannot build with
`std`, as that is pre-compiled without our CPU flag.

See the config in `.cargo/config.toml`.

Dump available CPUs:

```
rustc --target=i686-pc-windows-msvc --print=target-cpus
```

## rust-analyzer

rust-analyzer gets angry about the x86 assembly used in here. In theory we could
maybe make this a separate Rust "workspace" so that rust-analyzer could pick up
the specific Rust configuration to put it into x86 mode, but you currently
cannot configure the Rust feature flags on a per-workspace basis, and the
feature flags we specify for retrowin32 then end up breaking rust-analyzer for
the separate workspace.
