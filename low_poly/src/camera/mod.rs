mod components;
mod resources;

pub use components::FollowCamera;
pub use resources::SelectableCameras;

use crate::physics;
use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(SelectableCameras {
            selected: 0,
            camera_ids: vec![],
        })
        .add_system(camera_follow_system.system());
    }
}

fn camera_follow_system(
    query_followables: Query<&physics::Position>,
    mut query_follow_cameras: Query<(&mut Transform, &components::FollowCamera)>,
) {
    for (mut transform, follow) in query_follow_cameras.iter_mut() {
        let position = query_followables.get(follow.entity).unwrap();
        transform.look_at(position.0, Vec3::unit_y());
    }
}
