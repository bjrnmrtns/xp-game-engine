use crate::client::command::FrameCommand;
use crate::entity;
use crate::graphics::clipmap;

pub fn handle_frame(
    frame_commands: Vec<FrameCommand>,
    player: &mut entity::Entity,
    frame_time: f32,
    clipmap_renderer: &clipmap::Renderable,
) {
    for frame_command in frame_commands {
        player.handle_frame(frame_command, frame_time, &clipmap_renderer);
    }
}
