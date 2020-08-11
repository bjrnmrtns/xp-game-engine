use nalgebra_glm::*;
use crate::{camera, entity};
use crate::commands::Command;


pub struct Simulation {
    last_hash: u32,
    pub freelook_camera: camera::FreeLook,
    pub freelook_camera2: camera::FreeLook,
}

impl Simulation {
    pub fn new() -> Simulation {
        let freelook_camera2 = camera::FreeLook::new(vec3(0.0, 3.0, 3.0), vec3(0.0, -1.0, -1.0));
        Simulation { last_hash: 0, freelook_camera: camera::FreeLook::new(vec3(0.0, 3.0, 3.0), vec3(0.0, -1.0, -1.0)), freelook_camera2,}
    }

    fn hash_state_now(&mut self) -> u32 {
/*        let mut hasher = crc32fast::Hasher::new_with_initial(self.last_hash);
        unsafe {
            hasher.update(std::slice::from_raw_parts(self.player_position.as_ptr() as *const u8, self.player_position.len() * 4));
        };
        self.last_hash = hasher.finalize();*/
        self.last_hash
    }

    pub fn handle_frame(&mut self, commands: &(u64, Vec<Command>), camera: &camera::CameraType, player: &mut entity::Entity) -> u32 {
        let _ = commands.1.iter().map(|command| {
            match &command {
                Command::CameraMove(move_) => {
                    let forward: i32 = move_.forward as i32 - move_.back as i32;
                    let right: i32 = move_.right as i32 - move_.left as i32;
                    match camera {
                        camera::CameraType::FreeLook => { self.freelook_camera.move_(forward as f32 / 10.0, right as f32 / 10.0); },
                        camera::CameraType::FreeLook2 => { self.freelook_camera2.move_(forward as f32 / 10.0, right as f32 / 10.0); },
                        camera::CameraType::Follow => { player.move_(forward as f32 / 10.0, right as f32 / 10.0); },
                    }
                },
                Command::CameraRotate(rotate) => {
                    match camera {
                        camera::CameraType::FreeLook => { self.freelook_camera.camera_rotate(rotate.around_local_x as f32 / 100.0, rotate.around_global_y as f32 / 100.0); },
                        camera::CameraType::FreeLook2 => { self.freelook_camera2.camera_rotate(rotate.around_local_x as f32 / 100.0, rotate.around_global_y as f32 / 100.0); },

                        camera::CameraType::Follow => { player.orient(rotate.around_global_y as f32 / 100.0)},
                    }
                }
            }
        }).collect::<Vec<_>>();
        self.hash_state_now()
    }
}