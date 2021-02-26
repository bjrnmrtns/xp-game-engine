mod input;
mod keyboard;

pub use input::Input;
pub use keyboard::KeyCode;

#[derive(Default)]
pub struct InputState {
    pub keyboard: Input<KeyCode>,
}
