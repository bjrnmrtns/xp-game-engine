mod resources;

pub use crate::input::resources::{CameraViewEvent, CommandEvent, InputState};

use crate::{client, client::SelectionRender, helpers};
use bevy::{
    input::{
        mouse::{MouseButtonInput, MouseWheel},
        system::exit_on_esc_system,
        ElementState,
    },
    prelude::*,
    render::camera::Camera,
};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<resources::CommandEvent>()
            .add_event::<resources::CameraViewEvent>()
            .add_resource(InputState::default())
            .add_system(input_system.system())
            .add_system(exit_on_esc_system.system())
            .add_system(handle_selection_rendering.system());
    }
}

#[derive(Default)]
struct State {
    mouse_button_event_reader: EventReader<MouseButtonInput>,
    mouse_wheel_event_reader: EventReader<MouseWheel>,
}

fn cursor_position_to_world(
    projection: Mat4,
    view: Mat4,
    screen_size: Vec2,
    screen_coordinate: Vec2,
) -> Vec2 {
    const HEIGHT: f32 = 0.0;
    let cursor_pos_ndc: Vec3 =
        ((screen_coordinate / screen_size) * 2.0 - Vec2::from([1.0, 1.0])).extend(1.0);

    let (_, _, camera_position) = view.to_scale_rotation_translation();

    let ndc_to_world = view * projection.inverse();
    let cursor_position = ndc_to_world.transform_point3(cursor_pos_ndc);
    let direction = cursor_position - camera_position;
    let lambda = (HEIGHT - cursor_position.y) / direction.y;
    let world_3d = cursor_position + direction * lambda;
    Vec2::new(world_3d.x, world_3d.z)
}

fn input_system(
    mut state: Local<State>,
    game_info: Res<client::GameInfo>,
    mouse_button_events: Res<Events<MouseButtonInput>>,
    mouse_wheel_events: Res<Events<MouseWheel>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut input_state: ResMut<InputState>,
    windows: Res<Windows>,
    camera_query: Query<(&GlobalTransform, &Camera)>,
    mut command_events: ResMut<Events<resources::CommandEvent>>,
    mut camera_view_events: ResMut<Events<resources::CameraViewEvent>>,
) {
    let window = windows.get_primary().unwrap();
    let (width, height) = (window.width(), window.height());
    let (border_margin_width, border_margin_height) = (width / 20.0, height / 20.0);
    let screen_size = Vec2::from([window.width() as f32, window.height() as f32]);

    if let (Some(cursor_position), Some(camera_entity)) =
        (window.cursor_position(), game_info.camera)
    {
        let (view, camera) = camera_query.get(camera_entity).unwrap();
        input_state.world_mouse_position = cursor_position_to_world(
            camera.projection_matrix,
            view.compute_matrix(),
            screen_size,
            cursor_position,
        );
        if let Some(current_position) = window.cursor_position() {
            let x = if current_position.x > width - border_margin_width {
                1.0
            } else if current_position.x < border_margin_width {
                -1.0
            } else {
                0.0
            };
            let y = if current_position.y > height - border_margin_height {
                -1.0
            } else if current_position.y < border_margin_height {
                1.0
            } else {
                0.0
            };
            camera_view_events.send(CameraViewEvent::CameraMove(Vec2::new(x, y)));
        }

        for event in state.mouse_button_event_reader.iter(&mouse_button_events) {
            match (event, keyboard_input.pressed(KeyCode::LControl)) {
                (
                    MouseButtonInput {
                        button: MouseButton::Left,
                        state: ElementState::Pressed,
                    },
                    true,
                ) => {
                    input_state.last_selection_begin = Some(input_state.world_mouse_position);
                }
                (
                    MouseButtonInput {
                        button: MouseButton::Left,
                        state: ElementState::Pressed,
                    },
                    false,
                ) => command_events.send(CommandEvent::Create(input_state.world_mouse_position)),
                (
                    MouseButtonInput {
                        button: MouseButton::Left,
                        state: ElementState::Released,
                    },
                    _,
                ) => {
                    if let Some(selection_begin) = input_state.last_selection_begin {
                        let (low, high) = helpers::calculate_low_high(
                            selection_begin,
                            input_state.world_mouse_position,
                        );
                        command_events.send(CommandEvent::Select(low, high));
                        input_state.last_selection_begin = None;
                    }
                }
                (
                    MouseButtonInput {
                        button: MouseButton::Right,
                        state: ElementState::Pressed,
                    },
                    _,
                ) => {
                    command_events.send(resources::CommandEvent::Move(
                        input_state.world_mouse_position,
                    ));
                }
                _ => (),
            }
        }
    }
    for event in state.mouse_wheel_event_reader.iter(&mouse_wheel_events) {
        match event {
            MouseWheel { y, .. } => camera_view_events.send(CameraViewEvent::Zoom(*y)),
        }
    }
}

fn handle_selection_rendering(
    selection: Res<InputState>,
    mut query: Query<(&SelectionRender, &mut Visible, &mut Transform)>,
) {
    for (_, mut visible, mut transform) in query.iter_mut() {
        if let (Some(selection_begin), selection_current) = (
            selection.last_selection_begin,
            selection.world_mouse_position,
        ) {
            let (midpoint, scale) =
                helpers::calculate_midpoint_scale(selection_begin, selection_current);
            transform.translation = Vec3::new(midpoint.x, 0.5, midpoint.y);
            transform.scale = Vec3::new(scale.x, 1.0, scale.y);
            visible.is_visible = true;
        } else {
            visible.is_visible = false;
        }
    }
}
