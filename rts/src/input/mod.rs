use crate::client;
use bevy::{
    input::{mouse::MouseButtonInput, system::exit_on_esc_system, ElementState},
    prelude::*,
    render::camera::Camera,
};

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
}

fn calculate_world_position_from_screen_position_at_world_height(
    projection: Mat4,
    view: Mat4,
    screen_size: Vec2,
    screen_coordinate: Vec2,
    y: f32,
) -> Vec3 {
    let cursor_pos_ndc: Vec3 =
        ((screen_coordinate / screen_size) * 2.0 - Vec2::from([1.0, 1.0])).extend(1.0);

    let (_, _, camera_position) = view.to_scale_rotation_translation();

    let ndc_to_world = view * projection.inverse();
    let cursor_position = ndc_to_world.transform_point3(cursor_pos_ndc);
    let direction = cursor_position - camera_position;
    let lambda = (y - cursor_position.y) / direction.y;
    cursor_position + direction * lambda
}

fn input_system(
    mut state: Local<State>,
    mouse_button_events: Res<Events<MouseButtonInput>>,
    windows: Res<Windows>,
    mut controllers: Query<(&mut client::CameraController, &mut client::PlayerController)>,
    camera: Query<(&GlobalTransform, &Camera)>,
) {
    let window = windows.get_primary().unwrap();
    let (width, height) = (window.width(), window.height());
    let (border_margin_width, border_margin_height) = (width / 10.0, height / 10.0);

    let mut place_object: Option<Vec3> = None;
    for (view, camera) in camera.iter() {
        if let Some(current_position) = window.cursor_position() {
            let screen_size = Vec2::from([window.width() as f32, window.height() as f32]);
            let world_position = calculate_world_position_from_screen_position_at_world_height(
                camera.projection_matrix,
                view.compute_matrix(),
                screen_size,
                current_position,
                0.0,
            );
            place_object = Some(world_position);
        }
    }

    for (mut camera_controller, mut player_controller) in controllers.iter_mut() {
        if let Some(current_position) = window.cursor_position() {
            let x = if current_position.x > width - border_margin_width {
                1.0
            } else if current_position.x < border_margin_width {
                -1.0
            } else {
                0.0
            };
            let y = if current_position.y > height - border_margin_height {
                1.0
            } else if current_position.y < border_margin_height {
                -1.0
            } else {
                0.0
            };
            camera_controller.move_position = Some(Vec2::new(x, y));
        }
        for event in state.mouse_button_event_reader.iter(&mouse_button_events) {
            match event {
                MouseButtonInput {
                    button: MouseButton::Left,
                    state: ElementState::Pressed,
                } => player_controller.place_object = place_object,
                MouseButtonInput {
                    button: MouseButton::Left,
                    state: ElementState::Released,
                } => (),
                _ => (),
            }
        }
    }
}
