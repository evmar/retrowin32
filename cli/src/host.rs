#[cfg(not(feature = "sdl"))]
use crate::headless::GUI;
#[cfg(feature = "sdl")]
use crate::sdl::GUI;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use std::{cell::RefCell, io::Write, rc::Rc};
use typed_path::{UnixPath, WindowsPath, WindowsPathBuf};
use win32::winapi::types::io_error_to_win32;
use win32::{FileOptions, ReadDir, Stat};

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

    fn set_len(&self, len: u64) -> Result<(), u32> {
        self.f.set_len(len).map_err(|e| io_error_to_win32(&e))
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

struct ReadDirIter {
    iter: std::fs::ReadDir,
}

impl ReadDir for ReadDirIter {
    fn next(&mut self) -> Result<Option<win32::ReadDirEntry>, u32> {
        match self.iter.next() {
            Some(Ok(entry)) => {
                let name = entry
                    .path()
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .into_owned();
                let meta = entry.metadata().unwrap();
                Ok(Some(win32::ReadDirEntry {
                    name,
                    stat: metadata_to_stat(&meta),
                }))
            }
            Some(Err(ref e)) => Err(io_error_to_win32(e)),
            None => Ok(None),
        }
    }
}

struct ReadDirFile {
    file: win32::ReadDirEntry,
    consumed: bool,
}

impl ReadDir for ReadDirFile {
    fn next(&mut self) -> Result<Option<win32::ReadDirEntry>, u32> {
        if self.consumed {
            Ok(None)
        } else {
            self.consumed = true;
            Ok(Some(self.file.clone()))
        }
    }
}

pub struct Env {
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

    pub fn exit_code(&self) -> Option<u32> {
        self.exit_code
    }
}

#[derive(Clone)]
pub struct EnvRef(pub Rc<RefCell<Env>>);

impl win32::Host for EnvRef {
    fn exit(&self, code: u32) {
        self.0.borrow_mut().exit_code = Some(code);
    }

    fn ticks(&self) -> u32 {
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

    fn current_dir(&self) -> Result<WindowsPathBuf, u32> {
        let path = std::env::current_dir().map_err(|e| io_error_to_win32(&e))?;
        Ok(host_to_windows_path(&path))
    }

    fn open(&self, path: &WindowsPath, options: FileOptions) -> Result<Box<dyn win32::File>, u32> {
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

    fn stat(&self, path: &WindowsPath) -> Result<Stat, u32> {
        let path = windows_to_host_path(path);
        match std::fs::metadata(path) {
            Ok(ref meta) => Ok(metadata_to_stat(meta)),
            Err(ref e) => Err(io_error_to_win32(e)),
        }
    }

    fn read_dir(&self, path: &WindowsPath) -> Result<Box<dyn ReadDir>, u32> {
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
                    Ok(Box::new(ReadDirIter { iter }))
                } else {
                    let filename = full_path
                        .file_name()
                        .unwrap()
                        .to_string_lossy()
                        .into_owned();
                    let file = win32::ReadDirEntry {
                        name: filename,
                        stat: metadata_to_stat(&meta),
                    };
                    Ok(Box::new(ReadDirFile {
                        file,
                        consumed: false,
                    }))
                }
            }
            Err(ref e) => Err(io_error_to_win32(e)),
        }
    }

    fn create_dir(&self, path: &WindowsPath) -> Result<(), u32> {
        let path = windows_to_host_path(path);
        std::fs::create_dir(path).map_err(|e| io_error_to_win32(&e))
    }

    fn remove_file(&self, path: &WindowsPath) -> Result<(), u32> {
        let path = windows_to_host_path(path);
        std::fs::remove_file(path).map_err(|e| io_error_to_win32(&e))
    }

    fn remove_dir(&self, path: &WindowsPath) -> Result<(), u32> {
        let path = windows_to_host_path(path);
        std::fs::remove_dir(path).map_err(|e| io_error_to_win32(&e))
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

pub fn new_host() -> EnvRef {
    EnvRef(Rc::new(RefCell::new(Env::new())))
}

/// Convert a `SystemTime` to nanoseconds relative to the Unix epoch.
fn system_time_to_nanos(t: SystemTime) -> i64 {
    match t.duration_since(UNIX_EPOCH) {
        Ok(d) => d.as_nanos() as i64,
        Err(e) => -(e.duration().as_nanos() as i64),
    }
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
        atime: meta.accessed().map_or(0, system_time_to_nanos),
        ctime: meta.created().map_or(0, system_time_to_nanos),
        mtime: meta.modified().map_or(0, system_time_to_nanos),
    }
}

#[cfg(unix)]
fn windows_to_host_path(mut path: &WindowsPath) -> PathBuf {
    path = path
        .strip_prefix("\\\\?\\")
        .or_else(|_| path.strip_prefix("\\\\.\\"))
        .unwrap_or(path);
    path = path
        .strip_prefix("Z:")
        .or_else(|_| path.strip_prefix("z:"))
        .unwrap_or(path);
    let unix = path.with_unix_encoding();
    PathBuf::from(unix.as_path())
}

#[cfg(unix)]
fn host_to_windows_path(path: &Path) -> WindowsPathBuf {
    let unix_path = UnixPath::new(path.as_os_str().as_encoded_bytes());
    let windows_path = unix_path.with_windows_encoding();
    if unix_path.is_absolute() {
        WindowsPath::new("Z:").join(windows_path)
    } else {
        windows_path
    }
}

#[cfg(windows)]
#[inline]
fn windows_to_host_path(path: &WindowsPath) -> PathBuf {
    PathBuf::from(path)
}

#[cfg(windows)]
#[inline]
fn host_to_windows_path(path: &Path) -> WindowsPathBuf {
    WindowsPathBuf::from(path.as_os_str().as_encoded_bytes())
}
