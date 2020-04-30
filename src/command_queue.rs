use std::collections::VecDeque;
use crate::window;
use crate::window::{Key, Event};
pub use serde::{Serialize, Deserialize};
use crate::commands::{Command, CameraMove, CameraRotation};

pub struct CommandQueue {
    commands: Vec<(u64, Command)>,
    last_frame_nr: u64,
}

impl CommandQueue {
    pub fn new() -> CommandQueue {
        CommandQueue {
            commands: Vec::new(),
            last_frame_nr: 0,
        }
    }

    pub fn handle_input(&mut self, inputs: &mut window::InputQueue, current_frame_nr: u64) -> Vec<(u64, Vec<Command>)> {
        // store always with frame number, and return frames of previous frames when available,
        // so we are sure that it is the complete set
        while let Some(event) = inputs.event() {
            match event {
                Event::MouseMotion { x_rel, y_rel } => {
                    self.commands.push((current_frame_nr, Command::camera_rotate(
                        CameraRotation { around_local_x: -y_rel, around_global_y: -x_rel, }
                    )))
                },
                _ => (),
            }
        }
        let mut frames = Vec::new();
        for frame_nr in self.last_frame_nr..current_frame_nr {
            let mut frame = Vec::new();
            frame.push(Command::camera_move(CameraMove {
                forward: inputs.is_key_down(Key::KeyW),
                back: inputs.is_key_down(Key::KeyS),
                left: inputs.is_key_down(Key::KeyA),
                right: inputs.is_key_down(Key::KeyD),
            }));
            frame.extend(self.commands.iter().filter(|c| c.0 == frame_nr).map(|c| c.1.clone()));
            frames.push((frame_nr, frame));
        }
        self.last_frame_nr = current_frame_nr;
        self.commands.retain(|c| c.0 >= current_frame_nr);
        return frames;
    }
}
