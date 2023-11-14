use std::sync::Arc;

use wgpu::{
    BindGroup, Buffer, CommandEncoder, RenderPass, RenderPipeline, SurfaceTexture, TextureView,
};

use crate::rendering_context::RenderingContext;

pub struct RenderPassExecutor {
    encoder: CommandEncoder,
    ctx: Arc<RenderingContext>,
}

impl RenderPassExecutor {
    pub fn new(ctx: Arc<RenderingContext>) -> Self {
        let encoder = ctx
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        Self { ctx, encoder }
    }

    pub fn queue_object(
        &mut self,
        pipeline: &RenderPipeline,
        vertex_buffer: &Buffer,
        index_buffer: &Buffer,
        bind_group: &BindGroup,
        view: &TextureView,
    ) {
        let mut render_pass: RenderPass<'_> =
            self.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
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

        render_pass.set_pipeline(pipeline);
        render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.set_bind_group(0, bind_group, &[]);
        render_pass.draw_indexed(0..3, 0, 0..1);
    }

    pub fn submit(self, frame: SurfaceTexture) {
        self.ctx.queue.submit(Some(self.encoder.finish()));
        frame.present();
    }
}
