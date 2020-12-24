mod components;
mod helpers;
mod resources;

pub use components::{Action, CameraController, CameraPivot, CharacterController};

use crate::client::{
    components::{ToolCenter, VegetationEntity},
    helpers::{create_cube, create_player, create_world_ground_plane},
    resources::{MeshMap, PhysicsSteps, WorldGrid},
};
use bevy::{prelude::*, render::mesh::VertexAttributeValues};
use noise::{Fbm, MultiFractal, NoiseFn};
use rand::prelude::Distribution;
use rapier3d::{
    dynamics::{IntegrationParameters, JointSet, RigidBodyHandle, RigidBodySet},
    geometry::{BroadPhase, ColliderHandle, ColliderSet, NarrowPhase},
    ncollide::na::{Isometry3, Vector3},
    pipeline::PhysicsPipeline,
};

pub struct ClientPlugin;
impl Plugin for ClientPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(IntegrationParameters::default())
            .add_resource(PhysicsPipeline::new())
            .add_resource(BroadPhase::new())
            .add_resource(NarrowPhase::new())
            .add_resource(RigidBodySet::new())
            .add_resource(ColliderSet::new())
            .add_resource(JointSet::new())
            .add_resource(PhysicsSteps::new())
            .add_resource(MeshMap::default())
            .add_system(physics_system.system())
            .add_resource(WorldGrid::default())
            .add_startup_system(load_world_assets.system())
            .add_startup_system(create_world.system())
            .add_system(handle_player_camera.system())
            .add_system(update_world.system())
            .add_system(mesh_asset_add_uv_coord.system());
    }
}

fn load_world_assets(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mesh_map: ResMut<MeshMap>,
) {
    mesh_map.handles.insert(
        "one_cube".to_string(),
        meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    );
    mesh_map.handles.insert(
        "player".to_string(),
        meshes.add(Mesh::from(shape::Icosphere {
            radius: 0.5,
            subdivisions: 3,
        })),
    );
    mesh_map.handles.insert(
        "ground_plane".to_string(),
        meshes.add(Mesh::from(shape::Plane { size: 24.0 })),
    );

    mesh_map.handles.insert(
        "tool".to_string(),
        meshes.add(Mesh::from(shape::Icosphere {
            radius: 0.1,
            subdivisions: 3,
        })),
    );
    let range = rand::distributions::Uniform::new(-1.0, 1.0);
    let mut rng = rand::thread_rng();
    let scene_handle = asset_server.load("tree.gltf");
    let trees = Fbm::new()
        .set_octaves(6)
        .set_frequency(0.001)
        .set_lacunarity(2.09)
        .set_persistence(1.0);
    for x in -30..30 {
        for z in -30..30 {
            let x_offset = range.sample(&mut rng);
            let z_offset = range.sample(&mut rng);
            let rotation = range.sample(&mut rng);
            if trees.get([x as f64, z as f64]) > 0.4 {
                commands
                    .spawn(VegetationEntity)
                    .with(
                        Transform::from_translation(Vec3::new(
                            x as f32 * 5.0 + x_offset,
                            0.0,
                            z as f32 * 5.0 + z_offset,
                        ))
                        .mul_transform(Transform::from_rotation(
                            Quat::from_rotation_y(std::f32::consts::PI * rotation),
                        )),
                    )
                    .with(GlobalTransform::default())
                    .with_children(|parent| {
                        parent.spawn_scene(scene_handle.clone());
                    });
            }
        }
    }
}

#[derive(Default)]
struct State {
    mesh_event_reader: EventReader<AssetEvent<Mesh>>,
}

fn mesh_asset_add_uv_coord(
    mut state: Local<State>,
    mesh_events: Res<Events<AssetEvent<Mesh>>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for event in state.mesh_event_reader.iter(&mesh_events) {
        if let AssetEvent::Created { handle } = event {
            let mesh = meshes.get_mut(handle).unwrap();
            if let None = mesh.attribute(Mesh::ATTRIBUTE_UV_0) {
                let uvs =
                    vec![[0.0f32, 0.0f32]; mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap().len()];
                mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, VertexAttributeValues::Float2(uvs));
            }
        }
    }
}

fn create_world(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut bodies: ResMut<RigidBodySet>,
    mut colliders: ResMut<ColliderSet>,
    mesh_map: Res<MeshMap>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    create_world_ground_plane(
        commands,
        &asset_server,
        &mut bodies,
        &mut colliders,
        &mesh_map,
        &mut materials,
    );

    commands.spawn(LightBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 80.0, 0.0)),
        ..Default::default()
    });

    create_player(
        commands,
        &mut bodies,
        &mut colliders,
        &mesh_map,
        &mut materials,
    );
}

