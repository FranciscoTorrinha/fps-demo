use std::sync::Arc;

use nalgebra::{Matrix4, Point3};
use uuid::Uuid;
use wgpu::TextureView;

use crate::{
    common::Transfomation, renderer::RenderableObject, Camera, RenderPassExecutor,
    RenderingContext, WindowDimensions,
};

pub struct SceneObject {
    id: Uuid,
    label: String,
    renderable: Box<dyn RenderableObject>,
    model: Matrix4<f32>,
}

impl SceneObject {
    pub fn new(
        renderable: Box<dyn RenderableObject>,
        transformation: Transfomation,
        label: &str,
    ) -> Arc<Self> {
        Arc::new(Self {
            id: Uuid::new_v4(),
            label: String::from(label),
            renderable: renderable,
            model: transformation.as_matrix(),
        })
    }

    fn update(&self) {
        println!("Updating object: {}: {}", self.label, self.id)
    }

    fn draw(
        &self,
        camera: &Camera,
        executor: &mut RenderPassExecutor,
        view: &TextureView,
        ctx: Arc<RenderingContext>,
    ) {
        let mvp = camera.calculate_mvp(self.model);
        self.renderable.draw(executor, view, Some(mvp), ctx.clone());
    }
}

pub struct Scene {
    camera: Camera,
    objects: Vec<Arc<SceneObject>>,
    ctx: Arc<RenderingContext>,
}

impl Scene {
    pub fn new(ctx: Arc<RenderingContext>, window_dimensions: WindowDimensions) -> Self {
        Self {
            camera: Camera::with_perpective(
                window_dimensions,
                Point3::new(0.0, 0.0, -5.0),
                Point3::new(0.0, 0.0, 0.0),
            ),
            objects: vec![],
            ctx,
        }
    }

    pub fn add_object(&mut self, obj: Arc<SceneObject>) {
        self.objects.push(obj)
    }

    pub fn update(&self) {
        self.objects.iter().for_each(|o| o.update())
    }

    pub fn draw(&self, executor: &mut RenderPassExecutor, view: &TextureView) {
        self.objects
            .iter()
            .for_each(|o| o.draw(&self.camera, executor, view, self.ctx.clone()))
    }
}
