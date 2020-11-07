use bevy::prelude::*;

#[derive(Debug, Default, Properties)]
pub struct CameraController {
    horizontal_axis: f32,
    vertical_axis: f32,
}

impl CameraController {
    pub fn new() -> Self {
        Self {
            horizontal_axis: 0.0,
            vertical_axis: 0.0,
        }
    }
}

#[derive(Debug, Default, Properties)]
pub struct FollowCamera {
    distance_to_entity: f32,
    yaw_direction_offset: f32,
    pitch_direction_offset: f32,
}

impl FollowCamera {
    pub fn new() -> Self {
        Self {
            distance_to_entity: 5.0,
            yaw_direction_offset: 0.0,
            pitch_direction_offset: 0.0,
        }
    }
}
#[derive(Debug, Default, Properties)]
pub struct FreelookCamera;
