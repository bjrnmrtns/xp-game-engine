mod assets;
mod entity;
mod renderer;
mod terrain;

use crate::{
    assets::Assets,
    entity::Entity,
    renderer::{Mesh, Plane, Shape},
    terrain::Terrain,
};
use nalgebra_glm::{identity, vec3};
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

    let projection = nalgebra_glm::perspective(
        renderer.swap_chain_descriptor.width as f32 / renderer.swap_chain_descriptor.height as f32,
        45.0,
        0.1,
        1000.0,
    );
    let view = nalgebra_glm::look_at(
        &vec3(60.0, 10.0, 60.0),
        &vec3(0.0, 0.0, 0.0),
        &vec3(0.0, 1.0, 0.0),
    );
    let world_light_position = [60.0, 10.0, 60.0, 1.0];
    let light_color = [1.0, 1.0, 1.0, 1.0];
    let mut meshes = Assets::new();
    let terrain = Entity {
        mesh_handle: meshes.add(Mesh::from_shape(
            &renderer,
            Shape::from(Plane::new(100.0, 6, Box::new(Terrain::new()))),
        )),
        model: identity(),
    };

    event_loop.run(move |event, _, control_flow| {
        *control_flow = winit::event_loop::ControlFlow::Poll;
        match event {
            Event::RedrawRequested(_) => {
                pipeline.render(
                    &terrain,
                    &meshes,
                    projection,
                    view,
                    world_light_position,
                    light_color,
                    &mut renderer,
                );
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
