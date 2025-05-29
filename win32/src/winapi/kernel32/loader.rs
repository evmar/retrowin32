//! PE image loading.
//!
//! The two main entry points are load_exe and load_dll.
//! These call into a shared load_module, which does the PE loading
//! as well as recursively resolving imported modules.

use super::{get_state, init::retrowin32_main};
use crate::machine::Machine;
use memory::{Extensions, ExtensionsMut, Mem};
use std::collections::{HashMap, HashSet};
use win32_system::{
    System,
    dll::{DLLResolution, Shim},
    host,
    memory::{Mapping, Memory},
};
use win32_winapi::{HMODULE, WindowsPath};

/// Create a memory mapping, optionally copying some data to it.
fn map_memory(memory: &mut Memory, mapping: Mapping, buf: Option<&[u8]>) {
    let Mapping { addr, size, .. } = *memory.mappings.add(mapping);

    let memory_end = addr + size;
    if memory_end > memory.len() {
        panic!("not enough memory reserved");
    }

    if let Some(buf) = buf {
        memory
            .mem()
            .sub32_mut(addr, buf.len() as u32)
            .copy_from_slice(buf);
    }
}

/// Copy the file header itself into memory, choosing a base address.
fn load_image(
    memory: &mut Memory,
    module_name: &str,
    file: &pe::File,
    buf: &[u8],
    relocate: Option<Option<u32>>,
) -> u32 {
    let addr = match relocate {
        Some(Some(addr)) => addr,
        Some(None) => memory.mappings.find_space(file.opt_header.SizeOfImage),
        None => file.opt_header.ImageBase,
    };

    let first_page_size = std::cmp::min(buf.len(), 0x1000);
    map_memory(
        memory,
        Mapping {
            addr,
            size: first_page_size as u32,
            module: Some(module_name.into()),
            desc: format!("file header"),
            flags: pe::IMAGE_SCN::MEM_READ,
        },
        Some(&buf[..first_page_size]),
    );

    addr
}

/// Load a PE section into memory.
fn load_section(
    memory: &mut Memory,
    module_name: &str,
    base: u32,
    buf: &[u8],
    sec: &pe::IMAGE_SECTION_HEADER,
) {
    let mut src = sec.PointerToRawData as usize;
    if src == 1 {
        // Graphism (crinkler) hacks this as 1 but gets loaded as if it was zero.
        // TODO: something about alignment?  Maybe this section gets ignored?
        src = 0;
    }
    let dst = base + sec.VirtualAddress;
    // sec.SizeOfRawData is the amount of data in the file that should be copied to memory.
    // sec.VirtualSize is the in-memory size of the resulting section, which can be:
    // - greater than SizeOfRawData for sections that should be zero-filled (like uninitialized data),
    // - less than SizeOfRawData because SizeOfRawData is padded up to FileAlignment(!),
    // - and also seems to sometimes be zero, I guess in favor of SizeOfRawData.

    let mapping_size = sec.VirtualSize.max(sec.SizeOfRawData);
    let data_size = sec.SizeOfRawData;
    let flags = sec.characteristics().unwrap();

    // Decide whether t load the section contents from the file.
    // Note: kkrunchy-packed files have a single section marked
    // CODE | INITIALIZED_DATA | UNINITIALIZED_DATA | MEM_EXECUTE | MEM_READ | MEM_WRITE
    // so we ignore the UNINITIALIZED_DATA flag.
    // Another executable has its .bss section with non-zero SizeOfRawData, so we can't trust that
    // either.
    let load_data =
        flags.contains(pe::IMAGE_SCN::CODE) || flags.contains(pe::IMAGE_SCN::INITIALIZED_DATA);

    let mapping = Mapping {
        addr: dst,
        size: mapping_size,
        module: Some(module_name.into()),
        desc: format!("{:?} ({:?})", sec.name().unwrap_or("[invalid]"), flags),
        flags,
    };

    map_memory(
        memory,
        mapping,
        if load_data && data_size > 0 {
            Some(&buf[src..][..data_size as usize])
        } else {
            None
        },
    );
}

