use nalgebra_glm::*;
use crate::camera;
use crate::commands::Command;
use std::f32::consts::PI;

pub struct Simulation {
    last_hash: u32,
    pub player_position: Vec3,
    pub player_orientation: Quat,
    pub freelook_camera: camera::FreeLook,
}

impl Simulation {
    pub fn new() -> Simulation {
        Simulation { last_hash: 0, player_position: vec3(2.0, 0.0, 0.0), player_orientation: quat_angle_axis( (3.0 / 2.0) * PI, &vec3(0.0, 1.0, 0.0)),
            freelook_camera: camera::FreeLook::new(vec3(0.0, 3.0, 3.0), vec3(0.0, -1.0, -1.0))}
    }

    fn player_move(&mut self, forward: f32, right: f32) {
        // point small vector in same direction as player and add it to player position
        let movement = vec4_to_vec3(&(quat_to_mat4(&self.player_orientation) * vec4(right, 0.0, -forward, 1.0)));
        self.player_position = &self.player_position + movement;
    }

    fn player_rotate(&mut self, around: f32, updown: f32) {
        // most important rule about quaternions is that all rotations are around local quaternion axis, say you rotate around x -> store in quatX, rotate around y -> store in quatY, multiply quatX
        // with quatY -> quatX * quatY, means that the rotation quatY is around local Y of quatX.
        self.player_orientation = &self.player_orientation * quat_angle_axis(around, &vec3(0.0, 1.0, 0.0)) * quat_angle_axis(updown, &vec3(1.0, 0.0, 0.0));
    }

    fn hash_state_now(&mut self) -> u32 {
        let mut hasher = crc32fast::Hasher::new_with_initial(self.last_hash);
        unsafe {
            hasher.update(std::slice::from_raw_parts(self.player_position.as_ptr() as *const u8, self.player_position.len() * 4));
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
                        camera::CameraType::FreeLook => { self.freelook_camera.move_(forward as f32 / 10.0, right as f32 / 10.0); },
                        camera::CameraType::Follow => { self.player_move(forward as f32 / 10.0, right as f32 / 10.0); },
                    }
                },
                Command::CameraRotate(rotate) => {
                    match camera {
                        camera::CameraType::FreeLook => { self.freelook_camera.camera_rotate(rotate.around_local_x as f32 / 100.0, rotate.around_global_y as f32 / 100.0); },
                        camera::CameraType::Follow => { self.player_rotate(rotate.around_global_y as f32 / 100.0, 0.0)},
                    }
                }
            }
        }).collect::<Vec<_>>();
        self.hash_state_now()
    }
}