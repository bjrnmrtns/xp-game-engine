use std::collections::VecDeque;
use crate::window;
use crate::window::{Key, Event};
pub use serde::{Serialize, Deserialize};
use crate::commands::{Command, CameraMove, CameraRotation};

pub struct CommandQueue {
    commands: Vec<(u64, Command)>,
}

impl CommandQueue {
    pub fn new() -> CommandQueue {
        CommandQueue {
            commands: Vec::new(),
        }
    }

    fn add(&mut self, frame_nr: u64, command: Command) {
        self.commands.push((frame_nr, command))
    }

    pub fn clear_commands_until_frame(&mut self, frame_nr: u64) {
        self.commands.retain(|c| c.0 > frame_nr);
    }

    pub fn retrieve_commands(&self, frame_nr: u64) -> Vec<&Command> {
        self.commands.iter().filter(|c| c.0 == frame_nr).map(|c| &c.1 ).collect()
    }

    pub fn handle_input(&mut self, inputs: &mut window::InputQueue, frame_nr: u64) {
        self.add(frame_nr, Command::camera_move(CameraMove {
            forward: inputs.is_key_down(Key::KeyW),
            back: inputs.is_key_down(Key::KeyS),
            left: inputs.is_key_down(Key::KeyA),
            right: inputs.is_key_down(Key::KeyD),
        }));
        while let Some(event) = inputs.event() {
            match event {
                Event::MouseMotion { x_rel, y_rel } => {
                    self.add(frame_nr, Command::camera_rotate(
                        CameraRotation { around_local_x: -y_rel, around_global_y: -x_rel, }
                    ))
                },
                _ => (),
            }
        }
    }
}
