#[cfg(not(feature = "sdl"))]
use crate::headless::GUI;
#[cfg(feature = "sdl")]
use crate::sdl::GUI;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use std::{cell::RefCell, io::Write, rc::Rc};
use win32::winapi::types::{io_error_to_win32, unix_nanos_to_filetime};
use win32::{FileOptions, FindHandle, Stat};

struct File {
    f: std::fs::File,
}

impl win32::File for File {
    fn stat(&self) -> Result<Stat, u32> {
        match self.f.metadata() {
            Ok(ref meta) => Ok(metadata_to_stat(meta)),
            Err(ref e) => Err(io_error_to_win32(e)),
        }
    }
}

impl std::io::Read for File {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.f.read(buf)
    }
}

impl std::io::Write for File {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.f.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.f.flush()
    }
}

impl std::io::Seek for File {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        self.f.seek(pos)
    }
}

struct FindIterDir {
    iter: std::fs::ReadDir,
}

impl FindHandle for FindIterDir {
    fn next(&mut self) -> Result<Option<win32::FindFile>, u32> {
        match self.iter.next() {
            Some(Ok(entry)) => {
                let path = entry.path();
                let windows_path = host_to_windows_path(&path);
                let name = path.file_name().unwrap().to_string_lossy().into_owned();
                let meta = entry.metadata().unwrap();
                Ok(Some(win32::FindFile {
                    path: windows_path,
                    name,
                    stat: metadata_to_stat(&meta),
                }))
            }
            Some(Err(ref e)) => Err(io_error_to_win32(e)),
            None => Ok(None),
        }
    }
}

struct FindIterFile {
    file: win32::FindFile,
    consumed: bool,
}

impl FindHandle for FindIterFile {
    fn next(&mut self) -> Result<Option<win32::FindFile>, u32> {
        if self.consumed {
            Ok(None)
        } else {
            self.consumed = true;
            Ok(Some(self.file.clone()))
        }
    }
}

struct Env {
    gui: Option<GUI>,
    exit_code: Option<u32>,
}

impl Env {
    pub fn new() -> Self {
        Env {
            gui: None,
            exit_code: None,
        }
    }

    pub fn ensure_gui(&mut self) -> anyhow::Result<&mut GUI> {
        if self.gui.is_none() {
            self.gui = Some(GUI::new()?);
        }
        Ok(self.gui.as_mut().unwrap())
    }
}

#[derive(Clone)]
struct EnvRef(Rc<RefCell<Env>>);

impl win32::Host for EnvRef {
    fn exit(&self, code: u32) {
        self.0.borrow_mut().exit_code = Some(code);
    }

    fn time(&self) -> u32 {
        let mut env = self.0.borrow_mut();
        let gui = env.ensure_gui().unwrap();
        gui.time()
    }

    fn system_time(&self) -> chrono::DateTime<chrono::Local> {
        chrono::Local::now()
    }

    fn get_message(&self) -> Option<win32::Message> {
        let mut env = self.0.borrow_mut();
        let gui = env.gui.as_mut().unwrap();
        gui.get_message()
    }

    fn block(&self, wait: Option<u32>) -> bool {
        let mut env = self.0.borrow_mut();
        let gui = env.gui.as_mut().unwrap();
        gui.block(wait)
    }

    fn canonicalize(&self, path: &str) -> Result<String, u32> {
        let path = windows_to_host_path(path);
        match std::fs::canonicalize(path) {
            Ok(ref p) => Ok(host_to_windows_path(p)),
            Err(ref e) => Err(io_error_to_win32(e)),
        }
    }

    fn open(&self, path: &str, options: FileOptions) -> Result<Box<dyn win32::File>, u32> {
        let path = windows_to_host_path(path);
        let result = std::fs::File::options()
            .read(options.read)
            .write(options.write)
            .truncate(options.truncate)
            .create(options.create)
            .create_new(options.create_new)
            .open(path);
        match result {
            Ok(f) => Ok(Box::new(File { f })),
            Err(ref e) => Err(io_error_to_win32(e)),
        }
    }

