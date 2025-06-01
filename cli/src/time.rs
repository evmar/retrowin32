//! Implementation of the `ticks()` method of win32::host trait.

#[derive(Clone)] // shared with SDL
pub struct Time {
    pub start: std::time::Instant,
}

impl Time {
    pub fn new() -> Self {
        Time {
            start: std::time::Instant::now(),
        }
    }

    pub fn ticks(&self) -> u32 {
        self.start.elapsed().as_millis() as u32
    }
}
