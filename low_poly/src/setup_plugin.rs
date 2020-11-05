pub struct SetupPlugin;
use crate::components;

use bevy::prelude::*;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup_scene.system());
    }
}

fn setup_scene(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    commands.spawn(PbrComponents {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 10.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });
    commands.spawn(LightComponents {
        transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
        ..Default::default()
    });
    commands.spawn(Camera3dComponents {
        transform: Transform::from_translation(Vec3::new(-3.0, 5.0, 8.0))
            .looking_at(Vec3::default(), Vec3::unit_y()),
        ..Default::default()
    });
    spawn_player(&mut commands, &mut meshes, &mut materials);
}

fn spawn_player(commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<StandardMaterial>>) {
        commands.spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Icosphere { radius: 2.0, subdivisions: 3 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..Default::default()})
        .with(components::actor::Player)
        .with(components::actor::Controller { forward: 0.0, right: 0.0, toggle_camera: false })
        .with(components::physics::Velocity::new(0.0, 0.0, 0.0))
        .with(components::physics::Position::new(0.0, 1.0, 0.0));
}