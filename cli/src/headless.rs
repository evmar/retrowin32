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
        unimplemented!();
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
        unimplemented!();
    }

    pub fn create_surface(&mut self, _opts: &win32::SurfaceOptions) -> Box<dyn win32::Surface> {
        unimplemented!();
    }
}
