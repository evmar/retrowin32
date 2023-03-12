pub struct GUI {}

impl GUI {
    pub fn new() -> anyhow::Result<Self> {
        Ok(GUI {})
    }
    pub fn pump_messages(&mut self) -> bool {
        unimplemented!();
    }
    pub fn create_window(&mut self) -> Box<dyn win32::Window> {
        unimplemented!();
    }
    pub fn create_surface(&mut self, _opts: &win32::SurfaceOptions) -> Box<dyn win32::Surface> {
        unimplemented!();
    }
}
