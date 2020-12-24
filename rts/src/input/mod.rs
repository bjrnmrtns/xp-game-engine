use crate::{client, client::Action};
use bevy::{
    input::{
        mouse::{MouseButtonInput, MouseMotion},
        system::exit_on_esc_system,
        ElementState,
    },
    prelude::*,
};
use std::ops::DerefMut;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(input_system.system())
            .add_system(exit_on_esc_system.system());
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
    mut controllers: Query<&mut client::Controller>,
) {
    let mut delta = Vec2::zero();
    for event in state.mouse_motion_event_reader.iter(&mouse_motion_events) {
        delta += event.delta;
    }
    for mut controller in controllers.iter_mut() {
        controller.deref_mut().move_forward = Some(
            match (
                keyboard_input.pressed(KeyCode::W),
                keyboard_input.pressed(KeyCode::S),
            ) {
                (true, false) => 1.0,
                (false, true) => -1.0,
                _ => 0.0,
            },
        );
        controller.deref_mut().strafe_right = Some(
            match (
                keyboard_input.pressed(KeyCode::D),
                keyboard_input.pressed(KeyCode::A),
            ) {
                (true, false) => 1.0,
                (false, true) => -1.0,
                _ => 0.0,
            },
        );
        for event in state.mouse_button_event_reader.iter(&mouse_button_events) {
            match event {
                MouseButtonInput {
                    button: MouseButton::Left,
                    state: ElementState::Pressed,
                } => controller.action_enabled = true,
                MouseButtonInput {
                    button: MouseButton::Left,
                    state: ElementState::Released,
                } => controller.action_enabled = false,
                _ => (),
            }
        }
        if keyboard_input.just_pressed(KeyCode::B) {
            match controller.action {
                Action::Add => controller.action = Action::Remove,
                Action::Remove => controller.action = Action::Add,
            }
        }
    }
}
