mod components;
mod resources;

pub use components::Followable;
pub use resources::FollowCamera;
pub use resources::FreelookCameras;

use crate::{camera, client, physics};
use bevy::{prelude::*, render::camera::ActiveCameras};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(FreelookCameras::new())
            .add_resource(FollowCamera::new())
            .add_system(follow_entity_system.system())
            .add_system(activate_correct_camera_system.system());
    }
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
