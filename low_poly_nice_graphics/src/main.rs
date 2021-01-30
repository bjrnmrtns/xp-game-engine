mod assets;
mod entity;
mod renderer;

use crate::{
    assets::Assets,
    entity::Entity,
    renderer::{Mesh, SimpleTriangle},
};
use nalgebra_glm::identity;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .build(&event_loop)
        .expect("Could not create window");
    let mut renderer = futures::executor::block_on(renderer::Renderer::new(&window))
        .expect("Could not create renderer");
    let pipeline = futures::executor::block_on(renderer::Pipeline::new(&renderer))
        .expect("Could not create pipeline");

    let mut meshes = Assets::new();
    let triangle = Entity {
        mesh_handle: meshes.add(Mesh::from_simple_triangle(
            &renderer,
            &SimpleTriangle::default(),
        )),
        model: identity(),
    };

    event_loop.run(move |event, _, control_flow| {
        *control_flow = winit::event_loop::ControlFlow::Poll;
        match event {
            Event::RedrawRequested(_) => {
                pipeline.render(&triangle, &meshes, &mut renderer);
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => match event {
                WindowEvent::Resized(size) => {
                    futures::executor::block_on(renderer.resize(size.width, size.height));
                }
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    futures::executor::block_on(
                        renderer.resize(new_inner_size.width, new_inner_size.height),
                    );
                }
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            _ => (),
        }
    });
}
