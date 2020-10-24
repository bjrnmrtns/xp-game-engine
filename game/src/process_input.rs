use crate::window_input::input_state::InputState;
use crate::{scene, simulation, transformation};
use nalgebra_glm::{cross, rotate_vec3, vec3, Vec3};

const ROTATION_SPEED: f32 = 5.0;
const FREE_LOOK_MOVEMENT_SPEED: f32 = 20.0;

pub fn process_input(
    input_state: InputState,
    frames: std::ops::Range<u64>,
    frame_time: f32,
    time_elapsed: std::time::Duration,
    selected_camera: Option<&mut scene::Camera>,
    mut player: &mut scene::Entity,
    frame_input_handler: &mut dyn simulation::FrameInputHandler,
) {
    match selected_camera {
        Some(scene::Camera::Follow) => {
            // orientation change is independent of simulation step
            if let Some(orientation_change) = &input_state.orientation_change {
                if let scene::Entity::Player { pose, .. } = &mut player {
                    pose.orientation = transformation::rotate_around_local_axis(
                        &pose.orientation,
                        0.0,
                        orientation_change.yaw * time_elapsed.as_secs_f32() * ROTATION_SPEED,
                        0.0,
                    )
                }
            }
            for frame_nr in frames {
                frame_input_handler.handle(frame_nr, &input_state, &mut player, frame_time);
            }
        }
        Some(scene::Camera::Freelook {
            position,
            direction,
        }) => {
            if let Some(orientation_change) = &input_state.orientation_change {
                *direction = freelook_rotate(
                    &direction,
                    orientation_change.pitch * time_elapsed.as_secs_f32() * ROTATION_SPEED,
                    orientation_change.yaw * time_elapsed.as_secs_f32() * ROTATION_SPEED,
                );
            }
            if let Some(movement) = &input_state.movement {
                *position = freelook_move(
                    &position,
                    &direction,
                    movement.forward * time_elapsed.as_secs_f32() * FREE_LOOK_MOVEMENT_SPEED,
                    movement.right * time_elapsed.as_secs_f32() * FREE_LOOK_MOVEMENT_SPEED,
                );
            }
        }
        None => assert!(false),
    }
}

pub fn freelook_move(position: &Vec3, direction: &Vec3, forward: f32, right: f32) -> Vec3 {
    let right_vector = cross(&direction, &vec3(0.0, 1.0, 0.0));
    position + direction * forward + right_vector * right
}

pub fn freelook_rotate(direction: &Vec3, updown: f32, around: f32) -> Vec3 {
    let left_vector = cross(&vec3(0.0, 1.0, 0.0), &direction);
    let temp_direction = &rotate_vec3(&direction, around, &vec3(0.0, 1.0, 0.0)).normalize();
    rotate_vec3(&temp_direction, updown, &left_vector).normalize()
}
