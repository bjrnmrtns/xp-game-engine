use crate::components;

use bevy::{
    input::{keyboard::KeyCode, Input},
    prelude::*,
};

pub fn handle_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(
        &components::actor::Player,
        &mut components::actor::Controller,
    )>,
) {
    for (_, mut controller) in &mut query.iter_mut() {
        (*controller).forward = match (
            keyboard_input.pressed(KeyCode::W),
            keyboard_input.pressed(KeyCode::S),
        ) {
            (true, false) => 1.0,
            (false, true) => -1.0,
            _ => 0.0,
        };
        (*controller).right = match (
            keyboard_input.pressed(KeyCode::D),
            keyboard_input.pressed(KeyCode::A),
        ) {
            (true, false) => 1.0,
            (false, true) => -1.0,
            _ => 0.0,
        };
        if keyboard_input.just_pressed(KeyCode::C) {
            (*controller).toggle_camera = true;
        }
    }
}
