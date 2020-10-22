use crate::window_input::events::UserInterfaceEvents;
use crate::window_input::input_state::InputState;

pub trait InputHandler {
    fn is_userinterface_enabled(&self) -> bool;
    fn get_input_state(&mut self) -> InputState;
    fn get_camera_toggled(&mut self) -> u32;
    fn get_ui_events(&mut self) -> UserInterfaceEvents;
    fn quit(&self) -> bool;
}
