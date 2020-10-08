use crate::client::command::FrameCommand;
use crate::entity;
use crate::terrain::Generator;

pub fn handle_frame(
    frame_commands: Vec<FrameCommand>,
    player: &mut entity::Entity,
    frame_time: f32,
    generator: &dyn Generator,
) {
    for frame_command in frame_commands {
        player.handle_frame(frame_command, frame_time, &*generator);
    }
}
