use std::mem::size_of;



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

/**
 * This is a generic implementation for a vertex, it should be able to fulfill most purposes for game dev,
 * however if a custom vertex type is requires use the [ImplVertex] trait
 */

#[derive(Debug, Default, Clone, Copy)]
pub struct GenericVertex {
    _position: [f32; 4],
    _texture: [f32; 2],
    _normal: [f32; 4],
}

pub trait ImplVertex {
    fn size(&self) -> usize;
}

impl ImplVertex for GenericVertex {
    fn size(&self) -> usize {
        size_of::<Self>()
    }
}