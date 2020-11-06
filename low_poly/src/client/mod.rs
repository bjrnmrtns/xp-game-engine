mod components;
mod resources;

pub use components::EntityController;

use crate::camera;
use bevy::prelude::*;

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(client_startup_system.system());
    }
}

fn client_startup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut follow_camera: ResMut<camera::FollowCamera>,
) {
    follow_camera.set_entity(
        commands
            .spawn(PbrComponents {
                mesh: meshes.add(Mesh::from(shape::Icosphere {
                    radius: 2.0,
                    subdivisions: 3,
                })),
                material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
                ..Default::default()
            })
            .with(EntityController)
            .current_entity()
            .unwrap(),
    );
}
