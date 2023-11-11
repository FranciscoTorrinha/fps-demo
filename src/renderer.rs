use std::{mem::size_of, sync::Arc};

use wgpu::{Buffer, BufferDescriptor, BufferUsages, Device, Instance, Queue};
use winit::window::Window;

/**
   * Provides all necessary functionality to interact with WGPU, as of right now
   * the supported actions for [RenderingContext] are:
   * 
   * -> Vertex Buffer Creation

   * Only one instance of [RenderingContext] should exist during program execution,
   * the context is wrapped around and Arc an thus can be cloned as many times as necessary

   * At the momento there is no interior mutability in the [RenderingContext] and adding it should
   * be avoided at all cost.
*/
pub struct RenderingContext {
    instance: Instance,
    device: Device,
    queue: Queue,
}

impl RenderingContext {
    /**
     * In "regular" code window should be passed in as [Some(Window)] however, winit has terrible test support,
     * and thus, until a better solution if found the context should be initialized with a [None] window for test
     * purposes
    */
    pub fn new(window: Option<&Window>) -> Arc<Self> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
            ..Default::default()
        });

        let surface = if let Some(w) = window {
            Some(unsafe { instance.create_surface(w) }.unwrap())
        } else {
            None
        };

        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            // Request an adapter which can render to our surface
            compatible_surface: surface.as_ref(),
        }))
        .unwrap();

        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::empty(),
                // Make sure we use the texture resolution limits from the adapter, so we can support images the size of the swapchain.
                limits:
                    wgpu::Limits::downlevel_webgl2_defaults().using_resolution(adapter.limits()),
            },
            None,
        ))
        .unwrap();

        Arc::new(RenderingContext {
            device,
            instance,
            queue,
        })
    }

    pub fn create_vertex_buffer(&self, data: &[impl ImplVertex]) -> Option<Buffer> {
        let size = data.into_iter().map(|vertex| vertex.size() as u64).sum();

        self.device.create_buffer(&BufferDescriptor {
            label: None,
            size,
            usage: BufferUsages::VERTEX,
            mapped_at_creation: false,
        });
        None
    }
}


/**
 * This is a generic implementation for a vertex, it should be able to fulfill most purposes for game dev, 
 * however if a custom vertex type is requires use the [ImplVertex] trait
 */

#[derive(Debug, Default, Clone, Copy)]
pub struct GenericVertex {
    position: [f32; 4],
    texture: [f32; 2],
    normal: [f32; 4],
}

pub trait ImplVertex {
    fn size(&self) -> usize;
}

impl ImplVertex for GenericVertex {
    fn size(&self) -> usize {
        size_of::<Self>()
    }
}

#[cfg(test)]
mod test {
    use super::{GenericVertex, RenderingContext};

    #[test]
    fn create_vertex_buffer() {
        let ctx = RenderingContext::new(None);
        let buffer = vec![GenericVertex::default(), GenericVertex::default()];
        ctx.create_vertex_buffer(&buffer);
    }
}
