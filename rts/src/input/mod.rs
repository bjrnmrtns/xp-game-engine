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
    mouse_button_event_reader: EventReader<MouseButtonInput>,
    cursor_moved_event_reader: EventReader<CursorMoved>,
}

fn input_system(
    mut state: Local<State>,
    mouse_button_events: Res<Events<MouseButtonInput>>,
    cursor_moved_events: Res<Events<CursorMoved>>,
    mut controllers: Query<&mut client::Controller>,
) {
    let mut current_position: Option<Vec2> = None;
    for event in state.cursor_moved_event_reader.iter(&cursor_moved_events) {
        current_position = Some(event.position);
    }

    for mut controller in controllers.iter_mut() {
        if let Some(current_position) = current_position {
            let x = if current_position.x > 1100.0 {
                1.0
            } else if current_position.x < 100.0 {
                -1.0
            } else {
                0.0
            };
            let y = if current_position.y > 700.0 {
                1.0
            } else if current_position.y < 100.0 {
                -1.0
            } else {
                0.0
            };
            controller.move_position = Some(Vec2::new(x, y));
        }
        for event in state.mouse_button_event_reader.iter(&mouse_button_events) {
            match event {
                MouseButtonInput {
                    button: MouseButton::Left,
                    state: ElementState::Pressed,
                } => (),
                MouseButtonInput {
                    button: MouseButton::Left,
                    state: ElementState::Released,
                } => (),
                _ => (),
            }
        }
    }
}
