mod components;
mod resources;

pub use components::EntityController;
pub use components::Player;
pub use resources::ControllableEntities;

use crate::camera;
use bevy::prelude::*;

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
    mut follow_camera: ResMut<camera::FollowEntity>,
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
            ..Default::default()
        })
        .with(Player::new())
        .with(EntityController::new())
        .current_entity();
    let player2 = commands
        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: 2.0,
                subdivisions: 3,
            })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            ..Default::default()
        })
        .with(Player::new().with_position(Vec3::new(3.0, 0.0, 3.0)))
        .with(EntityController::new())
        .current_entity();
    follow_camera.set_entity(player1.unwrap());
    controllable_entities.add(player1.unwrap());
    controllable_entities.add(player2.unwrap());
}

fn handle_physics(
    time: Res<Time>,
    mut query: Query<(&EntityController, &mut Transform, &mut Player)>,
) {
    for (entity_controller, mut transform, mut player) in query.iter_mut() {
        if let Some(move_forward) = entity_controller.move_forward {
            let translation = player.direction * time.delta_seconds * move_forward;
            player.position += translation;
            transform.translation = player.position;
        }
    }
}
