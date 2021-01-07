mod components;

pub use components::{
    CameraCenterController, CameraZoomController, PlayerController, SelectionRender,
};

use crate::client::components::{CameraCenter, Unit};
use bevy::prelude::*;

pub struct ClientPlugin;
impl Plugin for ClientPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(create_world.system())
            .add_system(handle_camera.system())
            .add_system(handle_player.system());
    }
}

fn create_world(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 100.0 })),
        material: materials.add(StandardMaterial {
            albedo: Color::rgb(0.0, 1.0, 0.0),
            ..Default::default()
        }),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        ..Default::default()
    });

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 1.0 })),
            material: materials.add(StandardMaterial {
                albedo: Color::rgba(0.0, 0.0, 1.0, 0.25),
                ..Default::default()
            }),
            visible: Visible {
                is_visible: false,
                is_transparent: true,
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.1, 0.0)),
            ..Default::default()
        })
        .with(SelectionRender);

    commands.spawn(LightBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 120.0, 0.0)),
        ..Default::default()
    });

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(StandardMaterial {
                albedo: Color::rgb(1.0, 0.0, 1.0),
                ..Default::default()
            }),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..Default::default()
        })
        .with(Unit);

    commands
        .spawn(CameraCenter)
        .with(GlobalTransform::identity())
        .with(Transform::identity())
        .with(CameraCenterController::default())
        .with(PlayerController::default())
        .with_children(|parent| {
            parent
                .spawn(Camera3dBundle {
                    transform: Transform::from_translation(Vec3::new(0.0, 20.0, 0.0))
                        .mul_transform(Transform::from_rotation(Quat::from_rotation_x(
                            -std::f32::consts::FRAC_PI_2,
                        ))),
                    ..Default::default()
                })
                .with(CameraZoomController::default());
        });
}

fn handle_camera(
    mut query_center: Query<(&CameraCenterController, &mut Transform)>,
    mut query_zoom: Query<(&mut CameraZoomController, &mut Transform)>,
) {
    for (controller, mut center) in query_center.iter_mut() {
        if let Some(move_position) = controller.move_position {
            center.translation.x += move_position.x * 0.5;
            center.translation.z -= move_position.y * 0.5;
        }
    }
    for (mut controller, mut center) in query_zoom.iter_mut() {
        if let Some(zoom) = controller.zoom {
            center.translation.y += zoom;
            controller.zoom = None;
        }
    }
}

fn handle_player(
    mut query: Query<&mut PlayerController>,
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for mut controller in query.iter_mut() {
        if let Some((begin, end)) = controller.rectangle_select {
            commands.spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                material: materials.add(StandardMaterial {
                    albedo: Color::rgb(1.0, 0.0, 0.0),
                    ..Default::default()
                }),
                transform: Transform::from_translation(end),
                ..Default::default()
            });
            println!("{:?}, {:?}", begin, end);
            controller.rectangle_select = None;
        }
    }
}
