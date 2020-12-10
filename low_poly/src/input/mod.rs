use crate::client;
use bevy::input::mouse::{MouseButtonInput, MouseMotion};
use bevy::input::ElementState;
use bevy::prelude::*;
use std::ops::DerefMut;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(input_system.system());
    }
}

#[derive(Default)]
struct State {
    mouse_motion_event_reader: EventReader<MouseMotion>,
    mouse_button_event_reader: EventReader<MouseButtonInput>,
}

fn input_system(
    mut state: Local<State>,
    mouse_button_events: Res<Events<MouseButtonInput>>,
    mouse_motion_events: Res<Events<MouseMotion>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query_characters: Query<&mut client::CharacterController>,
    mut query_cameras: Query<&mut client::CameraController>,
) {
    let mut delta = Vec2::zero();
    for event in state.mouse_motion_event_reader.iter(&mouse_motion_events) {
        delta += event.delta;
    }
    for mut camera_controller in query_cameras.iter_mut() {
        camera_controller.deref_mut().rotate_x = delta.y;
    }
    for mut character_controller in query_characters.iter_mut() {
        character_controller.deref_mut().rotate_y = -delta.x;
        character_controller.deref_mut().move_forward = Some(
            match (
                keyboard_input.pressed(KeyCode::W),
                keyboard_input.pressed(KeyCode::S),
            ) {
                (true, false) => 1.0,
                (false, true) => -1.0,
                _ => 0.0,
            },
        );
        character_controller.deref_mut().strafe_right = Some(
            match (
                keyboard_input.pressed(KeyCode::D),
                keyboard_input.pressed(KeyCode::A),
            ) {
                (true, false) => 1.0,
                (false, true) => -1.0,
                _ => 0.0,
            },
        );
        character_controller.jump = keyboard_input.just_pressed(KeyCode::Space);
        for event in state.mouse_button_event_reader.iter(&mouse_button_events) {
            match event {
                MouseButtonInput {
                    button: MouseButton::Left,
                    state: ElementState::Pressed,
                } => character_controller.place_object = true,
                MouseButtonInput {
                    button: MouseButton::Left,
                    state: ElementState::Released,
                } => character_controller.place_object = false,
                _ => (),
            }
        }
    }
}
