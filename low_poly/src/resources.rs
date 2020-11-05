use bevy::prelude::*;

#[derive(Debug)]
pub enum Camera
{
    Follow,
    Freelook,
}

impl Default for Camera {
    fn default() -> Self {
        Self::Follow
    }
}

#[derive(Debug, Default)]
pub struct CameraSelected {
    pub camera: Camera,
}
