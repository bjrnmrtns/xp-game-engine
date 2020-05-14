use std::collections::{VecDeque};
use winit::event::{VirtualKeyCode, KeyboardInput, ElementState};
use crate::input::Event::KeyEvent;

pub enum Event {
    Quit,
    MouseMotion { x_rel: i32, y_rel: i32 },
    KeyEvent { key: VirtualKeyCode, down: bool },
}

fn convert(k: &VirtualKeyCode) -> usize {
    *k as usize
}

pub struct InputQueue {
    queue: VecDeque<Event>,
    keyboard_state: Vec<bool>,
}

impl InputQueue {
    pub fn new() -> InputQueue {
        InputQueue {
            queue: VecDeque::new(),
            keyboard_state: vec![false; convert(&VirtualKeyCode::Cut) + 1],
        }
    }

    pub fn is_key_down(&self, key: VirtualKeyCode) -> bool {
        self.keyboard_state[convert(&key)]
    }

    pub fn push_keyboard_input(&mut self, input: &KeyboardInput) {
        match input {
            KeyboardInput { state, virtual_keycode: Some(key_code), .. } => {
                self.keyboard_state[convert(key_code)] = state == &ElementState::Pressed;
                self.queue.push_back(KeyEvent { key: *key_code, down: state == &ElementState::Pressed, });
            },
            _ => ()
        }
    }

    pub fn event(&mut self) -> Option<Event> {
        self.queue.pop_front()
    }
}
