use std::{cell::RefCell, mem::size_of, sync::Arc};

use wgpu::{
    Adapter, Buffer, BufferDescriptor, BufferUsages, Device, Instance, Queue, RenderPass, Surface,
    TextureView, SurfaceTexture,
};
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
    _instance: Instance,
    pub device: Device,
    pub queue: Queue,
    adapter: Adapter,
    surface: Option<Surface>,
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

    pub fn current_frame(&self) -> wgpu::SurfaceTexture {
        assert!(self.surface.is_some());
        self.surface
            .as_ref()
            .unwrap()
            .get_current_texture()
            .unwrap()
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

pub struct RenderPassExecutor {
    objects: RefCell<Vec<Box<dyn RenderableObject>>>,
    ctx: Arc<RenderingContext>,
    view: TextureView,
    frame: SurfaceTexture
}

impl RenderPassExecutor {
    pub fn new(ctx: Arc<RenderingContext>) -> Self {
        let frame = ctx.current_frame();
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        Self {
            ctx,
            objects: RefCell::new(vec![]),
            view,
            frame
        }
    }

    pub fn queue_object(&self, object: Box<dyn RenderableObject>) {
        self.objects.borrow_mut().push(object);
    }

    pub fn submit(self) {
        let mut encoder = self
            .ctx
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            self.objects.borrow().iter().for_each(|obj| {
                obj.render(&mut rpass);
            })
        }

        self.ctx.queue.submit(Some(encoder.finish()));
        self.frame.present();
    }
}

pub trait RenderableObject {
    fn render(&self, rp: &mut RenderPass<'_>);
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
