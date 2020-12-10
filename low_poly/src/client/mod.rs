mod components;
mod helpers;
mod resources;

pub use components::CameraController;
pub use components::CameraPivot;
pub use components::CharacterController;

use crate::client::components::ToolCenter;
use crate::client::helpers::{create_cube, create_player, create_world_ground_plane};
use crate::client::resources::{MeshMap, PhysicsSteps, WorldGrid};
use bevy::prelude::*;
use rapier3d::dynamics::{IntegrationParameters, JointSet, RigidBodyHandle, RigidBodySet};
use rapier3d::geometry::{BroadPhase, ColliderSet, NarrowPhase};
use rapier3d::ncollide::na::{Isometry3, Vector3};
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
            .add_startup_system(load_world_assets.system())
            .add_startup_system(create_world.system())
            .add_system(handle_player_camera.system())
            .add_system(update_world.system());
    }
}

fn load_world_assets(mut meshes: ResMut<Assets<Mesh>>, mut mesh_map: ResMut<MeshMap>) {
    mesh_map.hanldes.insert(
        "one_cube".to_string(),
        meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    );
    mesh_map.hanldes.insert(
        "player".to_string(),
        meshes.add(Mesh::from(shape::Icosphere {
            radius: 0.5,
            subdivisions: 3,
        })),
    );
    mesh_map.hanldes.insert(
        "ground_plane".to_string(),
        meshes.add(Mesh::from(shape::Plane { size: 24.0 })),
    );

    mesh_map.hanldes.insert(
        "tool".to_string(),
        meshes.add(Mesh::from(shape::Icosphere {
            radius: 0.1,
            subdivisions: 3,
        })),
    );
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
    create_cube(
        commands,
        &mut bodies,
        &mut colliders,
        &mesh_map,
        &mut materials,
        (1, 0, 1),
    );
    create_cube(
        commands,
        &mut bodies,
        &mut colliders,
        &mesh_map,
        &mut materials,
        (4, 0, 4),
    );

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
    mut character_controllers: Query<&CharacterController>,
    mut tool_centers: Query<(&ToolCenter, &GlobalTransform)>,
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
        if character_controller.place_object {
            if let Some(grid_cell) = grid_cell {
                match world_grid.grid.get(&grid_cell) {
                    None => {
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
                    Some(_entity) => (),
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
