use bevy::prelude::*;
use std::ops::DerefMut;

use crate::camera;
use crate::client;
use crate::physics;

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup_scene.system());
    }
}

fn setup_scene(
    mut commands: Commands,
    mut freelook_cameras: ResMut<camera::FreelookCameras>,
    mut follow_camera: ResMut<camera::FollowCamera>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
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
    let freelook1 = commands
        .spawn(Camera3dComponents {
            transform: Transform::from_translation(Vec3::new(-3.0, 1.0, 8.0))
                .looking_at(Vec3::default(), Vec3::unit_y()),
            ..Default::default()
        })
        .current_entity();
    let freelook2 = commands
        .spawn(Camera3dComponents {
            transform: Transform::from_translation(Vec3::new(-5.0, 8.0, -8.0))
                .looking_at(Vec3::default(), Vec3::unit_y()),
            ..Default::default()
        })
        .current_entity();
    let follow = commands
        .spawn(Camera3dComponents {
            transform: Transform::from_translation(Vec3::new(8.0, 8.0, -8.0)),
            ..Default::default()
        })
        .current_entity();
    freelook_cameras.add(freelook1.unwrap());
    freelook_cameras.add(freelook2.unwrap());
    freelook_cameras.add(follow.unwrap());
    follow_camera.set_follow_camera(follow.unwrap());

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
            .with(client::EntityController)
            .current_entity()
            .unwrap(),
    );
}
