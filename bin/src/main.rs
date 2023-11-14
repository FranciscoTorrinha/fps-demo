use renderer::{Renderer, SceneObject, Transfomation, TrianglePrimitive, WindowDimensions};

fn main() {
    let mut renderer = Renderer::new(WindowDimensions::new(800, 600));
    let triangle = TrianglePrimitive::new(renderer.rendering_context());

    let triangle_object = SceneObject::new(
        triangle,
        Transfomation::new([0.0, 0.0, 0.0], [0.0, 0.0, 0.0], 0.5),
        "Triangle",
    );

    renderer.add_object(triangle_object);

    renderer.run();
}
