use crate::client::command::FrameCommand;
use crate::scene;
use crate::window_input::input_state::InputState;

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
            for frame_nr in last_frame_plus..=current_frame {
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
