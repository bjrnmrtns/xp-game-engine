use crate::{components, resources};
use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(camera_system.system())
            .init_resource::<resources::CameraSelected>();
    }
}

fn camera_system(
    mut camera_selected: ResMut<resources::CameraSelected>,
    mut query: Query<&mut components::actor::Controller>,
) {
    for mut controller in &mut query.iter_mut() {
        if (*controller).toggle_camera {
            match camera_selected.camera {
                resources::Camera::Follow => camera_selected.camera = resources::Camera::Freelook,
                resources::Camera::Freelook => camera_selected.camera = resources::Camera::Follow,
            }
        }
        (*controller).toggle_camera = false;
    }
}
