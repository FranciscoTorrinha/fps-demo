use std::{cell::RefCell, sync::Arc};

use wgpu::{TextureView, SurfaceTexture, RenderPass};

use crate::rendering_context::RenderingContext;

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
