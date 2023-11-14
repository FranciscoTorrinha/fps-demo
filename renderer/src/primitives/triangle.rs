use crate::{
    renderer::RenderableObject,
    vertex::{GenericVertex, MVPBuff},
    RenderPassExecutor, RenderingContext,
};
use nalgebra::Matrix4;
use std::{borrow::Cow, sync::Arc};
use uuid::Uuid;
use wgpu::{BindGroupLayout, Buffer, RenderPipeline, TextureView};

#[derive(Debug)]
pub struct TrianglePrimitive {
    uuid: Uuid,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    pipeline: RenderPipeline,
    bind_group_layout: BindGroupLayout,
}

impl TrianglePrimitive {
    pub fn new(ctx: Arc<RenderingContext>) -> Box<Self> {
        assert!(ctx.surface.is_some());

        let vertecies = [
            GenericVertex {
                position: [0.0, 0.0, 0.0, 1.0],
                normal: [0.0, 0.0, 0.0, 0.0],
                texture: [0.0, 0.0],
            },
            GenericVertex {
                position: [-1.0, -1.0, 0.0, 1.0],
                normal: [0.0, 0.0, 0.0, 0.0],
                texture: [0.0, 0.0],
            },
            GenericVertex {
                position: [1.0, -1.0, 0.0, 1.0],
                normal: [0.0, 0.0, 0.0, 0.0],
                texture: [0.0, 0.0],
            },
        ];

        let indicies: [u16; 3] = [0, 1, 2];

        let vertex_buffer = ctx.create_vertex_buffer(vertecies.into_iter());
        let index_buffer = ctx.create_index_buffer(&indicies);

        let swapchain_format = ctx.surface_capabilities().formats[0];

        let bind_group_layout =
            ctx.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: None,
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: wgpu::BufferSize::new(64),
                        },
                        count: None,
                    }],
                });

        let pipeline_layout = ctx
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            });

        let shader = ctx
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!(
                    "shaders/shader.wgsl"
                ))),
            });

        let pipeline = ctx
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: None,
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[GenericVertex::description()],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[Some(swapchain_format.into())],
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList, // 1.
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw, // 2.
                    cull_mode: Some(wgpu::Face::Back),
                    // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                    polygon_mode: wgpu::PolygonMode::Fill,
                    // Requires Features::DEPTH_CLIP_CONTROL
                    unclipped_depth: false,
                    // Requires Features::CONSERVATIVE_RASTERIZATION
                    conservative: false,
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                multiview: None,
            });

        Box::new(Self {
            vertex_buffer,
            index_buffer,
            pipeline,
            bind_group_layout,
            uuid: Uuid::new_v4(),
        })
    }

    pub fn transform(&self) {}
}

impl<'rp> RenderableObject for TrianglePrimitive {
    fn get_uuid(&self) -> Uuid {
        self.uuid
    }

    fn draw(
        &self,
        rp_exec: &mut RenderPassExecutor,
        view: &TextureView,
        mvp: Option<Matrix4<f32>>,
        ctx: Arc<RenderingContext>,
    ) {
        let mvp_buff = match mvp {
            Some(m) => ctx.create_uniform_buffer(MVPBuff::from_mat4(m)),
            None => ctx.create_uniform_buffer(MVPBuff::from_mat4(Matrix4::identity())),
        };
        // let mvp_buff = ctx.create_uniform_buffer(MVPBuff::from_mat4(Matrix4::identity()));

        println!("MVP: {}", mvp.unwrap());

        let bind_group = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &self.bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: mvp_buff.as_entire_binding(),
            }],
            label: None,
        });

        rp_exec.queue_object(
            &self.pipeline,
            &self.vertex_buffer,
            &self.index_buffer,
            &bind_group,
            view,
        );
    }
}