    fn stat(&self, path: &str) -> Result<Stat, u32> {
        let path = windows_to_host_path(path);
        match std::fs::metadata(path) {
            Ok(ref meta) => Ok(metadata_to_stat(meta)),
            Err(ref e) => Err(io_error_to_win32(e)),
        }
    }

    fn find(&self, path: &str) -> Result<Box<dyn FindHandle>, u32> {
        let path = windows_to_host_path(path);
        let full_path = match std::fs::canonicalize(path) {
            Ok(p) => p,
            Err(ref e) => return Err(io_error_to_win32(e)),
        };
        match std::fs::metadata(&full_path) {
            Ok(meta) => {
                if meta.is_dir() {
                    let iter = match std::fs::read_dir(&full_path) {
                        Ok(iter) => iter,
                        Err(ref e) => return Err(io_error_to_win32(e)),
                    };
                    Ok(Box::new(FindIterDir { iter }))
                } else {
                    let path = host_to_windows_path(&full_path);
                    let filename = full_path
                        .file_name()
                        .unwrap()
                        .to_string_lossy()
                        .into_owned();
                    let file = win32::FindFile {
                        path,
                        name: filename,
                        stat: metadata_to_stat(&meta),
                    };
                    Ok(Box::new(FindIterFile {
                        file,
                        consumed: false,
                    }))
                }
            }
            Err(ref e) => Err(io_error_to_win32(e)),
        }
    }

    fn log(&self, buf: &[u8]) {
        std::io::stdout().lock().write_all(buf).unwrap();
    }

    fn create_window(&mut self, hwnd: u32) -> Box<dyn win32::Window> {
        let mut env = self.0.borrow_mut();
        let gui = env.ensure_gui().unwrap();
        gui.create_window(hwnd)
    }

    fn create_surface(
        &mut self,
        _hwnd: u32,
        opts: &win32::SurfaceOptions,
    ) -> Box<dyn win32::Surface> {
        let mut env = self.0.borrow_mut();
        let gui = env.ensure_gui().unwrap();
        gui.create_surface(opts)
    }
}

pub fn new_host() -> impl win32::Host + Clone {
    EnvRef(Rc::new(RefCell::new(Env::new())))
}

/// Convert a `SystemTime` to hecto-nanoseconds since Windows epoch (1601-01-01).
/// See https://learn.microsoft.com/en-us/windows/win32/api/minwinbase/ns-minwinbase-filetime
fn system_time_to_filetime(t: SystemTime) -> u64 {
    unix_nanos_to_filetime(t.duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64)
}

fn metadata_to_stat(meta: &std::fs::Metadata) -> Stat {
    let kind = if meta.is_dir() {
        win32::StatKind::Directory
    } else if meta.is_file() {
        win32::StatKind::File
    } else {
        win32::StatKind::Symlink
    };
    Stat {
        kind,
        size: meta.len(),
        atime: meta.accessed().map_or(0, system_time_to_filetime),
        ctime: meta.created().map_or(0, system_time_to_filetime),
        mtime: meta.modified().map_or(0, system_time_to_filetime),
    }
}

#[cfg(unix)]
fn windows_to_host_path(mut path: &str) -> PathBuf {
    path = path
        .strip_prefix("\\\\?\\")
        .or_else(|| path.strip_prefix("\\\\.\\"))
        .unwrap_or(path);
    path = path
        .strip_prefix("Z:")
        .or_else(|| path.strip_prefix("z:"))
        .unwrap_or(path);
    let path = path.replace(['\\', ':'], "/");
    PathBuf::from(path)
}

#[cfg(unix)]
fn host_to_windows_path(path: &Path) -> String {
    let path_string = path.to_string_lossy().to_string().replace('/', "\\");
    if path.is_absolute() {
        format!("Z:{}", path_string)
    } else {
        path_string
    }
}

#[cfg(windows)]
fn windows_to_host_path(path: &str) -> PathBuf {
    PathBuf::from(path)
}

#[cfg(windows)]
fn host_to_windows_path(path: &Path) -> String {
    path.to_string_lossy()
        .trim_start_matches("\\\\?\\")
        .into_owned()
}
