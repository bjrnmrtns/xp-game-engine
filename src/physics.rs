use nalgebra_glm::*;
use crate::camera;
use crate::commandqueue::*;

pub struct State {
    last_frame_nr: u64,
    pub camera_position: Vec3,
    pub camera_direction: Vec3,
}

impl State {
    pub fn new() -> State {
        State { last_frame_nr: 0, camera_position: vec3(0.0, 0.0, 2.0), camera_direction: vec3(0.0, 0.0, -1.0), }
    }

    fn camera_move(&mut self, forward: i32, right: i32) {
        self.camera_position = camera::move_(forward as f32 / 10.0, right as f32 / 10.0, &self.camera_position, &self.camera_direction);
    }

    fn camera_rotate(&mut self, around_local_x: f32, around_global_y: f32) {
        self.camera_direction = camera::rotate(around_local_x, around_global_y, &self.camera_direction);
    }

    fn handle_frame(&mut self, commands :&[&CommandF], frame_nr: u64) {
        for command in commands {
            match command {
                CommandF { frame: _, command: Command::camera_move(move_) } => {
                    let forward: i32 = move_.forward as i32 - move_.back as i32;
                    let right: i32 = move_.right as i32 - move_.left as i32;
                    self.camera_move(forward, right);
                },
                CommandF { frame: _, command: Command::camera_rotate(rotate) } => {
                    self.camera_rotate(rotate.around_local_x, rotate.around_global_y);
                }
            }
        }
    }

    pub fn run(&mut self, commands: &mut CommandFQueue, frame_nr: u64) {
        for frame_nr in self.last_frame_nr..frame_nr +1 {
            self.handle_frame(&commands.retrieve_commands(frame_nr), frame_nr);
        }
        commands.clear_commands()
    }
}