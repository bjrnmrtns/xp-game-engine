pub mod cameras;
pub mod controllers;
pub mod entity;
pub mod generators;
pub mod gltf;
pub mod input;
pub mod mesh;
mod physics;
pub mod registry;
pub mod renderer;
pub mod tiles;
pub mod transform;
mod voxel;
pub mod winit_impl;
mod world;

use crate::{
    cameras::FollowCamera,
    controllers::{CameraController, CharacterController},
    entity::Entity,
    input::{keyboard_state_from_events, InputAll},
    mesh::{Cube, IcoSphere, Mesh},
    physics::{Body, BodyStatus, CollisionShape, Cuboid, Physics, Sphere},
    registry::Registry,
    renderer::{BindGroup, DirectionalProperties, Light, LightBindGroup, PointProperties, SpotProperties},
    tiles::LoadError,
    transform::Transform,
};
use glam::Vec3;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

#[derive(Debug)]
pub enum GameError {
    TileLoadError(LoadError),
}

impl From<LoadError> for GameError {
    fn from(e: LoadError) -> GameError {
        GameError::TileLoadError(e)
    }
}

fn main() -> Result<(), GameError> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .build(&event_loop)
        .expect("Could not create window");
    let mut renderer =
        futures::executor::block_on(renderer::Renderer::new(&window)).expect("Could not create renderer");
    let pipeline_bindgroup = BindGroup::new(&renderer);
    let pipeline = futures::executor::block_on(renderer::Pipeline::new(&renderer, &pipeline_bindgroup))
        .expect("Could not create pipeline");
    let light_pipeline_bindgroup = LightBindGroup::new(&renderer);
    let pipeline_light =
        futures::executor::block_on(renderer::LightPipeline::new(&renderer, &light_pipeline_bindgroup))
            .expect("Could not create pipeline light");

    let mut physics = Physics::default();
    let mut meshes = Registry::new();
    let mut lights = Registry::new();
    let mut entities = Registry::new();
    let light_mesh_handle = meshes.add(Mesh::from(Cube::new(0.25)));
    lights.add(Light::Directional(DirectionalProperties::new([-1.0, -0.5, -1.0, 1.0])));

    lights.add(Light::Spot(SpotProperties::new(
        [0.0, 14.0, 0.0, 1.0],
        [0.0, -1.0, 0.0, 1.0],
    )));
    lights.add(Light::Spot(SpotProperties::new(
        [8.0, 14.0, 0.0, 1.0],
        [0.0, -1.0, 0.0, 1.0],
    )));
    lights.add(Light::Point(PointProperties::new([8.0, 14.0, 8.0, 1.0])));
    lights.add(Light::Point(PointProperties::new([-8.0, 14.0, 8.0, 1.0])));

    /*    let vox_data = std::fs::read("res/example.vox").unwrap();
        voxel::load_vox(&vox_data, |mesh| {
            let mesh = meshes.add(mesh);
            entities.add(Entity {
                mesh_handle: mesh.clone(),
                collision_shape: None,
                transform: Transform::default(),
            });
            mesh
        });
    */
    voxel::load_test_vox_files(|mesh| {
        let mesh = meshes.add(mesh);
        entities.add(Entity {
            mesh_handle: mesh.clone(),
            collision_shape: None,
            transform: Transform::default(),
        });
        mesh
    });

    let cube = entities.add(Entity {
        mesh_handle: meshes.add(Mesh::from(Cube::new(1.0))),
        collision_shape: Some(CollisionShape {
            body_status: BodyStatus::Static,
            body: Body::Cuboid(Cuboid {
                half_extent_x: 0.5,
                half_extent_y: 0.5,
                half_extent_z: 0.5,
            }),
        }),
        transform: Transform::from_translation(Vec3::new(0.0, 10.0, 0.0)),
    });

    physics.register(cube, &entities);

    let character = entities.add(Entity {
        mesh_handle: meshes.add(Mesh::from(IcoSphere::new(0.5))),
        collision_shape: Some(CollisionShape {
            body_status: BodyStatus::Dynamic,
            body: Body::Sphere(Sphere { radius: 0.5 }),
        }),
        transform: Transform::from_translation(Vec3::new(0.0, 10.0, 4.0)),
    });
    physics.register(character.clone(), &entities);
    physics.register_character(character.clone());

    let mut follow_camera = FollowCamera::new(
        entities.get(&character).unwrap().transform.clone(),
        renderer.swap_chain_descriptor.width as f32 / renderer.swap_chain_descriptor.height as f32,
    );

    let mut input_all = InputAll::default();
    let mut character_controller = CharacterController::default();
    let mut camera_controller = CameraController::default();
    let start_time = std::time::Instant::now();
    let mut steps_taken = 0;
    event_loop.run(move |event, _, control_flow| {
        *control_flow = winit::event_loop::ControlFlow::Poll;
        match event {
            Event::RedrawRequested(_) => {
                let steps_since_start = (std::time::Instant::now() - start_time).as_millis() * 60 / 1000;
                let steps = steps_since_start - steps_taken;
                keyboard_state_from_events(&input_all.keyboard_events, &mut input_all.keyboard_input);
                character_controller.keyboard(&input_all.keyboard_input);
                camera_controller.mouse_handling(&input_all.mouse_wheel_events, &input_all.mouse_motion_events);
                follow_camera.handle_camera_controller(&camera_controller);
                for _ in 0..steps {
                    physics.step(&mut entities, &character_controller);
                }
                steps_taken = steps_since_start;
                follow_camera.follow(entities.get(&character).unwrap().transform.clone());
                input_all.clear_events();

                let before_render = std::time::Instant::now();
                let target = &renderer
                    .swap_chain
                    .get_current_frame()
                    .expect("Could not get next frame texture_view")
                    .output
                    .view;

                pipeline.render(
                    &entities,
                    &mut meshes,
                    &lights,
                    &pipeline_bindgroup,
                    &follow_camera,
                    &mut renderer,
                    target,
                );
                let after_render = std::time::Instant::now();
                pipeline_light.render(
                    &light_mesh_handle,
                    &lights,
                    &light_pipeline_bindgroup,
                    &follow_camera,
                    &mut renderer,
                    target,
                );
                println!("render-time: {}", (after_render - before_render).as_millis());
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            Event::WindowEvent {
                event: ref window_event,
                window_id,
            } if window_id == window.id() => match window_event {
                WindowEvent::Resized(size) => {
                    follow_camera.set_aspect_ratio(size.width as f32 / size.height as f32);
                    futures::executor::block_on(renderer.resize(size.width, size.height));
                }
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    follow_camera.set_aspect_ratio(new_inner_size.width as f32 / new_inner_size.height as f32);
                    futures::executor::block_on(renderer.resize(new_inner_size.width, new_inner_size.height));
                }
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput { .. } => {
                    winit_impl::handle_input(&mut input_all, &event);
                }
                WindowEvent::MouseWheel { .. } => {
                    winit_impl::handle_input(&mut input_all, &event);
                }
                _ => (),
            },
            Event::DeviceEvent { .. } => winit_impl::handle_input(&mut input_all, &event),
            _ => (),
        }
    });
}
