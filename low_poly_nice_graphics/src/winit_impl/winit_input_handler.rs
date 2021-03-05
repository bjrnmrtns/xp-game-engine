use crate::{
    input::{Events, Input, KeyCode, KeyboardInput},
    winit_impl::converters::convert_keyboard_input,
};
use winit::{event::WindowEvent, event_loop::EventLoop};

pub fn handle_input(
    keyboard_events: &mut Events<KeyboardInput>,
    window_event: &winit::event::WindowEvent,
) {
    match window_event {
        WindowEvent::KeyboardInput { ref input, .. } => {
            keyboard_events.send(convert_keyboard_input(input));
        }
        _ => (),
    }
}
