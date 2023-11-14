use nalgebra::{Isometry3, Matrix4, Perspective3, Point3, Vector3};

use crate::WindowDimensions;

pub struct Camera {
    pub projection: Matrix4<f32>, //remove these pubs
    pub view: Matrix4<f32>,
}

impl Camera {
    pub fn with_perpective(
        window_dimensions: WindowDimensions,
        position: Point3<f32>,
        look_at: Point3<f32>,
    ) -> Self {
        let up = Vector3::new(0.0, 1.0, 0.0);
        let view = Isometry3::look_at_lh(&position, &look_at, &up).to_homogeneous();

        Self {
            projection: Perspective3::new(
                window_dimensions.width as f32 / window_dimensions.height as f32,
                45.0,
                0.01,
                1000.0,
            )
            .as_matrix()
            .clone(),

            view,
        }
    }

    pub fn calculate_mvp(&self, model: Matrix4<f32>) -> Matrix4<f32> {
        self.projection * self.view * model
    }
}
