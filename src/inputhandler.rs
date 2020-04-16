use crate::window::{Event, Key};
use crate::window;

pub struct InputHandler {

}

impl InputHandler {
    pub fn new() -> InputHandler {
        InputHandler { }
    }

    pub fn handle_input(&mut self, input_queue: &mut window::InputQueue) -> bool {
        while let Some(event) = input_queue.event() {
            match event {
                _ => (),
            }
        }
        true
    }
}