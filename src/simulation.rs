use nalgebra_glm::*;
use crate::camera;
use crate::commands::Command;

pub struct Simulation {
    last_hash: u32,
    pub camera_position: Vec3,
    pub camera_direction: Vec3,
    pub player_position: Vec3,
    pub player_direction: Vec3,
}

impl Simulation {
    pub fn new() -> Simulation {
        Simulation { last_hash: 0, camera_position: vec3(-3.0, 3.0, 0.0), camera_direction: vec3(1.0, -1.0, 0.0),
                                   player_position: vec3(0.0, 0.0, 0.0), player_direction: vec3(1.0, 0.0, 0.0), }
    }

    fn camera_move(&mut self, forward: i32, right: i32) {
        self.camera_position = camera::move_(forward as f32 / 10.0, right as f32 / 10.0, &self.camera_position, &self.camera_direction);
    }

    fn camera_rotate(&mut self, around_local_x: f32, around_global_y: f32) {
        self.camera_direction = camera::rotate(around_local_x, around_global_y, &self.camera_direction);
    }

    fn player_move(&mut self, forward: i32, _right: i32) {
        self.player_position = &self.player_position + &self.player_direction * forward as f32 / 10.0;
    }

    fn hash_state_now(&mut self) -> u32 {
        let mut hasher = crc32fast::Hasher::new_with_initial(self.last_hash);
        unsafe {
            hasher.update(std::slice::from_raw_parts(self.camera_position.as_ptr() as *const u8, self.camera_position.len() * 4));
            hasher.update(std::slice::from_raw_parts(self.camera_direction.as_ptr() as *const u8, self.camera_position.len() * 4));
        };
        self.last_hash = hasher.finalize();
        self.last_hash
    }

    pub fn handle_frame(&mut self, commands: &(u64, Vec<Command>), camera: &camera::CameraType) -> u32 {
        let _ = commands.1.iter().map(|command| {
            match &command {
                Command::CameraMove(move_) => {
                    let forward: i32 = move_.forward as i32 - move_.back as i32;
                    let right: i32 = move_.right as i32 - move_.left as i32;
                    match camera {
                        camera::CameraType::FreeLook => { self.camera_move(forward, right); },
                        camera::CameraType::Follow => { self.player_move(forward, right); },
                    }
                },
                Command::CameraRotate(rotate) => {
                    match camera {
                        camera::CameraType::FreeLook => { self.camera_rotate(rotate.around_local_x as f32 / 100.0, rotate.around_global_y as f32 / 100.0); },
                        camera::CameraType::Follow => (),
                    }
                }
            }
        }).collect::<Vec<_>>();
        self.hash_state_now()
    }
}