use std::collections::VecDeque;
use crate::window;
use crate::window::{Key, Event};

pub struct CommandCameraMove {
    pub forward: bool,
    pub back: bool,
    pub left: bool,
    pub right: bool,
}

pub struct CommandCameraRotation {
    pub around_local_x: f32,
    pub around_global_y: f32,
}

pub enum Command {
    camera_move(CommandCameraMove),
    camera_rotate(CommandCameraRotation),
}

pub struct CommandF {
    pub frame: u64,
    pub command: Command,
}

impl CommandF {
    pub fn new(frame: u64, command: Command) -> CommandF {
        CommandF { frame: frame, command: command }
    }
}

pub struct CommandFQueue {
    commands: VecDeque<CommandF>,
}

impl CommandFQueue {
    pub fn new() -> CommandFQueue {
        CommandFQueue {
            commands: VecDeque::new()
        }
    }

    fn add(&mut self, frame_count: u64, command: Command) {
        self.commands.push_back(CommandF::new(frame_count, command))
    }

    pub fn command(&mut self) -> Option<CommandF> {
        self.commands.pop_front()
    }

    pub fn handle_input(&mut self, inputs: &mut window::InputQueue, frame_count: u64) {
        self.add(frame_count, Command::camera_move(CommandCameraMove {
            forward: inputs.is_key_down(Key::KeyW),
            back: inputs.is_key_down(Key::KeyS),
            left: inputs.is_key_down(Key::KeyA),
            right: inputs.is_key_down(Key::KeyD),
        }));
        while let Some(event) = inputs.event() {
            match event {
                Event::MouseMotion { x_rel, y_rel } => {
                    self.add(frame_count, Command::camera_rotate(
                        CommandCameraRotation { around_local_x: -y_rel as f32 / 100.0, around_global_y: -x_rel as f32 / 100.0, }
                    ))
                },
                _ => (),
            }
        }
    }
}
