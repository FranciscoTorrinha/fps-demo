#[derive(Debug, Clone, Copy)]
pub struct WindowDimensions {
    pub width: u32,
    pub height: u32,
}

impl Default for WindowDimensions {
    fn default() -> Self {
        Self {
            width: 800,
            height: 600,
        }
    }
}

impl WindowDimensions {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}
