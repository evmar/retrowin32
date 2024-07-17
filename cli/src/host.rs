#[cfg(not(feature = "sdl"))]
use crate::headless::GUI;
#[cfg(feature = "sdl")]
use crate::sdl::GUI;
use std::io::ErrorKind;
use std::{cell::RefCell, io::Write, rc::Rc};
use win32::winapi::types::{
    ERROR_ACCESS_DENIED, ERROR_FILE_EXISTS, ERROR_FILE_NOT_FOUND, ERROR_INVALID_ACCESS,
    ERROR_OPEN_FAILED,
};
use win32::{FileOptions, Stat};

struct File {
    f: std::fs::File,
}

impl win32::File for File {
    fn info(&self) -> u32 {
        let meta = self.f.metadata().unwrap();
        meta.len() as u32
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
        match std::fs::canonicalize(path) {
            Ok(p) => Ok(p.to_string_lossy().into_owned()),
            Err(e) => Err(match e.kind() {
                ErrorKind::NotFound => ERROR_FILE_NOT_FOUND,
                ErrorKind::PermissionDenied => ERROR_ACCESS_DENIED,
                _ => {
                    log::warn!("canonicalize({:?}): {:?}", path, e);
                    ERROR_OPEN_FAILED
                }
            }),
        }
    }

    fn open(&self, path: &str, options: FileOptions) -> Result<Box<dyn win32::File>, u32> {
        let result = std::fs::File::options()
            .read(options.read)
            .write(options.write)
            .truncate(options.truncate)
            .create(options.create)
            .create_new(options.create_new)
            .open(path);
        match result {
            Ok(f) => Ok(Box::new(File { f })),
            Err(e) => Err(match e.kind() {
                ErrorKind::NotFound => ERROR_FILE_NOT_FOUND,
                ErrorKind::PermissionDenied => ERROR_ACCESS_DENIED,
                ErrorKind::AlreadyExists => ERROR_FILE_EXISTS,
                ErrorKind::InvalidInput => ERROR_INVALID_ACCESS,
                _ => {
                    log::warn!("open({:?}): {:?}", path, e);
                    ERROR_OPEN_FAILED
                }
            }),
        }
    }

    fn stat(&self, path: &str) -> Result<Stat, u32> {
        match std::fs::metadata(path) {
            Ok(meta) => Ok(Stat {
                kind: if meta.is_dir() {
                    win32::StatKind::Directory
                } else if meta.is_file() {
                    win32::StatKind::File
                } else {
                    win32::StatKind::Symlink
                },
                size: meta.len() as u32,
            }),
            Err(e) => Err(match e.kind() {
                ErrorKind::NotFound => ERROR_FILE_NOT_FOUND,
                ErrorKind::PermissionDenied => ERROR_ACCESS_DENIED,
                _ => {
                    log::warn!("stat({:?}): {:?}", path, e);
                    ERROR_OPEN_FAILED
                }
            }),
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
