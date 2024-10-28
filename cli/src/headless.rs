pub struct Window {}
impl win32::Window for Window {
    fn set_title(&self, _title: &str) {}
    fn set_size(&self, _width: u32, _height: u32) {}
    fn fullscreen(&self) {}
}

pub struct Surface {}
impl win32::Surface for Surface {
    fn write_pixels(&self, _pixels: &[u8]) {}
    fn show(&self) {}
    fn bit_blt(&self, _dst_rect: &win32::RECT, _src: &dyn win32::Surface, _src_rect: &win32::RECT) {
    }
}

pub struct Audio {}
impl win32::Audio for Audio {
    fn write(&mut self, _buf: &[u8]) {}
}

pub struct GUI {
    start: std::time::Instant,
}

impl GUI {
    pub fn new() -> anyhow::Result<Self> {
        Ok(GUI {
            start: std::time::Instant::now(),
        })
    }

    pub fn time(&self) -> u32 {
        std::time::Instant::now()
            .duration_since(self.start)
            .as_millis() as u32
    }

    pub fn get_message(&mut self) -> Option<win32::Message> {
        None
    }

    pub fn block(&mut self, wait: Option<u32>) -> bool {
        if let Some(wait) = wait {
            let when = self.start + std::time::Duration::from_millis(wait as u64);
            if let Some(remaining) = when.checked_duration_since(std::time::Instant::now()) {
                std::thread::sleep(remaining);
            }
            true
        } else {
            unimplemented!();
        }
    }

    pub fn create_window(&mut self, _hwnd: u32) -> Box<dyn win32::Window> {
        Box::new(Window {})
    }

    pub fn create_surface(&mut self, _opts: &win32::SurfaceOptions) -> Box<dyn win32::Surface> {
        Box::new(Surface {})
    }

    pub fn init_audio(&mut self, _sample_rate: u32) -> Box<dyn win32::Audio> {
        Box::new(Audio {})
    }
}
