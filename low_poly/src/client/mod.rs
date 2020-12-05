mod components;
mod resources;

pub use components::CameraController;
pub use components::CameraPlayerOrbit;
pub use components::CharacterController;

use crate::client::resources::{PhysicsSteps, WorldAssetHandle, WorldGrid};
use bevy::prelude::*;
use rapier3d::dynamics::{
    IntegrationParameters, JointSet, RigidBodyBuilder, RigidBodyHandle, RigidBodySet,
};
use rapier3d::geometry::{BroadPhase, ColliderBuilder, ColliderSet, NarrowPhase};
use rapier3d::ncollide::na::{Quaternion, UnitQuaternion, Vector3};
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
            .add_system(physics_system.system())
            .add_resource(WorldGrid::default())
            .add_resource(WorldAssetHandle::default())
            .add_startup_system(load_world.system())
            .add_startup_system(create_world.system())
            .add_system(handle_player_camera.system())
            .add_system(update_world.system());
    }
}

fn load_world(mut world_resource: ResMut<WorldAssetHandle>, asset_server: Res<AssetServer>) {
    world_resource.handle = asset_server.load("world.world");
}

fn create_world(
    mut world_resource: ResMut<WorldAssetHandle>,
    world_assets: Res<Assets<crate::world_loader::WorldAsset>>,
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut bodies: ResMut<RigidBodySet>,
    mut colliders: ResMut<ColliderSet>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if !world_resource.loaded {
        world_resource.loaded = true;
    }
    let grid_texture_handle = asset_server.load("grid.png");
    let rigid_body_ground = RigidBodyBuilder::new_static()
        .translation(0.0, -0.1, 0.0)
        .build();
    let rb_ground_handle = bodies.insert(rigid_body_ground);
    let collider_ground = ColliderBuilder::cuboid(12.0, 0.2, 12.0).build();
    colliders.insert(collider_ground, rb_ground_handle, &mut bodies);
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 24.0 })),
        material: materials.add(StandardMaterial {
            albedo_texture: Some(grid_texture_handle),
            shaded: false,
            ..Default::default()
        }),
        ..Default::default()
    });

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

    let rigid_body_player = RigidBodyBuilder::new_dynamic()
        .translation(0.0, 20.0, 0.0)
        .build();
    let rb_player_handle = bodies.insert(rigid_body_player);
    let collider_player = ColliderBuilder::ball(0.5).friction(0.0).build();
    colliders.insert(collider_player, rb_player_handle, &mut bodies);
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: 0.5,
                subdivisions: 3,
            })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            ..Default::default()
        })
        .with(rb_player_handle)
        .with_children(|parent| {
            parent
                .spawn(CameraPlayerOrbit {
                    transform: Transform::identity(),
                    global_transform: GlobalTransform::identity(),
                })
                .with_children(|parent| {
                    let mut third_person_camera_transform =
                        Transform::from_translation(Vec3::new(-1.0, 2.0, -8.0));
                    third_person_camera_transform.rotation =
                        Transform::from_rotation(Quat::from_rotation_y(std::f32::consts::PI))
                            .rotation;
                    parent.spawn(Camera3dBundle {
                        transform: third_person_camera_transform,
                        ..Default::default()
                    });
                    parent.spawn(PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 4.0)),
                        ..Default::default()
                    });
                })
                .with(CameraController::new());
        })
        .with(CharacterController::new());
}

fn update_world(
    mut world_grid: ResMut<WorldGrid>,
    commands: &mut Commands,
    mut bodies: ResMut<RigidBodySet>,
    mut colliders: ResMut<ColliderSet>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(&CharacterController, &mut Transform, &RigidBodyHandle)>,
) {
    for (character_controller, mut transform, rigid_body_handle) in query.iter_mut() {
        if character_controller.place_object {
            let cube_size = 1.0;
            let one_cube = meshes.add(Mesh::from(shape::Cube { size: cube_size }));
            let new_grid_cell = (4, 4, 4);
            match world_grid.grid.get(&new_grid_cell) {
                None => {
                    let rigid_body_cube = RigidBodyBuilder::new_static()
                        .translation(
                            new_grid_cell.0 as f32 + cube_size / 2.0,
                            new_grid_cell.1 as f32 + cube_size / 2.0,
                            new_grid_cell.2 as f32 + cube_size / 2.0,
                        )
                        .build();
                    let cube_handle = bodies.insert(rigid_body_cube);
                    let collider_cube = ColliderBuilder::cuboid(0.5, 0.5, 0.5).build();
                    colliders.insert(collider_cube, cube_handle, &mut bodies);
                    let entity = commands
                        .spawn(PbrBundle {
                            mesh: one_cube,
                            material: materials.add(Color::rgb(0.3, 0.3, 0.3).into()),
                            transform: Transform::from_translation(Vec3::new(
                                new_grid_cell.0 as f32 + cube_size / 2.0,
                                new_grid_cell.1 as f32 + cube_size / 2.0,
                                new_grid_cell.2 as f32 + cube_size / 2.0,
                            )),
                            ..Default::default()
                        })
                        .current_entity()
                        .unwrap();
                    world_grid.grid.insert(new_grid_cell, entity);
                }
                Some(entity) => (),
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
        let translation = rb.position.translation;
        // translation of physics engine is leading
        transform.translation = Vec3::new(translation.x, translation.y, translation.z);
        transform.rotate(Quat::from_rotation_y(character_controller.rotate_y / 100.0));
        // rotation of controller is leading
        rb.position.rotation = UnitQuaternion::from_quaternion(Quaternion::from([
            transform.rotation.x,
            transform.rotation.y,
            transform.rotation.z,
            transform.rotation.w,
        ]));
        rb.mass_properties.inv_principal_inertia_sqrt = Vector3::new(0.0, 0.0, 0.0);

        let jump_speed = if character_controller.jump { 10.0 } else { 0.0 };

        if let Some(move_forward) = character_controller.move_forward {
            let movement = transform.forward().normalize() * move_forward * 10.0;
            rb.wake_up(true);
            rb.linvel = Vector3::new(movement.x, rb.linvel.y + jump_speed, movement.z);
        } else {
            rb.linvel = Vector3::new(0.0, rb.linvel.y + jump_speed, 0.0);
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
