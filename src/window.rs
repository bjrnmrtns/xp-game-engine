use std::collections::{VecDeque};

pub enum Event {
    Quit,
    MouseMotion { x_rel: i32, y_rel: i32 },
    KeyEvent { key: Key, down: bool },
}

#[repr(i32)]
#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum  Key {
    KeyW,
    KeyA,
    KeyS,
    KeyD,
    KeyEscape,
}

fn convert(k: &Key) -> usize {
    *k as usize
}

pub trait Window {
    fn update(&self);
    fn poll_input(&self) -> Option<Event>;
}

pub struct InputQueue {
    queue: VecDeque<Event>,
    keyboard_state: Vec<bool>,
}

impl InputQueue {
    pub fn new() -> InputQueue {
        InputQueue {
            queue: VecDeque::new(),
            keyboard_state: vec![false; convert(&Key::KeyEscape) + 1],
        }
    }

    pub fn is_key_down(&self, key: Key) -> bool {
        self.keyboard_state[convert(&key)]
    }

    pub fn pump(&mut self, window: &Window) -> bool {
        while let Some(input) = window.poll_input() {
            match input {
                Event::Quit => return false,
                Event::KeyEvent { key, down } => {
                    self.keyboard_state[convert(&key)] = down;
                    self.queue.push_back(input)
                },
                _ => self.queue.push_back(input),
            }
        }
        true
    }

    pub fn event(&mut self) -> Option<Event> {
        self.queue.pop_front()
    }
}

