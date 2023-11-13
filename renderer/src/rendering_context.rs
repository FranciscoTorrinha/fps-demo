use std::sync::Arc;

use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    Adapter, Buffer, BufferUsages, Device, Instance, Queue, Surface, SurfaceCapabilities,
};
use winit::window::Window;

use crate::common::{ImplVertex, MemoryLayoutable, WindowDimensions};

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
    _instance: Instance,
    pub device: Device,
    pub queue: Queue,
    adapter: Adapter,
    pub surface: Option<Surface>,
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

        if let Some(surf) = surface.as_ref() {
            Self::create_swapchain(&device, &adapter, &surf, WindowDimensions::default());
        }

        Arc::new(RenderingContext {
            device,
            _instance: instance,
            queue,
            adapter,
            surface,
        })
    }

    pub fn create_vertex_buffer<I>(&self, data: I) -> Buffer
    where
        I: Iterator + Clone,
        I::Item: ImplVertex,
    {
        self.device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: &data.layout_memory(),
            usage: BufferUsages::VERTEX,
        })
    }

    pub fn create_index_buffer(&self, data: &[u16]) -> Buffer {
        self.device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(data),
            usage: BufferUsages::INDEX,
        })
    }

    pub fn current_frame(&self) -> wgpu::SurfaceTexture {
        assert!(self.surface.is_some());
        self.surface
            .as_ref()
            .unwrap()
            .get_current_texture()
            .unwrap()
    }

    pub fn surface_capabilities(&self) -> SurfaceCapabilities {
        assert!(self.surface.is_some());
        self.surface
            .as_ref()
            .unwrap()
            .get_capabilities(&self.adapter)
    }

    fn create_swapchain(
        device: &Device,
        adapter: &Adapter,
        surface: &Surface,
        window_dimension: WindowDimensions,
    ) {
        let swapchain_capabilities = surface.get_capabilities(&adapter);
        let swapchain_format = swapchain_capabilities.formats[0];

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: swapchain_format,
            width: window_dimension.width,
            height: window_dimension.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: swapchain_capabilities.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(&device, &config);
    }

    pub fn recreate_swapchain(&self, wd: WindowDimensions) {
        assert!(self.surface.is_some());
        Self::create_swapchain(
            &self.device,
            &self.adapter,
            &self.surface.as_ref().unwrap(),
            wd,
        )
    }
}

#[cfg(test)]
mod test {
    use crate::common::GenericVertex;

    use super::RenderingContext;

    #[test]
    fn create_vertex_buffer() {
        let ctx = RenderingContext::new(None);
        let buffer = vec![GenericVertex::default(), GenericVertex::default()];
        ctx.create_vertex_buffer(buffer.into_iter());
    }
}
