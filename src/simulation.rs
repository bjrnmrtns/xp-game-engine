use nalgebra_glm::*;
use crate::camera;
use crate::commandqueue::*;
use crate::commands::Command;

pub struct Simulation {
    pub camera_position: Vec3,
    pub camera_direction: Vec3,
}

impl Simulation {
    pub fn new() -> Simulation {
        Simulation { camera_position: vec3(0.0, 0.0, 2.0), camera_direction: vec3(0.0, 0.0, -1.0), }
    }

    fn camera_move(&mut self, forward: i32, right: i32) {
        self.camera_position = camera::move_(forward as f32 / 10.0, right as f32 / 10.0, &self.camera_position, &self.camera_direction);
    }

    fn camera_rotate(&mut self, around_local_x: f32, around_global_y: f32) {
        self.camera_direction = camera::rotate(around_local_x, around_global_y, &self.camera_direction);
    }

    fn handle_frame(&mut self, commands: &(u64, Vec<Command>)) {
        commands.1.iter().map(|command| {
            match &command {
                Command::camera_move(move_) => {
                    let forward: i32 = move_.forward as i32 - move_.back as i32;
                    let right: i32 = move_.right as i32 - move_.left as i32;
                    self.camera_move(forward, right);
                },
                Command::camera_rotate(rotate) => {
                    self.camera_rotate(rotate.around_local_x as f32 / 100.0, rotate.around_global_y as f32 / 100.0);
                }
            }
        }).collect::<Vec<_>>();
    }

    pub fn run(&mut self, commands: &[(u64, Vec<Command>)], recorder: &mut std::io::Write) {
        for frame in commands {
            self.handle_frame(frame);
        }
    }
}