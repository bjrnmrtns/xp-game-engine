pub mod events;
pub mod input_handler;
pub mod input_state;
pub mod window_event;

use nalgebra_glm::Vec2;

pub use input_handler::InputHandler;
pub use window_event::WindowEvent;

pub type Position = Vec2;
