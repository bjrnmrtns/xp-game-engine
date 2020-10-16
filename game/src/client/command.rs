use crate::window_input::input_state::InputState;
pub use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct FrameCommand {
    pub command: InputState,
    pub frame: u64,
}
