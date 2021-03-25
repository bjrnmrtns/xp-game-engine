use crate::{
    input::{InputAll, MouseMotion, MouseScrollUnit, MouseWheelDelta},
    winit_impl::converters::convert_keyboard_input,
};
use glam::Vec2;
use winit::event::{DeviceEvent, WindowEvent};

pub fn handle_input(input_all: &mut InputAll, event: &winit::event::Event<()>) {
    match event {
        winit::event::Event::WindowEvent { event, .. } => match event {
            WindowEvent::KeyboardInput { ref input, .. } => {
                input_all.keyboard_events.send(convert_keyboard_input(input));
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
        },
        winit::event::Event::DeviceEvent {
            event: DeviceEvent::MouseMotion { delta },
            ..
        } => input_all.mouse_motion_events.send(MouseMotion {
            delta: Vec2::new(delta.0 as f32, delta.1 as f32),
        }),
        _ => (),
    }
}