/// Load the exports section of a module, gathering addresses of exported symbols.
fn load_exports(image_base: u32, image: &[u8], exports_data: &[u8]) -> Exports {
    let dir = pe::read_exports(exports_data);

    let mut fns = Vec::new();
    for addr in dir.fns(image) {
        if addr != 0 {
            fns.push(image_base + addr);
        } else {
            // Can have 0 entries in the table due to ordinals, e.g. comctl32
            // defines ordinal 17 but not earlier, then earlier entries are 0.
            fns.push(0);
        }
    }

    let mut names = HashMap::new();
    for (name, i) in dir.names(image) {
        let addr = fns[i as usize];
        names.insert(name.to_vec(), addr);
    }

    Exports {
        ordinal_base: dir.Base,
        fns,
        names,
    }
}

async fn load_imports(machine: &mut Machine, base: u32, imports_addr: &pe::IMAGE_DATA_DIRECTORY) {
    // Traverse the ILT, gathering up addresses that need to be fixed up to point at
    // the correct targets.
    let mut patches: Vec<(u32, u32)> = Vec::new();

    let image: &[u8] = unsafe { machine.mem().detach().slice(base..) };
    let Some(section) = imports_addr.as_slice(image) else {
        return;
    };
    for imports in pe::read_imports(section) {
        let Some(dll_name) = imports.image_name(image).try_ascii() else {
            log::warn!("skipping non-ascii import {:?}", imports.image_name(image));
            continue;
        };
        let hmodule = match load_dll(machine, dll_name).await {
            Ok(hmodule) => hmodule,
            Err(err) => {
                log::warn!("failed to load import {dll_name:?}: {err}");
                HMODULE::null()
            }
        };
        let mut state = get_state(machine);
        let module = state.modules.get_mut(&hmodule);

        // We traverse the IAT even if the module failed to load, because we still label the
        // IAT entries and set them to zero in that case.
        // Use DLL name from module if available.
        let (dll_name, mut exports) = match module {
            Some(module) => (module.name.as_str(), Some(&mut module.exports)),
            None => (dll_name, None),
        };
        for (iat_addr, entry) in imports.iat_iter(image) {
            let iat_addr = iat_addr + base;
            let sym = entry.as_import_symbol(image);
            let name = format!("{}!{}", dll_name, sym.to_string());
            // TODO: it's useful to label the IAT like this, but we can't mutate machine
            // deep in this call stack.  Figure out some other approach.
            // machine
            //     .memory
            //     .labels
            //     .insert(iat_addr, format!("{}@IAT", name));

            let resolved_addr = if let Some(exports) = &mut exports {
                if let Some(addr) = exports.resolve(&sym) {
                    Some(addr)
                } else {
                    log::warn!("missing symbol {name}");
                    None
                }
            } else {
                // No need to log anything here, because we already logged that the module itself
                // failed to load.
                None
            };

            let addr = resolved_addr.unwrap_or(0);
            patches.push((iat_addr, addr));
        }
    }

    for (addr, target) in patches {
        machine.mem().put_pod::<u32>(addr, target);
    }
}

/// TODO: any direct user of this misses out on the aliasing performed in resolve_dll.
pub fn normalize_module_name(name: &str) -> String {
    let mut name = name.to_ascii_lowercase();
    if !name.ends_with(".dll") && !name.ends_with(".exe") && !name.ends_with(".") {
        name.push_str(".dll");
    }
    name
}

/// Load an exe or dll without resolving its imports.
fn load_one_module(
    memory: &mut Memory,
    module_name: String,
    buf: &[u8],
    file: &pe::File,
    relocate: Option<Option<u32>>,
) -> anyhow::Result<Module> {
    let base = load_image(memory, &module_name, file, buf, relocate);

    for sec in file.sections.iter() {
        load_section(memory, &module_name, base, buf, sec);
    }

    if base != file.opt_header.ImageBase {
        if let Some(relocs) = file.get_data_directory(pe::IMAGE_DIRECTORY_ENTRY::BASERELOC) {
            let image = memory.mem().slice(base..);
            if let Some(sec) = relocs.as_slice(image) {
                // Warning: apply_relocs wants to mutate arbitrary memory, which violates Rust aliasing rules.
                // It has started silently failing in release builds in the past...
                pe::apply_relocs(
                    file.opt_header.ImageBase,
                    base,
                    sec,
                    |addr| {
                        let addr = base + addr;
                        memory.mem().get_pod::<u32>(addr)
                    },
                    |addr, val| {
                        let addr = base + addr;
                        memory.mem().put_pod::<u32>(addr, val);
                    },
                );
            }
        }
    }

    let image = memory.mem().slice(base..);
    let exports = if let Some(dir) = file.get_data_directory(pe::IMAGE_DIRECTORY_ENTRY::EXPORT) {
        let section = dir
            .as_slice(image)
            .ok_or_else(|| anyhow::anyhow!("invalid exports"))?;
        let exports = load_exports(base, image, section);
        for (name, &addr) in exports.names.iter() {
            if let Some(name) = name.try_ascii() {
                memory.labels.insert(addr, format!("{module_name}!{name}"));
            }
        }
        exports
    } else {
        Exports::default()
    };

    let resources = file
        .data_directory
        .get(pe::IMAGE_DIRECTORY_ENTRY::RESOURCE as usize)
        .cloned();

    let entry_point = if file.opt_header.AddressOfEntryPoint == 0 {
        None
    } else {
        Some(base + file.opt_header.AddressOfEntryPoint)
    };

    Ok(Module {
        name: module_name,
        image_base: base,
        exports,
        dynamic_imports: Default::default(),
        resources,
        entry_point,
    })
}

