pub mod cameras;
pub mod controllers;
pub mod entity;
pub mod generators;
pub mod gltf;
pub mod input;
pub mod mesh;
pub mod registry;
pub mod renderer;
pub mod tile;
pub mod transform;
pub mod winit_impl;
pub mod world;

use crate::{
    cameras::FollowCamera,
    controllers::{CameraController, CharacterController},
    entity::Entity,
    input::{keyboard_state_from_events, InputAll},
    mesh::{Cube, IcoSphere, Mesh},
    registry::Registry,
    renderer::{BindGroup, DirectionalProperties, Light, LightBindGroup, PointProperties, SpotProperties},
    tile::TileLoadError,
    transform::Transform,
    world::World,
};
use glam::{Quat, Vec3};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

#[derive(Debug)]
pub enum GameError {
    TileLoadError(TileLoadError),
}

impl From<TileLoadError> for GameError {
    fn from(e: TileLoadError) -> GameError {
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

    let mut meshes = Registry::new();
    let mut lights = Registry::new();
    let mut entities = Registry::new();
    let light_mesh_handle = meshes.add(Mesh::from(Cube::new(0.25)));
    lights.add(Light::Directional(DirectionalProperties::new([-1.0, -0.5, -1.0, 1.0])));

    lights.add(Light::Spot(SpotProperties::new(
        [0.0, 4.0, 0.0, 1.0],
        [0.0, -1.0, 0.0, 1.0],
    )));
    lights.add(Light::Spot(SpotProperties::new(
        [8.0, 4.0, 0.0, 1.0],
        [0.0, -1.0, 0.0, 1.0],
    )));
    lights.add(Light::Point(PointProperties::new([8.0, 4.0, 8.0, 1.0])));
    lights.add(Light::Point(PointProperties::new([-8.0, 4.0, 8.0, 1.0])));

    let world = World::default();
    let tile_mapping = tile::load_tiles(|mesh| meshes.add(mesh))?;

    for x in -3..3 {
        for z in -3..3 {
            let (tile, rotation) = world.get_tile_type(x, z);
            entities.add(Entity {
                mesh_handle: tile_mapping.get(&tile).unwrap().clone(),
                transform: Transform::from_translation_rotation(
                    Vec3::new(x as f32, 0.0, z as f32),
                    Quat::from_rotation_y(rotation),
                ),
            });
        }
    }

    let character = entities.add(Entity {
        mesh_handle: meshes.add(Mesh::from(IcoSphere::new(0.5))),
        transform: Transform::from_translation(Vec3::new(0.0, 0.5, 0.0)),
    });

    let mut follow_camera = FollowCamera::new(
        entities.get(character.clone()).unwrap().transform.clone(),
        renderer.swap_chain_descriptor.width as f32 / renderer.swap_chain_descriptor.height as f32,
    );

    let mut input_all = InputAll::default();
    let mut character_controller = CharacterController::default();
    let mut camera_controller = CameraController::default();
    event_loop.run(move |event, _, control_flow| {
        *control_flow = winit::event_loop::ControlFlow::Poll;
        match event {
            Event::RedrawRequested(_) => {
                keyboard_state_from_events(&input_all.keyboard_events, &mut input_all.keyboard_input);
                character_controller.keyboard(&input_all.keyboard_input);
                camera_controller.mouse_handling(&input_all.mouse_wheel_events, &input_all.mouse_motion_events);
                follow_camera.handle_camera_controller(&camera_controller);
                let entity = entities.get_mut(character.clone()).unwrap();
                entity.transform.rotation *= Quat::from_rotation_y(-character_controller.rotate * 0.02);
                entity.transform.translation += entity.transform.forward() * character_controller.forward * 0.1;
                follow_camera.follow(entity.transform.clone());
                input_all.clear_events();

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
                pipeline_light.render(
                    &light_mesh_handle,
                    &lights,
                    &light_pipeline_bindgroup,
                    &follow_camera,
                    &mut renderer,
                    target,
                );
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
