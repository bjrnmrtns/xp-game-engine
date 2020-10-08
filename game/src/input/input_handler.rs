use crate::input::player_input_state::PlayerInputState;

pub trait InputHandler {
    fn state(&mut self) -> PlayerInputState;
}
