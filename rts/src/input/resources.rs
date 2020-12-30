use bevy::prelude::*;

#[derive(Default)]
pub struct Selection {
    pub begin: Option<Vec3>,
    pub current_3d_mouse: Option<Vec3>,
}
