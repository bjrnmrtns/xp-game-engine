mod components;

pub use components::{CameraController, PlayerController};

use crate::{
    client::components::{CameraCenter, SelectionRender},
    input::Selection,
};
use bevy::prelude::*;

pub struct ClientPlugin;
impl Plugin for ClientPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(create_world.system())
            .add_system(handle_camera.system())
            .add_system(handle_player.system())
            .add_system(handle_selection_rendering.system());
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
        transform: Transform::from_translation(Vec3::new(0.0, 20.0, 0.0)),
        ..Default::default()
    });

    commands
        .spawn(CameraCenter)
        .with(GlobalTransform::identity())
        .with(Transform::identity())
        .with(CameraController::default())
        .with(PlayerController::default())
        .with_children(|parent| {
            parent.spawn(Camera3dBundle {
                transform: Transform::from_translation(Vec3::new(0.0, 20.0, 0.0)).mul_transform(
                    Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
                ),
                ..Default::default()
            });
        });
}

fn handle_camera(mut query: Query<(&CameraController, &mut Transform)>) {
    for (controller, mut center) in query.iter_mut() {
        if let Some(move_position) = controller.move_position {
            center.translation.x += move_position.x;
            center.translation.z -= move_position.y;
        }
    }
}

fn calculate_rectangle(point0: Vec3, point1: Vec3) -> (Vec2, Vec2) {
    let (top_left, bottom_right) = if point0.x < point1.x && point0.z < point1.z {
        (Vec2::new(point0.x, point0.z), Vec2::new(point1.x, point1.z))
    } else if point0.x < point1.x && point0.z > point1.z {
        (Vec2::new(point0.x, point1.z), Vec2::new(point1.x, point0.z))
    } else if point0.x > point1.x && point0.z < point1.z {
        (Vec2::new(point1.x, point0.z), Vec2::new(point0.x, point1.z))
    } else {
        (Vec2::new(point1.x, point1.z), Vec2::new(point0.x, point0.z))
    };

    let midpoint = (top_left + bottom_right) / 2.0;
    let scale = bottom_right - top_left;
    (midpoint, scale)
}

fn handle_selection_rendering(
    selection: Res<Selection>,
    mut query: Query<(&SelectionRender, &mut Visible, &mut Transform)>,
) {
    for (_, mut visible, mut transform) in query.iter_mut() {
        if let (Some(selection_begin), Some(selection_current)) =
            (selection.begin, selection.current_3d_mouse)
        {
            let (midpoint, scale) = calculate_rectangle(selection_begin, selection_current);
            transform.translation = Vec3::new(midpoint.x, 0.5, midpoint.y);
            transform.scale = Vec3::new(scale.x, 1.0, scale.y);
            visible.is_visible = true;
        } else {
            visible.is_visible = false;
        }
    }
}

fn handle_player(
    query: Query<&PlayerController>,
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for controller in query.iter() {
        if let Some(place_object) = controller.select {
            commands.spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                material: materials.add(StandardMaterial {
                    albedo: Color::rgb(1.0, 0.0, 0.0),
                    ..Default::default()
                }),
                transform: Transform::from_translation(place_object),
                ..Default::default()
            });
        }
    }
}
