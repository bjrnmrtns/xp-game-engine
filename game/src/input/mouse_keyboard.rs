use crate::input::input_handler::InputHandler;
use crate::input::player_input_state::{ForwardMovement, OrientationChange, StrafeMovement};
use crate::input::PlayerInputState;
use winit::event::{ElementState, KeyboardInput, VirtualKeyCode};

pub struct MouseKeyboardInputHandler {
    key_w: bool,
    key_a: bool,
    key_s: bool,
    key_d: bool,
    x_accumulated: f64,
    y_accumulated: f64,
}

impl MouseKeyboardInputHandler {
    pub fn new() -> Self {
        Self {
            key_w: false,
            key_a: false,
            key_s: false,
            key_d: false,
            x_accumulated: 0.0,
            y_accumulated: 0.0,
        }
    }
    pub fn handle_keyboard(&mut self, keyboard_input: &KeyboardInput) {
        match keyboard_input.virtual_keycode {
            Some(VirtualKeyCode::W) => self.key_w = keyboard_input.state == ElementState::Pressed,
            Some(VirtualKeyCode::A) => self.key_a = keyboard_input.state == ElementState::Pressed,
            Some(VirtualKeyCode::S) => self.key_s = keyboard_input.state == ElementState::Pressed,
            Some(VirtualKeyCode::D) => self.key_d = keyboard_input.state == ElementState::Pressed,
            _ => (),
        }
    }
    pub fn handle_mouse(&mut self, delta: &(f64, f64)) {
        self.x_accumulated += delta.0;
        self.y_accumulated += delta.1;
    }
}

impl InputHandler for MouseKeyboardInputHandler {
    fn state(&mut self) -> PlayerInputState {
        let mut state = PlayerInputState {
            forward: None,
            strafe: None,
            orientation_change: Some(OrientationChange {
                horizontal: (-self.x_accumulated / 100.0) as f32,
                vertical: (-self.y_accumulated / 100.0) as f32,
            }),
        };
        state.forward = match (self.key_w, self.key_s) {
            (true, false) => Some(ForwardMovement::Positive),
            (false, true) => Some(ForwardMovement::Negative),
            (_, _) => None,
        };
        state.strafe = match (self.key_a, self.key_d) {
            (true, false) => Some(StrafeMovement::Left),
            (false, true) => Some(StrafeMovement::Right),
            (_, _) => None,
        };
        self.x_accumulated = 0.0;
        self.y_accumulated = 0.0;
        state
    }
}
