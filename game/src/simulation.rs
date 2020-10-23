use crate::client::command::FrameCommand;
use crate::scene;
use crate::transformation;

pub fn handle_frame(
    frame_commands: Vec<FrameCommand>,
    player: &mut scene::Entity,
    frame_time: f32,
) {
    for frame_command in frame_commands {
        if let scene::Entity::Player { pose, max_velocity } = player {
            if let Some(orientation_change) = &frame_command.command.orientation_change {
                pose.orientation = transformation::rotate_around_local_axis(
                    &pose.orientation,
                    0.0,
                    orientation_change.yaw,
                    0.0,
                )
            }
            if let Some(movement) = &frame_command.command.movement {
                let forward = frame_time * *max_velocity * movement.forward;
                let right = frame_time * *max_velocity * movement.right;
                let movement =
                    transformation::move_along_local_axis(&pose.orientation, forward, right, 0.0);
                pose.position += movement;
            }
        }
    }
}
