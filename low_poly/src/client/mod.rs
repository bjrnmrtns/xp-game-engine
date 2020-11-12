mod components;
mod resources;

pub use components::CharacterController;
pub use resources::ControllableEntities;

use bevy::prelude::*;
use std::ops::{Deref, DerefMut};

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(ControllableEntities::new())
            .add_startup_system(client_startup_system.system())
            .add_system(handle_physics.system());
    }
}

fn client_startup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
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

    let player = commands
        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: 2.0,
                subdivisions: 3,
            })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            ..Default::default()
        })
        .with_children(|parent| {
            let mut transform = Transform::from_translation(Vec3::new(-1.0, 1.0, -8.0));
            transform.rotation =
                Transform::from_rotation(Quat::from_rotation_y(std::f32::consts::PI)).rotation;
            parent.spawn(Camera3dComponents {
                transform,
                ..Default::default()
            });
        })
        .with(CharacterController::new())
        .current_entity();

    controllable_entities.add(player.unwrap());
}

fn handle_physics(time: Res<Time>, mut query: Query<(&CharacterController, &mut Transform)>) {
    for (character_controller, mut transform) in query.iter_mut() {
        if let Some(move_forward) = character_controller.move_forward {
            let movement = transform.forward() * move_forward * time.delta_seconds;
            transform.deref_mut().translation += movement;
        }
    }
}
