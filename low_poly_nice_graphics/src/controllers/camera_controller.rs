use crate::input::{Events, MouseScrollUnit, MouseWheelDelta};

#[derive(Default)]
pub struct CameraController {
    pub zoom: f32,
}

impl CameraController {
    pub fn mouse_wheel(&mut self, mouse_wheel_events: &Events<MouseWheelDelta>) {
        self.zoom = 0.0;
        for event in mouse_wheel_events.values() {
            match event.unit {
                _ => {
                    self.zoom += event.y / 2.0;
                }
            }
        }
    }
}
