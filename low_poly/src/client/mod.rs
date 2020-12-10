mod components;
mod helpers;
mod resources;

pub use components::CameraController;
pub use components::CameraPlayerOrbit;
pub use components::CharacterController;

use crate::client::helpers::{create_cube, create_player, create_world_ground_plane};
use crate::client::resources::{MeshMap, PhysicsSteps, WorldAssetHandle, WorldGrid};
use bevy::prelude::*;
use rapier3d::dynamics::{
    IntegrationParameters, JointSet, MassProperties, RigidBodyBuilder, RigidBodyHandle,
    RigidBodySet,
};
use rapier3d::geometry::{BroadPhase, ColliderBuilder, ColliderSet, NarrowPhase};
use rapier3d::ncollide::na::{Isometry3, Quaternion, UnitQuaternion, Vector3};
use rapier3d::pipeline::PhysicsPipeline;

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
            .add_resource(WorldAssetHandle::default())
            .add_startup_system(load_world_assets.system())
            .add_startup_system(create_world.system())
            .add_system(handle_player_camera.system())
            .add_system(update_world.system());
    }
}

fn load_world_assets(
    mut meshes: ResMut<Assets<Mesh>>,
    mut mesh_map: ResMut<MeshMap>,
    mut world_resource: ResMut<WorldAssetHandle>,
    asset_server: Res<AssetServer>,
) {
    world_resource.handle = asset_server.load("world.world");
    mesh_map.hanldes.insert(
        "one_cube".to_string(),
        meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    );
}

fn create_world(
    mut world_resource: ResMut<WorldAssetHandle>,
    _world_assets: Res<Assets<crate::world_loader::WorldAsset>>,
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut bodies: ResMut<RigidBodySet>,
    mut colliders: ResMut<ColliderSet>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mesh_map: ResMut<MeshMap>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if !world_resource.loaded {
        world_resource.loaded = true;
    }
    mesh_map.hanldes.insert(
        "player".to_string(),
        meshes.add(Mesh::from(shape::Icosphere {
            radius: 0.5,
            subdivisions: 3,
        })),
    );
    create_world_ground_plane(
        commands,
        &asset_server,
        &mut bodies,
        &mut colliders,
        &mut meshes,
        &mut materials,
    );

    let rigid_body_cube = RigidBodyBuilder::new_static()
        .translation(-8.0, 2.0, -8.0)
        .build();
    let rb_cube_handle = bodies.insert(rigid_body_cube);
    let collider_cube = ColliderBuilder::cuboid(2.0, 2.0, 2.0).build();
    colliders.insert(collider_cube, rb_cube_handle, &mut bodies);
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 4.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        transform: Transform::from_translation(Vec3::new(-8.0, 2.0, -8.0)),
        ..Default::default()
    });

    let one_cube = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));

    let rigid_body_stepup_cube = RigidBodyBuilder::new_static()
        .translation(8.0, 0.2, 8.0)
        .build();
    let rb_stepup_cube_handle = bodies.insert(rigid_body_stepup_cube);
    let collider_stepup_cube = ColliderBuilder::cuboid(2.0, 0.2, 2.0).build();
    colliders.insert(collider_stepup_cube, rb_stepup_cube_handle, &mut bodies);
    commands.spawn(PbrBundle {
        mesh: one_cube.clone(),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        transform: Transform::from_translation(Vec3::new(8.0, 0.2, 8.0))
            .mul_transform(Transform::from_scale(Vec3::new(4.0, 0.4, 4.0))),
        ..Default::default()
    });

    commands.spawn(LightBundle {
        transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
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
    mesh_map: Res<MeshMap>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(&CharacterController, &mut Transform, &RigidBodyHandle)>,
) {
    for (character_controller, mut transform, rigid_body_handle) in query.iter_mut() {
        if character_controller.place_object {
            let new_grid_cell = (4, 4, 4);
            match world_grid.grid.get(&new_grid_cell) {
                None => {
                    let entity = create_cube(
                        commands,
                        &mut bodies,
                        &mut colliders,
                        &mesh_map,
                        &mut materials,
                        new_grid_cell,
                    );
                    world_grid.grid.insert(new_grid_cell, entity);
                }
                Some(_entity) => (),
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
        let mut rb = bodies.get_mut(*rigid_body_handle).unwrap();
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

        if let Some(move_forward) = character_controller.move_forward {
            let movement = transform.forward().normalize() * move_forward * 10.0;
            rb.set_linvel(
                Vector3::new(movement.x, rb.linvel().y + jump_speed, movement.z),
                true,
            );
        } else {
            rb.set_linvel(Vector3::new(0.0, rb.linvel().y + jump_speed, 0.0), true);
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
