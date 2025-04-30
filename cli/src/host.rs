#[cfg(not(feature = "sdl"))]
use crate::headless::GUI;
#[cfg(feature = "sdl")]
use crate::sdl::GUI;
use std::{
    cell::RefCell,
    io::Write,
    path::{Path, PathBuf},
    rc::Rc,
    time::{SystemTime, UNIX_EPOCH},
};
use win32::host::ERROR;
use win32::{UnixPath, WindowsPath, WindowsPathBuf};

struct File {
    f: std::fs::File,
}

impl win32::host::File for File {
    fn stat(&self) -> Result<win32::host::Stat, ERROR> {
        let meta = self.f.metadata()?;
        Ok(metadata_to_stat(&meta))
    }

    fn set_len(&self, len: u64) -> Result<(), ERROR> {
        self.f.set_len(len)?;
        Ok(())
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

impl win32::host::ReadDir for ReadDirIter {
    fn next(&mut self) -> Result<Option<win32::host::ReadDirEntry>, ERROR> {
        match self.iter.next() {
            Some(entry) => {
                let entry = entry?;
                let name = entry
                    .path()
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .into_owned();
                let meta = entry.metadata().unwrap();
                Ok(Some(win32::host::ReadDirEntry {
                    name,
                    stat: metadata_to_stat(&meta),
                }))
            }
            None => Ok(None),
        }
    }
}

struct ReadDirFile {
    file: win32::host::ReadDirEntry,
    consumed: bool,
}

impl win32::host::ReadDir for ReadDirFile {
    fn next(&mut self) -> Result<Option<win32::host::ReadDirEntry>, ERROR> {
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
}

impl Env {
    pub fn new() -> Self {
        Env { gui: None }
    }

    pub fn ensure_gui(&mut self) -> anyhow::Result<&mut GUI> {
        if self.gui.is_none() {
            self.gui = Some(GUI::new()?);
        }
        Ok(self.gui.as_mut().unwrap())
    }
}

#[derive(Clone)]
pub struct EnvRef(pub Rc<RefCell<Env>>);

impl win32::host::FileSystem for EnvRef {
    fn current_dir(&self) -> Result<WindowsPathBuf, ERROR> {
        let path = std::env::current_dir()?;
        Ok(host_to_windows_path(&path))
    }

    fn open(
        &self,
        path: &WindowsPath,
        options: win32::host::FileOptions,
    ) -> Result<Box<dyn win32::host::File>, ERROR> {
        let path = windows_to_host_path(path);
        log::debug!("open({path:?}, {options:?})");
        let f = std::fs::File::options()
            .read(options.read)
            .write(options.write)
            .truncate(options.truncate)
            .create(options.create)
            .create_new(options.create_new)
            .open(path)?;
        Ok(Box::new(File { f }))
    }

    fn stat(&self, path: &WindowsPath) -> Result<win32::host::Stat, ERROR> {
        let path = windows_to_host_path(path);
        let meta = std::fs::metadata(path)?;
        Ok(metadata_to_stat(&meta))
    }

    fn read_dir(&self, path: &WindowsPath) -> Result<Box<dyn win32::host::ReadDir>, ERROR> {
        let path = windows_to_host_path(path);
        let full_path = std::fs::canonicalize(path)?;
        let meta = std::fs::metadata(&full_path)?;
        if meta.is_dir() {
            let iter = std::fs::read_dir(&full_path)?;
            Ok(Box::new(ReadDirIter { iter }))
        } else {
            let filename = full_path
                .file_name()
                .unwrap()
                .to_string_lossy()
                .into_owned();
            let file = win32::host::ReadDirEntry {
                name: filename,
                stat: metadata_to_stat(&meta),
            };
            Ok(Box::new(ReadDirFile {
                file,
                consumed: false,
            }))
        }
    }

    fn create_dir(&self, path: &WindowsPath) -> Result<(), ERROR> {
        let path = windows_to_host_path(path);
        std::fs::create_dir(path)?;
        Ok(())
    }

    fn remove_file(&self, path: &WindowsPath) -> Result<(), ERROR> {
        let path = windows_to_host_path(path);
        std::fs::remove_file(path)?;
        Ok(())
    }

    fn remove_dir(&self, path: &WindowsPath) -> Result<(), ERROR> {
        let path = windows_to_host_path(path);
        std::fs::remove_dir(path)?;
        Ok(())
    }
}

impl win32::host::Host for EnvRef {
    fn ticks(&self) -> u32 {
        let mut env = self.0.borrow_mut();
        let gui = env.ensure_gui().unwrap();
        gui.time()
    }

    fn system_time(&self) -> chrono::DateTime<chrono::Local> {
        chrono::Local::now()
    }

    fn get_message(&self) -> Option<win32::host::Message> {
        let mut env = self.0.borrow_mut();
        let gui = env.gui.as_mut().unwrap();
        gui.get_message()
    }

    fn block(&self, wait: Option<u32>) -> bool {
        let mut env = self.0.borrow_mut();
        let gui = env.gui.as_mut().unwrap();
        gui.block(wait)
    }

    fn stdout(&self, buf: &[u8]) {
        std::io::stdout().lock().write_all(buf).unwrap();
    }

    fn create_window(&mut self, hwnd: u32) -> Box<dyn win32::host::Window> {
        let mut env = self.0.borrow_mut();
        let gui = env.ensure_gui().unwrap();
        gui.create_window(hwnd)
    }

    fn create_surface(
        &mut self,
        _hwnd: u32,
        opts: &win32::host::SurfaceOptions,
    ) -> Box<dyn win32::host::Surface> {
        let mut env = self.0.borrow_mut();
        let gui = env.ensure_gui().unwrap();
        gui.create_surface(opts)
    }

    fn init_audio(
        &mut self,
        sample_rate: u32,
        callback: win32::host::AudioCallback,
    ) -> Box<dyn win32::host::Audio> {
        let mut env = self.0.borrow_mut();
        let gui = env.ensure_gui().unwrap();
        gui.init_audio(sample_rate, callback)
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

fn metadata_to_stat(meta: &std::fs::Metadata) -> win32::host::Stat {
    let kind = if meta.is_dir() {
        win32::host::StatKind::Directory
    } else if meta.is_file() {
        win32::host::StatKind::File
    } else {
        win32::host::StatKind::Symlink
    };
    win32::host::Stat {
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
