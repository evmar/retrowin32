pub struct GUI {}

impl GUI {
    pub fn new() -> anyhow::Result<Self> {
        Ok(GUI {})
    }

    pub fn time(&self) -> u32 {
        0
    }

    pub fn get_message(&mut self) -> Option<win32::Message> {
        unimplemented!();
    }

    pub fn block(&mut self, _wait: Option<u32>) -> bool {
        unimplemented!();
    }

    pub fn create_window(&mut self, _hwnd: u32) -> Box<dyn win32::Window> {
        unimplemented!();
    }

    pub fn create_surface(&mut self, _opts: &win32::SurfaceOptions) -> Box<dyn win32::Surface> {
        unimplemented!();
    }
}
