use std::mem::size_of;

use bytemuck::{bytes_of, Pod, Zeroable};
use nalgebra::Matrix4;

/**
 * This is a generic implementation for a vertex, it should be able to fulfill most purposes for game dev,
 * however if a custom vertex type is requires use the [ImplVertex] trait
 */

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, Pod, Zeroable)]
pub struct GenericVertex {
    pub position: [f32; 4],
    pub texture: [f32; 2],
    pub normal: [f32; 4],
}

impl ImplVertex for GenericVertex {
    fn size(&self) -> usize {
        size_of::<Self>()
    }

    fn description(&self) -> wgpu::VertexBufferLayout<'static> {
        Self::description()
    }

    fn raw(&self) -> Vec<u8> {
        bytes_of(self).to_vec()
    }
}
impl GenericVertex {
    pub fn description() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<GenericVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                // Position
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x4,
                },
                // Texture
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
                // Normal
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 6]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MVPBuff {
    pub matrix: Matrix4<f32>,
}

impl MVPBuff {
    pub fn from_mat4(mat: Matrix4<f32>) -> Self {
        Self { matrix: mat }
    }

    pub fn description() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Matrix4<f32>>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: 4,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: 8,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: 12,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}

impl ImplVertex for MVPBuff {
    fn size(&self) -> usize {
        size_of::<f32>() * 16
    }

    fn description(&self) -> wgpu::VertexBufferLayout<'static> {
        Self::description()
    }

    fn raw(&self) -> Vec<u8> {
        bytes_of(&self.matrix.data.0).to_vec()
    }
}

pub trait ImplVertex {
    fn size(&self) -> usize;
    fn description(&self) -> wgpu::VertexBufferLayout<'static>;
    fn raw(&self) -> Vec<u8>;
}

pub trait MemoryLayoutable {
    fn layout_memory(&self) -> Vec<u8>;
}

impl<I> MemoryLayoutable for I
where
    I: Iterator + Clone,
    I::Item: ImplVertex,
{
    fn layout_memory(&self) -> Vec<u8> {
        self.clone().flat_map(|v| v.raw()).collect()
    }
}
