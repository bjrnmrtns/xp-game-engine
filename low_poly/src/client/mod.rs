mod components;
mod resources;

pub use components::EntityController;
pub use resources::ControllableEntities;

use crate::camera;
use bevy::prelude::*;

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(ControllableEntities::new())
            .add_startup_system(client_startup_system.system())
            .add_system(handle_controllables_system.system());
    }
}

fn client_startup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut follow_camera: ResMut<camera::FollowCamera>,
    mut controllable_entities: ResMut<ControllableEntities>,
) {
    commands.spawn(PbrComponents {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 10.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });
    commands.spawn(LightComponents {
        transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
        ..Default::default()
    });

    let player1 = commands
        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: 2.0,
                subdivisions: 3,
            })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..Default::default()
        })
        .with(EntityController::new())
        .current_entity();
    let player2 = commands
        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: 2.0,
                subdivisions: 3,
            })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_translation(Vec3::new(3.0, 0.0, 0.0)),
            ..Default::default()
        })
        .with(EntityController::new())
        .current_entity();
    follow_camera.set_entity(player1.unwrap());
    controllable_entities.add(player1.unwrap());
    controllable_entities.add(player2.unwrap());
}

fn handle_controllables_system(
    time: Res<Time>,
    mut query: Query<(&mut EntityController, &mut Transform)>,
) {
    for (mut entity_controller, mut transform) in query.iter_mut() {
        transform.translation += entity_controller.transform.translation * time.delta_seconds;
        entity_controller.transform = Transform::default();
    }
}
