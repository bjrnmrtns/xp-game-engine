use crate::camera;
use bevy::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(input_system.system());
    }
}

fn input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut controllable_cameras: ResMut<camera::FreelookCameras>,
) {
    if keyboard_input.just_pressed(KeyCode::C) {
        controllable_cameras.toggle();
    }
}
