use crate::camera;
use crate::client;
use bevy::prelude::*;
use std::ops::DerefMut;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(input_system.system());
    }
}

fn input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut controllable_cameras: ResMut<camera::FreelookCameras>,
    mut controllable_entities: ResMut<client::ControllableEntities>,
    mut query: Query<&mut client::EntityController>,
) {
    if let Some(entity) = controllable_entities.get_selected() {
        let forward = match (
            keyboard_input.pressed(KeyCode::W),
            keyboard_input.pressed(KeyCode::S),
        ) {
            (true, false) => 1.0,
            (false, true) => -1.0,
            _ => 0.0,
        };
        let right = match (
            keyboard_input.pressed(KeyCode::D),
            keyboard_input.pressed(KeyCode::A),
        ) {
            (true, false) => 1.0,
            (false, true) => -1.0,
            _ => 0.0,
        };
        let mut entity_controller = query.get_mut(entity).unwrap();
        entity_controller
            .deref_mut()
            .move_(Transform::from_translation(Vec3::new(forward, 0.0, right)));
    }
    if keyboard_input.just_pressed(KeyCode::C) {
        controllable_cameras.toggle();
    }
    if keyboard_input.just_pressed(KeyCode::P) {
        controllable_entities.toggle();
    }
}
