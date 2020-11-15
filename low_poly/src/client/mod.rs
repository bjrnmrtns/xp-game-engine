mod components;

pub use components::CameraController;
pub use components::CameraNodeThirdPerson;
pub use components::CharacterController;

use bevy::prelude::*;
use rapier3d::dynamics::{RigidBodyBuilder, RigidBodyHandle, RigidBodySet};
use rapier3d::geometry::{ColliderBuilder, ColliderSet};
use rapier3d::na::{Vector3, Vector4};
use rapier3d::ncollide::na::{Isometry3, Quaternion, UnitQuaternion};

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(client_startup_system.system())
            .add_system(handle_physics.system())
            .add_system(handle_player_camera.system());
    }
}

fn client_startup_system(
    commands: &mut Commands,
    mut bodies: ResMut<RigidBodySet>,
    mut colliders: ResMut<ColliderSet>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let rigid_body_cube = RigidBodyBuilder::new_static()
        .translation(-5.0, 2.0, -5.0)
        .build();
    let rb_cube_handle = bodies.insert(rigid_body_cube);
    let collider_cube = ColliderBuilder::cuboid(4.0, 4.0, 4.0).build();
    colliders.insert(collider_cube, rb_cube_handle, &mut bodies);
    commands.spawn(PbrComponents {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 4.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        transform: Transform::from_translation(Vec3::new(-5.0, 2.0, -5.0)),
        ..Default::default()
    });

    let rigid_body_stepup_cube = RigidBodyBuilder::new_static()
        .translation(5.0, 0.2, 5.0)
        .build();
    let rb_stepup_cube_handle = bodies.insert(rigid_body_stepup_cube);
    let collider_stepup_cube = ColliderBuilder::cuboid(4.0, 0.4, 4.0).build();
    colliders.insert(collider_stepup_cube, rb_stepup_cube_handle, &mut bodies);
    commands.spawn(PbrComponents {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        transform: Transform::from_translation(Vec3::new(5.0, 0.2, 5.0))
            .mul_transform(Transform::from_scale(Vec3::new(4.0, 0.4, 4.0))),
        ..Default::default()
    });
    let rigid_body_ground = RigidBodyBuilder::new_static()
        .translation(0.0, -0.1, 0.0)
        .build();
    let rb_ground_handle = bodies.insert(rigid_body_ground);
    let collider_ground = ColliderBuilder::cuboid(25.0, 0.1, 25.0).build();
    colliders.insert(collider_ground, rb_ground_handle, &mut bodies);
    commands.spawn(PbrComponents {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 50.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });

    commands.spawn(LightComponents {
        transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
        ..Default::default()
    });

    let rigid_body_player = RigidBodyBuilder::new_dynamic()
        .translation(0.0, 20.0, 0.0)
        .build();
    let rb_player_handle = bodies.insert(rigid_body_player);
    let mut collider_player = ColliderBuilder::ball(2.0).friction(0.0).build();
    colliders.insert(collider_player, rb_player_handle, &mut bodies);

    commands
        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: 2.0,
                subdivisions: 3,
            })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            ..Default::default()
        })
        .with(rb_player_handle)
        .with_children(|parent| {
            parent
                .spawn(CameraNodeThirdPerson {
                    transform: Transform::identity(),
                    global_transform: GlobalTransform::identity(),
                })
                .with_children(|parent| {
                    let mut transform = Transform::from_translation(Vec3::new(-1.0, 1.0, -8.0));
                    transform.rotation =
                        Transform::from_rotation(Quat::from_rotation_y(std::f32::consts::PI))
                            .rotation;
                    parent.spawn(Camera3dComponents {
                        transform,
                        ..Default::default()
                    });
                })
                .with(CameraController::new());
        })
        .with(CharacterController::new());
}

fn handle_player_camera(mut query: Query<(&CameraController, &mut Transform)>) {
    for (camera_controller, mut camera_orbit) in query.iter_mut() {
        camera_orbit.rotate(Quat::from_rotation_x(camera_controller.rotate_x / 100.0));
    }
}

fn handle_physics(
    time: Res<Time>,
    mut bodies: ResMut<RigidBodySet>,
    mut query: Query<(&CharacterController, &mut Transform, &RigidBodyHandle)>,
) {
    for (character_controller, mut transform, rigid_body_handle) in query.iter_mut() {
        let mut rb = bodies.get_mut(*rigid_body_handle).unwrap();
        // TODO: only need to be set once
        rb.mass_properties.inv_principal_inertia_sqrt.x = 0.0;
        rb.mass_properties.inv_principal_inertia_sqrt.y = 0.0;
        rb.mass_properties.inv_principal_inertia_sqrt.z = 0.0;
        let translation = rb.position.translation;
        // translation of physics engine is leading
        transform.translation = Vec3::new(translation.x, translation.y, translation.z);
        transform.rotate(Quat::from_rotation_y(character_controller.rotate_y / 100.0));
        // rotation of controller is leading
        rb.position.rotation = UnitQuaternion::from_quaternion(Quaternion::from([
            transform.rotation.x(),
            transform.rotation.y(),
            transform.rotation.z(),
            transform.rotation.w(),
        ]));

        if let Some(move_forward) = character_controller.move_forward {
            let movement = transform.forward().normalize() * move_forward * 10.0;
            rb.wake_up(true);
            rb.linvel = Vector3::new(movement.x(), rb.linvel.y, movement.z());
        } else {
            rb.linvel = Vector3::new(0.0, rb.linvel.y, 0.0);
        }
    }
}
