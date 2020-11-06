use bevy::prelude::*;
use std::ops::DerefMut;

use crate::camera;
use crate::physics;

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup_scene.system());
    }
}

fn setup_scene(
    mut commands: Commands,
    mut selectable_cameras: ResMut<camera::SelectableCameras>,
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
    if let Some(camera_entity) = commands
        .spawn(Camera3dComponents {
            transform: Transform::from_translation(Vec3::new(-3.0, 0.0, 8.0))
                .looking_at(Vec3::default(), Vec3::unit_y()),
            ..Default::default()
        })
        .current_entity()
    {
        selectable_cameras
            .deref_mut()
            .camera_ids
            .push(camera_entity);
    }
    if let Some(entity_to_follow) = commands
        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: 2.0,
                subdivisions: 3,
            })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..Default::default()
        })
        .with(physics::Position {
            0: Vec3::new(0.0, 3.0, 0.0),
        })
        .with(physics::EntityController)
        .current_entity()
    {
        if let Some(camera_entity) = commands
            .spawn(Camera3dComponents {
                transform: Transform::from_translation(Vec3::new(8.0, 8.0, -8.0)),
                ..Default::default()
            })
            .with(camera::FollowCamera {
                entity: entity_to_follow,
            })
            .current_entity()
        {
            selectable_cameras
                .deref_mut()
                .camera_ids
                .push(camera_entity);
        }
    }
}
