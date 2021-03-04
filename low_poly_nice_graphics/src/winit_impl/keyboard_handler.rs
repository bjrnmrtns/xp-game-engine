use crate::{
    input::{Input, KeyCode},
    winit_impl::converters::convert_virtual_keycode,
};
use winit::event_loop::EventLoop;

pub fn keyboard_handler(
    keyboard_input_state: &mut Input<KeyCode>,
    input: &winit::event::KeyboardInput,
) {
    if let &winit::event::KeyboardInput {
        virtual_keycode: Some(virtual_keycode),
        state,
        ..
    } = input
    {
        match state {
            winit::event::ElementState::Pressed => {
                keyboard_input_state.press(convert_virtual_keycode(virtual_keycode))
            }
            winit::event::ElementState::Released => {
                keyboard_input_state.release(convert_virtual_keycode(virtual_keycode))
            }
        }
    }
}
