use crate::camera;
use bevy::{
    input::{keyboard::KeyCode, Input},
    prelude::*,
    render::camera::ActiveCameras,
};
use std::ops::DerefMut;

pub fn input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut selectable_cameras: ResMut<camera::SelectableCameras>,
    mut active_cameras: ResMut<ActiveCameras>,
) {
    if keyboard_input.just_pressed(KeyCode::C) {
        active_cameras.set(
            bevy::render::render_graph::base::camera::CAMERA3D,
            selectable_cameras.camera_ids[selectable_cameras.selected],
        );
        selectable_cameras.selected =
            (selectable_cameras.selected + 1) % selectable_cameras.camera_ids.len();
    }
}
