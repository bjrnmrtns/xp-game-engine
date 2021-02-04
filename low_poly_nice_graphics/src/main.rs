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
    let world_camera_position = [60.0, 5.0, 60.0, 1.0];
    let view = nalgebra_glm::look_at(
        &vec3(
            world_camera_position[0],
            world_camera_position[1],
            world_camera_position[2],
        ),
        &vec3(0.0, 0.0, 0.0),
        &vec3(0.0, 1.0, 0.0),
    );
    let mut meshes = Assets::new();
    let mut terrain = Entity {
        mesh_handle: meshes.add(Mesh::from_shape(
            &renderer,
            Shape::from(Plane::new(100.0, 6, Box::new(Terrain::new()))),
        )),
        model: identity(),
    };

    let start_time = std::time::Instant::now();
    event_loop.run(move |event, _, control_flow| {
        *control_flow = winit::event_loop::ControlFlow::Poll;
        match event {
            Event::RedrawRequested(_) => {
                let time_since_start_secs = (std::time::Instant::now() - start_time).as_secs_f32();
                let light_position_value = time_since_start_secs.sin() * 50.0;
                let model_rotation_y = 0.0; //time_since_start_secs;
                terrain.model = nalgebra_glm::rotate_y(&identity(), model_rotation_y);
                pipeline.render(
                    &terrain,
                    &meshes,
                    projection,
                    view,
                    [light_position_value, 10.0, light_position_value, 1.0],
                    world_camera_position,
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
