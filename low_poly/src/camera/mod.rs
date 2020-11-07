mod components;
mod resources;

pub use components::{CameraController, FollowCamera};
pub use resources::Cameras;
pub use resources::FollowEntity;

use crate::{camera, client};
use bevy::{prelude::*, render::camera::ActiveCameras};
use std::ops::DerefMut;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(Cameras::new())
            .add_resource(FollowEntity::new())
            .add_startup_system(camera_startup_system.system())
            .add_system(follow_entity_system.system())
            .add_system(activate_correct_camera_system.system());
    }
}

fn camera_startup_system(mut commands: Commands, mut cameras: ResMut<camera::Cameras>) {
    let camera1 = commands
        .spawn(Camera3dComponents {
            transform: Transform::from_translation(Vec3::new(-3.0, 1.0, 8.0))
                .looking_at(Vec3::default(), Vec3::unit_y()),
            ..Default::default()
        })
        .current_entity();
    let camera2 = commands
        .spawn(Camera3dComponents {
            transform: Transform::from_translation(Vec3::new(-5.0, 8.0, -8.0))
                .looking_at(Vec3::default(), Vec3::unit_y()),
            ..Default::default()
        })
        .current_entity();
    let camera3 = commands
        .spawn(Camera3dComponents {
            transform: Transform::from_translation(Vec3::new(8.0, 8.0, -8.0)),
            ..Default::default()
        })
        .with(FollowCamera::new())
        .with(CameraController::new())
        .current_entity();
    cameras.add(camera1.unwrap());
    cameras.add(camera2.unwrap());
    cameras.add(camera3.unwrap());
}

fn follow_entity_system(
    follow_entity: Res<camera::FollowEntity>,
    query_player: Query<&client::Player>,
    mut query_camera: Query<(&mut Transform, &components::FollowCamera)>,
) {
    if let Some(entity) = follow_entity.entity {
        let player = query_player.get(entity).unwrap();
        let camera_position = player.position - player.direction * 5.0;
        for (mut transform, _) in query_camera.iter_mut() {
            *transform.deref_mut() = Transform::from_translation(camera_position)
                .looking_at(player.position, Vec3::unit_y());
        }
    }
}

fn activate_correct_camera_system(
    mut active_cameras: ResMut<ActiveCameras>,
    controllable_cameras: Res<camera::Cameras>,
) {
    if let Some(camera) = controllable_cameras.get_selected() {
        active_cameras.set(bevy::render::render_graph::base::camera::CAMERA3D, camera);
    }
}
