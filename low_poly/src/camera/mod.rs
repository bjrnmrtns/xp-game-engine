mod components;
mod resources;

pub use components::Followable;
pub use resources::FollowCamera;
pub use resources::FreelookCameras;

use crate::camera;
use bevy::{prelude::*, render::camera::ActiveCameras};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(FreelookCameras::new())
            .add_resource(FollowCamera::new())
            .add_startup_system(camera_startup_system.system())
            .add_system(follow_entity_system.system())
            .add_system(activate_correct_camera_system.system());
    }
}

fn camera_startup_system(
    mut commands: Commands,
    mut freelook_cameras: ResMut<camera::FreelookCameras>,
    mut follow_camera: ResMut<camera::FollowCamera>,
) {
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
}

fn follow_entity_system(
    follow_camera: Res<camera::FollowCamera>,
    mut query: Query<&mut Transform>,
) {
    match (follow_camera.camera, follow_camera.entity) {
        (Some(camera), Some(entity)) => {
            let translation = query.get_mut(entity).unwrap().translation;
            query
                .get_mut(camera)
                .unwrap()
                .look_at(translation, Vec3::unit_y())
        }
        _ => (),
    }
}

fn activate_correct_camera_system(
    mut active_cameras: ResMut<ActiveCameras>,
    controllable_cameras: Res<camera::FreelookCameras>,
) {
    if let Some(camera) = controllable_cameras.get_selected() {
        active_cameras.set(bevy::render::render_graph::base::camera::CAMERA3D, camera);
    }
}
