mod components;

pub use components::CameraController;
pub use components::CameraNodeThirdPerson;
pub use components::CharacterController;

use bevy::prelude::*;
use std::ops::DerefMut;

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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrComponents {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 100.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });
    commands.spawn(LightComponents {
        transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
        ..Default::default()
    });

    commands
        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: 2.0,
                subdivisions: 3,
            })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            ..Default::default()
        })
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

fn handle_physics(time: Res<Time>, mut query: Query<(&CharacterController, &mut Transform)>) {
    for (character_controller, mut transform) in query.iter_mut() {
        transform.rotate(Quat::from_rotation_y(character_controller.rotate_y / 100.0));
        if let Some(move_forward) = character_controller.move_forward {
            let movement = transform.forward() * move_forward * time.delta_seconds;
            transform.deref_mut().translation += movement;
        }
    }
}
