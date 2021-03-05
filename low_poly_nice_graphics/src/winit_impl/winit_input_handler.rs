use crate::{
    input::{Events, Input, InputAll, KeyCode, KeyboardInput, MouseScrollUnit, MouseWheelDelta},
    winit_impl::converters::convert_keyboard_input,
};
use winit::{
    event::{MouseScrollDelta, WindowEvent},
    event_loop::EventLoop,
};

pub fn handle_input(input_all: &mut InputAll, window_event: &winit::event::WindowEvent) {
    match window_event {
        WindowEvent::KeyboardInput { ref input, .. } => {
            input_all
                .keyboard_events
                .send(convert_keyboard_input(input));
        }
        WindowEvent::MouseWheel { delta, .. } => match delta {
            winit::event::MouseScrollDelta::LineDelta(x, y) => {
                input_all.mouse_wheel_events.send(MouseWheelDelta {
                    unit: MouseScrollUnit::Line,
                    x: *x,
                    y: *y,
                });
            }
            winit::event::MouseScrollDelta::PixelDelta(delta) => {
                input_all.mouse_wheel_events.send(MouseWheelDelta {
                    unit: MouseScrollUnit::Pixel,
                    x: delta.x as f32,
                    y: delta.y as f32,
                });
            }
        },
        _ => (),
    }
}
