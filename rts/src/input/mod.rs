mod resources;

pub use crate::input::resources::{CommandEvent, Selection};

use crate::{
    client,
    client::{Command1, Command2, SelectionRender},
    helpers,
};
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
            .add_resource(Selection::default())
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
    mouse_wheel_events: Res<Events<MouseWheel>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut selection: ResMut<Selection>,
    windows: Res<Windows>,
    mut controllers: Query<(
        &mut client::CameraCenterController,
        &mut client::PlayerController,
    )>,
    mut zoom_controllers: Query<&mut client::CameraZoomController>,
    camera: Query<(&GlobalTransform, &Camera)>,
    mut command_events: ResMut<Events<resources::CommandEvent>>,
) {
    let window = windows.get_primary().unwrap();
    let (width, height) = (window.width(), window.height());
    let (border_margin_width, border_margin_height) = (width / 20.0, height / 20.0);

    for (view, camera) in camera.iter() {
        if let Some(current_position) = window.cursor_position() {
            let screen_size = Vec2::from([window.width() as f32, window.height() as f32]);
            let world_position = calculate_world_position_from_screen_position_at_world_height(
                camera.projection_matrix,
                view.compute_matrix(),
                screen_size,
                current_position,
                0.5,
            );
            selection.current_3d_mouse = Some(world_position);
        }
    }

    for (mut camera_center_controller, mut player_controller) in controllers.iter_mut() {
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
            camera_center_controller.move_position = Some(Vec2::new(x, y));
        }
        for event in state.mouse_button_event_reader.iter(&mouse_button_events) {
            match event {
                MouseButtonInput {
                    button: MouseButton::Left,
                    state: ElementState::Pressed,
                } => selection.begin = selection.current_3d_mouse,
                MouseButtonInput {
                    button: MouseButton::Left,
                    state: ElementState::Released,
                } => {
                    if let Some(begin) = selection.begin {
                        player_controller.rectangle_select =
                            Some((begin, selection.current_3d_mouse.unwrap()));
                    }
                    selection.begin = None;
                }
                MouseButtonInput {
                    button: MouseButton::Right,
                    state: ElementState::Pressed,
                } => {
                    command_events.send(resources::CommandEvent::Move(Vec2::new(
                        selection.current_3d_mouse.unwrap().x,
                        selection.current_3d_mouse.unwrap().z,
                    )));
                    player_controller.command2 = Command2::Move(Some(Vec2::new(
                        selection.current_3d_mouse.unwrap().x,
                        selection.current_3d_mouse.unwrap().z,
                    )))
                }
                _ => (),
            }
        }

        if keyboard_input.just_pressed(KeyCode::B) {
            match &player_controller.command1 {
                Command1::Create => player_controller.command1 = Command1::Select,
                Command1::Select => player_controller.command1 = Command1::Create,
            }
        }
    }
    for mut camera_zoom_controller in zoom_controllers.iter_mut() {
        for event in state.mouse_wheel_event_reader.iter(&mouse_wheel_events) {
            match event {
                MouseWheel { y, .. } => {
                    camera_zoom_controller.zoom = Some(*y);
                }
            }
        }
    }
}

fn handle_selection_rendering(
    selection: Res<Selection>,
    mut query: Query<(&SelectionRender, &mut Visible, &mut Transform)>,
) {
    for (_, mut visible, mut transform) in query.iter_mut() {
        if let (Some(selection_begin), Some(selection_current)) =
            (selection.begin, selection.current_3d_mouse)
        {
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
