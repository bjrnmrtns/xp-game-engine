use crate::client;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use std::ops::DerefMut;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(mouse_motion_system.system())
            .add_system(keyboard_input_system.system());
    }
}

#[derive(Default)]
struct State {
    mouse_motion_event_reader: EventReader<MouseMotion>,
}

fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut controllable_entities: ResMut<client::ControllableEntities>,
    mut query: Query<&mut client::CharacterController>,
) {
    if let Some(entity) = controllable_entities.get_selected() {
        let mut entity_controller = query.get_mut(entity).unwrap();
        entity_controller.deref_mut().move_forward = Some(
            match (
                keyboard_input.pressed(KeyCode::W),
                keyboard_input.pressed(KeyCode::S),
            ) {
                (true, false) => 1.0,
                (false, true) => -1.0,
                _ => 0.0,
            },
        );
        entity_controller.deref_mut().strafe_right = Some(
            match (
                keyboard_input.pressed(KeyCode::D),
                keyboard_input.pressed(KeyCode::A),
            ) {
                (true, false) => 1.0,
                (false, true) => -1.0,
                _ => 0.0,
            },
        );
    }
    if keyboard_input.just_pressed(KeyCode::P) {
        controllable_entities.toggle();
    }
}

fn mouse_motion_system(mut state: Local<State>, mouse_motion_events: Res<Events<MouseMotion>>) {
    let mut delta = Vec2::zero();
    for event in state.mouse_motion_event_reader.iter(&mouse_motion_events) {
        delta += event.delta;
    }
}
