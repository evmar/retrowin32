pub struct GUI {}

impl GUI {
    pub fn new() -> anyhow::Result<Self> {
        Ok(GUI {})
    }
    pub fn get_message(&self, _wait: bool) -> Option<win32::Message> {
        unimplemented!();
    }
    pub fn create_window(&mut self) -> Box<dyn win32::Window> {
        unimplemented!();
    }
    pub fn create_surface(&mut self, _opts: &win32::SurfaceOptions) -> Box<dyn win32::Surface> {
        unimplemented!();
    }
}
