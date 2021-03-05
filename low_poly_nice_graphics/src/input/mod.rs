mod event;
mod input;
mod keyboard;
mod mouse;

pub use event::Events;
pub use input::Input;
pub use keyboard::{keyboard_state_from_events, ElementState, KeyCode, KeyboardInput};
pub use mouse::{MouseScrollUnit, MouseWheelDelta};

#[derive(Default)]
pub struct InputAll {
    pub keyboard_input: Input<KeyCode>,
    pub keyboard_events: Events<KeyboardInput>,
    pub mouse_wheel_events: Events<MouseWheelDelta>,
}

impl InputAll {
    pub fn clear_events(&mut self) {
        self.keyboard_events.clear();
        self.mouse_wheel_events.clear();
    }
}
