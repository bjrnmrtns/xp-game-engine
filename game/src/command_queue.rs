use crate::client::command::FrameCommand;
use crate::window_input::input_state::InputState;

pub struct CommandQueue {
    last_frame: Option<u64>,
}

impl CommandQueue {
    pub fn new() -> Self {
        Self { last_frame: None }
    }

    pub fn input_to_commands(
        &mut self,
        input_state: InputState,
        current_frame: u64,
    ) -> Vec<FrameCommand> {
        let mut commands = Vec::new();
        let last_frame_plus = if self.last_frame != None {
            self.last_frame.unwrap() + 1
        } else {
            current_frame
        };
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
        self.last_frame = Some(current_frame);
        commands
    }
}
