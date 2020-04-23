use std::collections::VecDeque;
use crate::window;
use crate::window::{Key, Event};
pub use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct CommandCameraMove {
    pub forward: bool,
    pub back: bool,
    pub left: bool,
    pub right: bool,
}

#[derive(Serialize, Deserialize)]
pub struct CommandCameraRotation {
    pub around_local_x: f32,
    pub around_global_y: f32,
}

#[derive(Serialize, Deserialize)]
pub enum Command {
    camera_move(CommandCameraMove),
    camera_rotate(CommandCameraRotation),
}

#[derive(Serialize, Deserialize)]
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
        self.add(frame_nr, Command::camera_move(CommandCameraMove {
            forward: inputs.is_key_down(Key::KeyW),
            back: inputs.is_key_down(Key::KeyS),
            left: inputs.is_key_down(Key::KeyA),
            right: inputs.is_key_down(Key::KeyD),
        }));
        while let Some(event) = inputs.event() {
            match event {
                Event::MouseMotion { x_rel, y_rel } => {
                    self.add(frame_nr, Command::camera_rotate(
                        CommandCameraRotation { around_local_x: -y_rel as f32 / 100.0, around_global_y: -x_rel as f32 / 100.0, }
                    ))
                },
                _ => (),
            }
        }
    }
}
