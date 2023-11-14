use std::{process::exit, sync::Arc};

use nalgebra::Matrix4;
use uuid::Uuid;
use wgpu::TextureView;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::Window,
};

use crate::{RenderPassExecutor, RenderingContext, Scene, SceneObject, WindowDimensions};

pub struct Renderer {
    rendering_context: Arc<RenderingContext>,
    event_loop: EventLoop<()>,
    window: Window,
    scene: Scene,
}

impl Renderer {
    pub fn new(window_dimensions: WindowDimensions) -> Self {
        let event_loop = EventLoop::new();
        let window = winit::window::WindowBuilder::new()
            .with_decorations(true)
            .with_resizable(true)
            .with_transparent(false)
            .with_title("egui-wgpu_winit example")
            .with_inner_size(winit::dpi::PhysicalSize {
                width: window_dimensions.width,
                height: window_dimensions.height,
            })
            .build(&event_loop)
            .unwrap();

        let rendering_context = RenderingContext::new(Some(&window));

        Self {
            scene: Scene::new(rendering_context.clone(), window_dimensions),
            rendering_context,
            event_loop,
            window,
        }
    }

    pub fn add_object(&mut self, obj: Arc<SceneObject>) {
        self.scene.add_object(obj);
    }

    /// TEMP FUNCTION NEEDS TO BE REMOVED ASAP
    pub fn rendering_context(&self) -> Arc<RenderingContext> {
        self.rendering_context.clone()
    }

    pub fn run(self) {
        self.event_loop.run(move |event, _, _| {
            match event {
                Event::RedrawRequested(_) => {
                    let frame = self.rendering_context.current_frame();
                    let view = frame
                        .texture
                        .create_view(&wgpu::TextureViewDescriptor::default());

                    let mut executor = RenderPassExecutor::new(self.rendering_context.clone());

                    self.scene.draw(&mut executor, &view);

                    executor.submit(frame);
                }

                _ => {}
            }

            if let Event::WindowEvent {
                window_id: _,
                event,
            } = event
            {
                match event {
                    WindowEvent::Resized(new_size) => {
                        self.rendering_context
                            .recreate_swapchain(WindowDimensions::new(
                                new_size.width,
                                new_size.height,
                            ));
                        self.window.request_redraw();
                    }
                    WindowEvent::CloseRequested => exit(0),
                    _ => {}
                };
            }
        })
    }
}

pub trait RenderableObject {
    fn get_uuid(&self) -> Uuid;
    fn draw(
        &self,
        rp_exec: &mut RenderPassExecutor,
        view: &TextureView,
        mvp: Option<Matrix4<f32>>,
        ctx: Arc<RenderingContext>,
    );
}
