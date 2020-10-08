use crate::input::PlayerInputState;
pub use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct FrameCommand {
    pub command: PlayerInputState,
    pub frame: u64,
}
