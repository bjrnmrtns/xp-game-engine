pub mod events;
pub mod input_handler;
pub mod input_state;
pub mod window_handler;
pub mod window_state;

use crate::window_input::events::UserInterfaceEvents;
use crate::window_input::input_state::InputState;
use nalgebra_glm::Vec2;

pub type Position = Vec2;
