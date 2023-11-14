use nalgebra::{Matrix4, Vector3};

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

#[derive(Debug, Clone, Copy, Default)]
pub struct Transfomation {
    translations: Vector3<f32>,
    rotation: Vector3<f32>,
    scale: f32,
}

impl Transfomation {
    pub fn new(translation: [f32; 3], rotation: [f32; 3], scale: f32) -> Self {
        Self {
            translations: Vector3::new(translation[0], translation[1], translation[2]),
            rotation: Vector3::new(rotation[0], rotation[1], rotation[2]),
            scale,
        }
    }

    pub fn as_matrix(&self) -> Matrix4<f32> {
        let rotation = Matrix4::new_rotation(self.rotation);
        let tranlation = Matrix4::new_translation(&self.translations);

        (tranlation * rotation).scale(self.scale)
    }
}
