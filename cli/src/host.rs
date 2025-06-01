#[cfg(not(feature = "sdl"))]
use crate::headless::GUI;
#[cfg(feature = "sdl")]
use crate::sdl::GUI;
use std::{
    cell::{RefCell, RefMut},
    io::Write,
};

#[derive(Default)]
pub struct Env {
    gui: RefCell<Option<GUI>>,
}

impl Env {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn ensure_gui(&self) -> RefMut<GUI> {
        let mut gui = self.gui.borrow_mut();
        if gui.is_none() {
            *gui = Some(GUI::new().unwrap());
        }
        RefMut::map(gui, |gui| gui.as_mut().unwrap())
    }
}

impl win32::host::Host for Env {
    fn ticks(&self) -> u32 {
        self.ensure_gui().time()
    }

    fn system_time(&self) -> chrono::DateTime<chrono::Local> {
        chrono::Local::now()
    }

    fn get_message(&self) -> Option<win32::host::Message> {
        self.ensure_gui().get_message()
    }

    fn block(&self, wait: Option<u32>) -> bool {
        self.ensure_gui().block(wait)
    }

    fn stdout(&self, buf: &[u8]) {
        std::io::stdout().lock().write_all(buf).unwrap();
    }

    fn create_window(&self, hwnd: u32) -> Box<dyn win32::host::Window> {
        self.ensure_gui().create_window(hwnd)
    }

    fn create_surface(
        &self,
        _hwnd: u32,
        opts: &win32::host::SurfaceOptions,
    ) -> Box<dyn win32::host::Surface> {
        self.ensure_gui().create_surface(opts)
    }

    fn init_audio(
        &self,
        sample_rate: u32,
        callback: win32::host::AudioCallback,
    ) -> Box<dyn win32::host::Audio> {
        self.ensure_gui().init_audio(sample_rate, callback)
    }
}

pub fn new_host() -> Env {
    Env::new()
}
