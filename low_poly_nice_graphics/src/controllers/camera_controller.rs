use crate::input::{Events, MouseMotion, MouseScrollUnit, MouseWheelDelta};

#[derive(Default)]
pub struct CameraController {
    pub zoom: f32,
    pub vertical_angle_update: f32,
}

impl CameraController {
    pub fn mouse_handling(
        &mut self,
        mouse_wheel_events: &Events<MouseWheelDelta>,
        mouse_motion_events: &Events<MouseMotion>,
    ) {
        self.zoom = 0.0;
        self.vertical_angle_update = 0.0;
        for event in mouse_wheel_events.values() {
            match event.unit {
                MouseScrollUnit::Line => {
                    self.zoom += event.y / 2.0;
                }
                // Touch scroll on MacOS behaves opposite to wheel scroll and is pixel based
                MouseScrollUnit::Pixel => {
                    self.zoom -= event.y / 2.0;
                }
            }
        }
        for event in mouse_motion_events.values() {
            self.vertical_angle_update += event.delta.y;
        }
    }
}