fn update_world(
    mut world_grid: ResMut<WorldGrid>,
    commands: &mut Commands,
    mut bodies: ResMut<RigidBodySet>,
    mut colliders: ResMut<ColliderSet>,
    mut joints: ResMut<JointSet>,
    mesh_map: Res<MeshMap>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut character_controllers: Query<&CharacterController>,
    mut tool_centers: Query<(&ToolCenter, &GlobalTransform)>,
    world_build: Query<(&RigidBodyHandle, &ColliderHandle)>,
) {
    let mut grid_cell = None;
    for (_, global_transform) in tool_centers.iter_mut() {
        grid_cell = Some((
            global_transform.translation.x.round() as i32,
            global_transform.translation.y.round() as i32,
            global_transform.translation.z.round() as i32,
        ));
    }
    for character_controller in character_controllers.iter_mut() {
        if character_controller.action_enabled {
            if let Some(grid_cell) = grid_cell {
                match world_grid.grid.get(&grid_cell) {
                    None => {
                        if character_controller.action == Action::Add {
                            let entity = create_cube(
                                commands,
                                &mut bodies,
                                &mut colliders,
                                &mesh_map,
                                &mut materials,
                                grid_cell,
                            );
                            world_grid.grid.insert(grid_cell, entity);
                        }
                    }
                    Some(entity) => {
                        if character_controller.action == Action::Remove {
                            let (rigid_body_handle, collider_handle) =
                                world_build.get(*entity).unwrap();
                            bodies.remove(*rigid_body_handle, &mut colliders, &mut joints);
                            colliders.remove(*collider_handle, &mut bodies, false);
                            commands.remove::<PbrBundle>(*entity);
                            world_grid.grid.remove(&grid_cell);
                        }
                    }
                }
            }
        }
    }
}

fn handle_player_camera(mut query: Query<(&CameraController, &mut Transform)>) {
    for (camera_controller, mut camera_orbit) in query.iter_mut() {
        camera_orbit.rotate(Quat::from_rotation_x(camera_controller.rotate_x / 100.0));
    }
}

fn physics_system(
    time: Res<Time>,
    integration_parameters: Res<IntegrationParameters>,
    mut physics_steps: ResMut<PhysicsSteps>,
    mut pipeline: ResMut<PhysicsPipeline>,
    mut broad_phase: ResMut<BroadPhase>,
    mut narrow_phase: ResMut<NarrowPhase>,
    mut bodies: ResMut<RigidBodySet>,
    mut colliders: ResMut<ColliderSet>,
    mut joints: ResMut<JointSet>,
    mut query: Query<(&CharacterController, &mut Transform, &RigidBodyHandle)>,
) {
    for (character_controller, mut transform, rigid_body_handle) in query.iter_mut() {
        let rb = bodies.get_mut(*rigid_body_handle).unwrap();
        let translation = rb.position().translation;
        // translation of physics engine is leading
        transform.translation = Vec3::new(translation.x, translation.y, translation.z);
        transform.rotate(Quat::from_rotation_y(character_controller.rotate_y / 100.0));
        // rotation of controller is leading
        rb.set_position(
            Isometry3::new(
                Vector3::new(
                    transform.translation.x,
                    transform.translation.y,
                    transform.translation.z,
                ),
                /*Quaternion::from([
                    transform.rotation.x,
                    transform.rotation.y,
                    transform.rotation.z,
                    transform.rotation.w,
                ])
                .vector()
                .normalize()*/
                Vector3::new(0.0, 0.0, 0.0),
            ),
            true,
        );

        let jump_speed = if character_controller.jump { 10.0 } else { 0.0 };

        match (
            character_controller.move_forward,
            character_controller.strafe_right,
        ) {
            (Some(move_forward), Some(strafe_right)) => {
                let movement_f = transform.forward() * move_forward;
                let movement_r = transform.forward().cross(Vec3::unit_y()) * strafe_right;
                let movement = (movement_f + movement_r) * 10.0;
                rb.set_linvel(
                    Vector3::new(movement.x, rb.linvel().y + jump_speed, movement.z),
                    true,
                );
            }
            (_, _) => (),
        }
    }
    let expected_steps =
        (time.time_since_startup().as_secs_f64() / integration_parameters.dt() as f64) as u64;
    for _ in physics_steps.done..expected_steps {
        pipeline.step(
            &(Vector3::y() * -40.0),
            &integration_parameters,
            &mut broad_phase,
            &mut narrow_phase,
            &mut bodies,
            &mut colliders,
            &mut joints,
            None,
            None,
            &(),
        );
    }
    physics_steps.done = expected_steps;
}
