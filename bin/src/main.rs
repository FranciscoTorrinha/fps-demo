use renderer::{Renderer, TrianglePrimitive, WindowDimensions};

fn main() {
    let mut renderer = Renderer::new(WindowDimensions::new(800, 600));
    let triangle = TrianglePrimitive::new(renderer.rendering_context());

    renderer.add_object(triangle);

    renderer.run();
}
