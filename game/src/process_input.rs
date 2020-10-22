use crate::client::command::FrameCommand;
use crate::scene;
use crate::window_input::input_state::InputState;
use nalgebra_glm::{cross, rotate_vec3, vec3, Vec3};

pub fn process_input(
    input_state: InputState,
    last_frame: Option<u64>,
    current_frame: u64,
    selected_camera: Option<&mut scene::Camera>,
) -> Vec<FrameCommand> {
    let mut commands = Vec::new();
    let last_frame_plus = if let Some(last_frame) = last_frame {
        last_frame + 1
    } else {
        current_frame
    };
    match selected_camera {
        Some(scene::Camera::Follow) => {
            commands.push(FrameCommand {
                command: input_state.clone(),
                frame: last_frame_plus,
            });
            for frame_nr in last_frame_plus + 1..=current_frame {
                commands.push(FrameCommand {
                    command: InputState {
                        movement: input_state.movement.clone(),
                        orientation_change: None,
                    },
                    frame: frame_nr,
                });
            }
        }
        Some(scene::Camera::Freelook {
            position,
            direction,
        }) => {
            if let Some(orientation_change) = &input_state.orientation_change {
                *direction =
                    freelook_rotate(&direction, orientation_change.pitch, orientation_change.yaw);
            }
            if let Some(movement) = &input_state.movement {
                *position = freelook_move(&position, &direction, movement.forward, movement.right);
            }
            for frame_nr in last_frame_plus..=current_frame {
                commands.push(FrameCommand {
                    command: InputState {
                        movement: None,
                        orientation_change: None,
                    },
                    frame: frame_nr,
                });
            }
        }
        None => assert!(false),
    }
    commands
}

pub fn freelook_move(position: &Vec3, direction: &Vec3, forward: f32, right: f32) -> Vec3 {
    let right_vector = cross(&vec3(0.0, 1.0, 0.0), &direction);
    position + direction * forward + right_vector * right
}

pub fn freelook_rotate(direction: &Vec3, updown: f32, around: f32) -> Vec3 {
    let right_vector = cross(&vec3(0.0, 1.0, 0.0), &direction);
    let temp_direction = &rotate_vec3(&direction, around, &vec3(0.0, 1.0, 0.0)).normalize();
    rotate_vec3(&temp_direction, updown, &right_vector).normalize()
}