/// Load a module's imports and invoke its DllMain().
async fn init_module(
    machine: &mut Machine,
    file: &pe::File,
    module: Module,
) -> anyhow::Result<HMODULE> {
    let Module {
        image_base,
        entry_point,
        ..
    } = module;

    // Register module before loading imports, because loaded imports may refer back to this module.
    let hmodule = HMODULE::from_raw(image_base);
    get_state(machine).modules.insert(hmodule, module);

    if let Some(imports) = file.get_data_directory(pe::IMAGE_DIRECTORY_ENTRY::IMPORT) {
        Box::pin(load_imports(machine, image_base, imports)).await;
    }

    // Invoke DllMain() if present.
    #[allow(non_snake_case)]
    if file.header.Characteristics.contains(pe::IMAGE_FILE::DLL) && entry_point.is_some() {
        let entry_point = entry_point.unwrap();
        let hInstance = image_base;
        let fdwReason = 1u32; // DLL_PROCESS_ATTACH
        let lpvReserved = 0u32;
        machine
            .call_x86(entry_point, vec![hInstance, fdwReason, lpvReserved])
            .await;
    }

    Ok(hmodule)
}

/// Load an exe or dll, recursively initializing its imports.
async fn load_module(
    machine: &mut Machine,
    module_name: String,
    buf: &[u8],
    file: &pe::File,
    relocate: Option<Option<u32>>,
) -> anyhow::Result<HMODULE> {
    let module = load_one_module(&mut machine.memory, module_name, buf, file, relocate)?;
    init_module(machine, file, module).await
}

pub async fn start_exe(
    machine: &mut Machine,
    exe_name: String,
    relocate: Option<Option<u32>>,
) -> anyhow::Result<()> {
    let path = WindowsPath::new(&exe_name);
    let buf = read_file(&*machine.host, path)?; // TODO: .await
    let file = pe::File::parse(&buf)?;

    // We need a current thread to load the exe, because we may need to
    // call DllMain()s on dependent DLLs.  But we only have the stack size
    // for that thread once we've parsed the file, so here is the place to
    // create it.  TODO: this feels wrong.  Perhaps caller should parse pe itself?
    // TODO: zig asks for 16mb of stack space, which then conflicts with the
    // exe load address.  Clip it here.
    let stack_size = file.opt_header.SizeOfStackReserve.min(0x10_0000);
    machine.new_thread(false, stack_size, 0, &[]);

    let filename = std::str::from_utf8(path.file_name().unwrap()).unwrap();
    let module_name = normalize_module_name(&filename);

    let module = load_one_module(&mut machine.memory, module_name, &buf, &file, relocate)?;
    let image_base = module.image_base;
    let entry_point = module.entry_point.unwrap();
    init_module(machine, &file, module).await.unwrap();
    get_state(machine).image_base = image_base;
    retrowin32_main(machine, entry_point).await;
    Ok(())
}

/// The result of loading an exe or DLL.
#[derive(Debug, Default)]
pub struct Module {
    /// Normalized module name, e.g. "kernel32.dll".
    pub name: String,

    /// Address where the module was loaded.
    pub image_base: u32,

    pub resources: Option<pe::IMAGE_DATA_DIRECTORY>,

    pub exports: Exports,

    /// Addresses of functions that were imported at runtime (vs load time).
    /// This is used for reconstructing the IAT when dumping a packed exe.
    pub dynamic_imports: HashSet<u32>,

    pub entry_point: Option<u32>,
}

