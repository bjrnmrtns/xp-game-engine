use crate::window::{Window, Event};
use std::collections::VecDeque;

pub struct EventQueue {
    queue: VecDeque<Event>,
}

impl EventQueue {
    pub fn new() -> EventQueue {
        EventQueue { queue: VecDeque::new() }
    }
    pub fn pump(&mut self, window: &Window) -> bool {
        while let Some(input) = window.poll_input() {
            match input {
                Event::Quit => return false,
                _ => self.queue.push_back(input),
            }
        }
        true
    }

    pub fn event(&mut self) -> Option<Event> {
        self.queue.pop_front()
    }
}
