//! Code generator for winapi functions.
//! Generates builtin.rs and win32/dll/*.

mod dll;
mod generate;
mod parse;

use std::path::Path;

use walkdir::WalkDir;

fn write_if_changed(path: &Path, contents: &[u8]) -> anyhow::Result<()> {
    if let Ok(old_contents) = std::fs::read(path) {
        if old_contents == contents {
            return Ok(());
        }
    }
    std::fs::write(path, contents)?;
    Ok(())
}

/// Parse a directory's collection of files.
fn parse_files(dll_name: &str, root: &Path) -> anyhow::Result<Vec<(Vec<String>, syn::File)>> {
    let mut files = Vec::new();
    for entry in WalkDir::new(root).sort_by_file_name() {
        let entry = entry?;
        if entry.file_type().is_dir() {
            continue;
        }
        let path = entry.path();
        if path.file_name().unwrap() == "builtin.rs" {
            continue;
        }
        if path.extension().unwrap() != "rs" {
            continue;
        }
        let buf = std::fs::read_to_string(path)?;
        let file =
            syn::parse_file(&buf).map_err(|err| anyhow::anyhow!("{}: {}", path.display(), err))?;

        // Construct relative module path, e.g. ["kernel32", "env"]
        let mut rel_path = path
            .strip_prefix(root)
            .unwrap()
            .with_extension("")
            .components()
            .map(|c| c.as_os_str().to_string_lossy().into_owned())
            .collect::<Vec<String>>();
        rel_path.insert(0, dll_name.to_string());
        let last = rel_path.last().unwrap();
        if last == "mod" || last == "lib" {
            rel_path.pop();
        }

        files.push((rel_path, file));
    }
    Ok(files)
}

fn rustfmt(tokens: &str) -> anyhow::Result<String> {
    use std::io::Write;
    // Stolen from https://github.com/microsoft/windows-rs/blob/master/crates/tools/lib/src/lib.rs
    let mut child = std::process::Command::new("rustfmt")
        .arg("--edition")
        .arg("2024")
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
    Ok(String::from_utf8(output.stdout)?)
}

/// Assign ordinals to all fns that don't have them already.
fn assign_ordinals(fns: &mut [parse::DllExport]) -> anyhow::Result<()> {
    let mut used_ordinals = std::collections::HashSet::new();
    for dllexport in fns.iter_mut() {
        if let Some(ordinal) = dllexport.meta.ordinal {
            if !used_ordinals.insert(ordinal) {
                return Err(syn::Error::new_spanned(dllexport.func, "duplicate ordinal").into());
            }
        }
    }

    let mut ordinal = 1;
    for dllexport in fns {
        if dllexport.meta.ordinal.is_none() {
            while used_ordinals.contains(&ordinal) {
                ordinal += 1;
            }
            dllexport.meta.ordinal = Some(ordinal);
            ordinal += 1;
        }
    }
    Ok(())
}

fn process_builtin_dll(dll_dir: &Path) -> anyhow::Result<()> {
    let dll_name = dll_dir.file_name().unwrap().to_string_lossy();

    let files = parse_files(&dll_name, &dll_dir.join("src"))?;
    let mut dllexports = parse::DllExports::default();
    for (module_name, file) in &files {
        if let Err(err) = parse::gather_dllexports(module_name, &file.items, &mut dllexports) {
            let loc = err.span().start();
            // TODO: get file name from span, needs later syn version.
            anyhow::bail!("{:?}:{}:{}: {}", module_name, loc.line, loc.column, err);
        }
    }

    // Sort by name, then assign ordinals satisfying the ordinals that were specified,
    // then sort by ordinal to ensure the output is deterministic.
    dllexports.fns.sort_by(|a, b| a.flat_name.cmp(&b.flat_name));
    assign_ordinals(&mut dllexports.fns).unwrap();
    dllexports.fns.sort_by_key(|e| e.meta.ordinal.unwrap());

    dll::generate_dll(&dll_name, &dllexports, |name, content| {
        write_if_changed(&dll_dir.join(name), &content)
    })?;

    let builtins_module = generate::shims_module(&dll_name, dllexports);
    let text = rustfmt(&builtins_module.to_string())?;
    write_if_changed(&dll_dir.join("src/builtin.rs"), text.as_bytes())?;

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let args = std::env::args().skip(1);

    for src_dir in args {
        process_builtin_dll(src_dir.as_ref())
            .map_err(|err| anyhow::anyhow!("processing module: {}", err))?;
    }

    Ok(())
}
