mod entity;
mod generators;
mod registry;
mod renderer;
mod static_camera;
mod transform;

use crate::{
    entity::Entity,
    registry::Registry,
    renderer::{
        BindGroup, Cube, DirectionalProperties, Light, LightBindGroup, Mesh, Plane,
        PointProperties, Shape, SpotProperties,
    },
    static_camera::StaticCamera,
    transform::Transform,
};
use glam::{Quat, Vec3};
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
    let pipeline_bindgroup = BindGroup::new(&renderer);
    let pipeline =
        futures::executor::block_on(renderer::Pipeline::new(&renderer, &pipeline_bindgroup))
            .expect("Could not create pipeline");
    let light_pipeline_bindgroup = LightBindGroup::new(&renderer);
    let pipeline_light = futures::executor::block_on(renderer::LightPipeline::new(
        &renderer,
        &light_pipeline_bindgroup,
    ))
    .expect("Could not create pipeline light");

    let mut camera = StaticCamera::new(
        Vec3::new(60.0, 50.0, 60.0),
        Vec3::new(0.0, 0.0, 0.0),
        renderer.swap_chain_descriptor.width as f32 / renderer.swap_chain_descriptor.height as f32,
    );

    let mut meshes = Registry::new();
    let mut lights = Registry::new();
    let mut entities = Registry::new();
    let light_mesh_handle = meshes.add(Mesh::from_shape(&renderer, Shape::from(Cube::new(1.0))));
    lights.add(Light::Directional(DirectionalProperties::new([
        -0.2, -1.0, -0.3, 1.0,
    ])));
    lights.add(Light::Spot(SpotProperties::new(
        [0.0, 7.0, 0.0, 1.0],
        [0.0, -1.0, 0.0, 1.0],
    )));
    lights.add(Light::Spot(SpotProperties::new(
        [30.0, 15.0, 0.0, 1.0],
        [0.0, -1.0, 0.0, 1.0],
    )));
    lights.add(Light::Point(PointProperties::new([30.0, 10.0, 30.0, 1.0])));
    lights.add(Light::Point(PointProperties::new([-30.0, 10.0, 30.0, 1.0])));

    let ground = entities.add(Entity {
        mesh_handle: meshes.add(Mesh::from_shape(
            &renderer,
            Shape::from(Plane::new(100.0, 5, Box::new(generators::SineCosine))),
            //Shape::from(Plane::new(100.0, 6, Box::new(generators::Zero))),
            //Shape::from(Cube::new(30.0)),
        )),
        transform: Transform::identity(),
    });
    entities.add(Entity {
        mesh_handle: meshes.add(Mesh::from_shape(&renderer, Shape::from(Cube::new(5.0)))),
        transform: Transform::identity(),
    });
    let start_time = std::time::Instant::now();
    event_loop.run(move |event, _, control_flow| {
        *control_flow = winit::event_loop::ControlFlow::Poll;
        match event {
            Event::RedrawRequested(_) => {
                let time_since_start_secs = (std::time::Instant::now() - start_time).as_secs_f32();
                let model_rotation_y = time_since_start_secs;
                entities.get_mut(ground.clone()).unwrap().transform.rotation =
                    Quat::from_rotation_y(model_rotation_y);

                let target = &renderer
                    .swap_chain
                    .get_current_frame()
                    .expect("Could not get next frame texture_view")
                    .output
                    .view;
                pipeline.render(
                    &entities,
                    &meshes,
                    &lights,
                    &pipeline_bindgroup,
                    &camera,
                    &mut renderer,
                    target,
                );
                pipeline_light.render(
                    &light_mesh_handle,
                    &meshes,
                    &lights,
                    &light_pipeline_bindgroup,
                    &camera,
                    &mut renderer,
                    target,
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
                    camera.set_aspect_ratio(size.width as f32 / size.height as f32);
                    futures::executor::block_on(renderer.resize(size.width, size.height));
                }
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    camera.set_aspect_ratio(
                        new_inner_size.width as f32 / new_inner_size.height as f32,
                    );
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
