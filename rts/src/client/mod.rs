mod components;
mod resources;

pub use components::{Action, Controller};

use crate::client::{components::CameraCenter, resources::WorldGrid};
use bevy::prelude::*;

pub struct ClientPlugin;
impl Plugin for ClientPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(WorldGrid::default())
            .add_startup_system(create_world.system())
            .add_system(handle_player_camera.system());
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
            shaded: false,
            ..Default::default()
        }),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        ..Default::default()
    });

    commands.spawn(LightBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 20.0, 0.0)),
        ..Default::default()
    });

    commands
        .spawn(CameraCenter)
        .with(GlobalTransform::identity())
        .with(Transform::identity())
        .with(Controller::default())
        .with_children(|parent| {
            parent.spawn(Camera3dBundle {
                transform: Transform::from_translation(Vec3::new(0.0, 200.0, 0.0)).mul_transform(
                    Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
                ),
                ..Default::default()
            });
        });
}

fn handle_player_camera(mut query: Query<(&Controller, &mut Transform)>) {
    for (controller, mut center) in query.iter_mut() {
        if let Some(move_position) = controller.move_position {
            center.translation.x += move_position.x;
            center.translation.z -= move_position.y;
        }
    }
}