impl Module {
    pub fn resources<'m>(&self, mem: Mem<'m>) -> Option<&'m [u8]> {
        let dir = self.resources.as_ref()?;
        Some(dir.as_slice(mem.slice(self.image_base..))?)
    }
}

#[derive(Debug, Default)]
pub struct Exports {
    /// Exported function name => resolved address.
    pub names: HashMap<Vec<u8>, u32>,

    /// fns[ordinal - ordinal base] => resolved address.
    pub ordinal_base: u32,
    pub fns: Vec<u32>,
}

impl Exports {
    pub fn resolve(&self, sym: &pe::ImportSymbol) -> Option<u32> {
        match *sym {
            pe::ImportSymbol::Name(name) => self.names.get(name).copied(),
            pe::ImportSymbol::Ordinal(ord) => {
                self.fns.get((ord - self.ordinal_base) as usize).copied()
            }
        }
    }

    /// Given a resolved address, find the name of the symbol that was used to resolve it.
    pub fn name_from_addr(&self, addr: u32) -> Option<&[u8]> {
        self.names
            .iter()
            .find(|&(_, &v)| v == addr)
            .map(|(k, _)| k.as_slice())
    }
}

fn read_file(host: &dyn host::Host, path: &WindowsPath) -> anyhow::Result<Vec<u8>> {
    let mut buf = Vec::new();
    let Ok(mut file) = host.open(path, host::FileOptions::read()) else {
        anyhow::bail!("{path} not found");
    };
    file.read_to_end(&mut buf).unwrap();
    // TODO: close file.
    buf.shrink_to_fit();
    Ok(buf)
}

/// Read a DLL from the DLL search path.
fn find_dll(machine: &Machine, filename: &str) -> anyhow::Result<Vec<u8>> {
    let exe = get_state(machine).cmdline.exe_name();
    let exe_dir = exe.rsplitn(2, '\\').last().unwrap();
    let dll_paths = [format!("{exe_dir}\\{filename}"), filename.to_string()];
    for path in &dll_paths {
        let path = WindowsPath::new(path);
        let buf = read_file(&*machine.host, path)?;
        if !buf.is_empty() {
            return Ok(buf);
        }
    }
    anyhow::bail!("{filename:?} not found in {dll_paths:?}");
}

pub async fn load_dll(machine: &mut Machine, name: &str) -> anyhow::Result<HMODULE> {
    let res = machine.resolve_dll(name);
    let module_name = res.name();

    // See if already loaded.
    if let Some((&hmodule, _)) = get_state(machine)
        .modules
        .iter()
        .find(|(_, dll)| dll.name == module_name)
    {
        return Ok(hmodule);
    }

    match res {
        DLLResolution::Builtin(builtin) => {
            let file = pe::File::parse(builtin.raw)?;
            let module_name = builtin.file_name.to_owned();
            let hmodule = load_module(machine, module_name, builtin.raw, &file, Some(None)).await?;
            register_builtin_shims(machine, builtin.shims, hmodule);
            Ok(hmodule)
        }
        DLLResolution::External(filename) => {
            let buf = find_dll(machine, &filename)?;
            let file = pe::File::parse(&buf)?;
            load_module(machine, filename.to_owned(), &buf, &file, Some(None)).await
        }
    }
}

fn register_builtin_shims(machine: &mut Machine, shims: &'static [Shim], hmodule: HMODULE) {
    // For builtins, register all the exports as known symbols.
    let exports = {
        let state = get_state(machine);
        let module = state.modules.get(&hmodule).unwrap();

        // It is critical that the DLL's exports match up to the shims array;
        // this is ensured by both being generated by the same generator.
        // Note we filter out 0 entries in the exports table, due to how ordinals
        // can create holes; see where .fns is filled in.
        module
            .exports
            .fns
            .iter()
            .copied()
            .filter(|&addr| addr != 0)
            .collect::<Vec<_>>()
    };

    for (&addr, shim) in exports.iter().zip(shims) {
        machine.emu.shims.register(addr, Ok(shim));
    }
}

pub fn get_symbol(machine: &Machine, dll: &str, name: &str) -> u32 {
    let res = machine.resolve_dll(dll);
    let dll_name = res.name();

    let state = get_state(machine);
    let module = state.modules.values().find(|m| m.name == dll_name).unwrap();
    module
        .exports
        .resolve(&pe::ImportSymbol::Name(name.as_bytes()))
        .unwrap()
}
