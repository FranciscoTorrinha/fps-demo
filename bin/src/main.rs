use std::process::exit;

use renderer::{RenderingContext, RenderPassExecutor, WindowDimensions};
use winit::{event_loop::EventLoop, event::{Event, WindowEvent}};

fn main() {
    let event_loop = EventLoop::new();
    let window = winit::window::WindowBuilder::new()
        .with_decorations(true)
        .with_resizable(true)
        .with_transparent(false)
        .with_title("egui-wgpu_winit example")
        .with_inner_size(winit::dpi::PhysicalSize {
            width: 800,
            height: 600,
        })
        .build(&event_loop)
        .unwrap();

    let ctx = RenderingContext::new(Some(&window));

    event_loop.run(move |event, _, _| {
        match event {
            Event::RedrawRequested(_) => {
                let rp = RenderPassExecutor::new(ctx.clone());
                rp.submit();
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
                    ctx.recreate_swapchain(WindowDimensions::new(new_size.width, new_size.height));
                    window.request_redraw();
                }
                WindowEvent::CloseRequested => exit(0),
                _ => {}
            };
        }
    })
}
