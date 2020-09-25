use crate::commands::Command;
use crate::entity;

pub struct Simulation;

impl Simulation {
    pub fn handle_frame(
        &mut self,
        commands: &(u64, Vec<Command>),
        player: &mut entity::Entity,
        frame_time: f32,
    ) {
        player.fall_velocity -= 9.81 * frame_time;
        player.pose.position.y += player.fall_velocity * frame_time;
        if player.pose.position.y < 0.0 {
            player.pose.position.y = 0.0
        };
        let _ = commands
            .1
            .iter()
            .map(|command| match &command {
                Command::CameraMove(move_) => {
                    let forward: i32 = move_.forward as i32 - move_.back as i32;
                    let right: i32 = move_.right as i32 - move_.left as i32;
                    player.move_(forward as f32 / 10.0, right as f32 / 10.0)
                }
                Command::CameraRotate(rotate) => {
                    player.orient(rotate.around_global_y as f32 / 100.0)
                }
            })
            .collect::<Vec<_>>();
    }
}
