use crate::window_input::events::{UserInterfaceEvent, UserInterfaceEvents};
use crate::window_input::input_handler::InputHandler;
use crate::window_input::input_state::{InputState, Movement, OrientationChange};
use crate::window_input::{window_event, Position};
use nalgebra_glm::vec2;
use winit::event::{ElementState, Event, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::Window;

pub struct WinitHandler {
    keyboard_state: Vec<bool>,
    ui_events: UserInterfaceEvents,
    ui_mouse_pos: Option<Position>,
    ui_enabled: bool,
    ig_last_retrieved_mouse_pos: Option<Position>,
    ig_current_mouse_pos: Option<Position>,
    quit: bool,
    toggle_camera: u32,
}

impl WinitHandler {
    pub fn new() -> Self {
        Self {
            keyboard_state: vec![false; VirtualKeyCode::Cut as usize + 1],
            ui_events: UserInterfaceEvents::new(),
            ui_mouse_pos: None,
            ui_enabled: false,
            ig_last_retrieved_mouse_pos: None,
            ig_current_mouse_pos: None,
            quit: false,
            toggle_camera: 0,
        }
    }

    fn handle_keyboard_input(&mut self, keyboard_input: &KeyboardInput) {
        match keyboard_input {
            KeyboardInput {
                state,
                virtual_keycode: Some(key_code),
                ..
            } => {
                self.keyboard_state[*key_code as usize] = state == &ElementState::Pressed;
                match (key_code, state) {
                    (VirtualKeyCode::Q, ElementState::Pressed) => self.quit = true,
                    (VirtualKeyCode::C, ElementState::Pressed) => self.toggle_camera += 1,
                    (VirtualKeyCode::Escape, ElementState::Pressed) => {
                        self.ui_enabled = !self.ui_enabled
                    }
                    _ => (),
                }
            }
            _ => (),
        }
    }

    fn handle_cursor_moved(&mut self, x_pos: f32, y_pos: f32) {
        self.ui_mouse_pos = Some(vec2(x_pos, y_pos));
        self.ig_current_mouse_pos = Some(vec2(x_pos, y_pos));
    }

    fn handle_left_click(&mut self) {
        if self.ui_enabled {
            if let Some(mouse_position) = &self.ui_mouse_pos {
                self.ui_events
                    .events
                    .push(UserInterfaceEvent::LeftClick(mouse_position.clone()));
            }
        }
    }

    fn handle_mouse_click(&mut self, state: &ElementState, button: &MouseButton) {
        match (state, button) {
            (ElementState::Pressed, MouseButton::Left) => self.handle_left_click(),
            _ => (),
        }
    }

    pub fn handle_event(
        &mut self,
        event: &Event<()>,
        window: &Window,
    ) -> Option<window_event::WindowEvent> {
        match event {
            Event::MainEventsCleared => {
                window.request_redraw();
                None
            }
            Event::RedrawRequested(_) => Some(window_event::WindowEvent::Redraw),
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == &window.id() => match event {
                #[allow(deprecated)]
                WindowEvent::Resized(physical_size) => Some(window_event::WindowEvent::Resize(
                    physical_size.width,
                    physical_size.height,
                )),
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => Some(
                    window_event::WindowEvent::Resize(new_inner_size.width, new_inner_size.height),
                ),
                _ => {
                    self.handle_window_event(&event);
                    None
                }
            },
            _ => None,
        }
    }

    pub fn handle_window_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::KeyboardInput {
                input: keyboard_input,
                ..
            } => self.handle_keyboard_input(&keyboard_input),
            WindowEvent::CloseRequested => self.quit = true,
            WindowEvent::CursorMoved { position, .. } => {
                self.handle_cursor_moved(position.x as f32, position.y as f32)
            }
            WindowEvent::MouseInput { state, button, .. } => self.handle_mouse_click(state, button),
            _ => (),
        }
    }
}

impl InputHandler for WinitHandler {
    fn is_userinterface_enabled(&self) -> bool {
        self.ui_enabled
    }

    fn get_ui_events(&mut self) -> UserInterfaceEvents {
        let mut ui_events = UserInterfaceEvents::new();
        std::mem::swap(&mut self.ui_events, &mut ui_events);
        ui_events
    }

    fn get_input_state(&mut self) -> InputState {
        let forward = match (
            self.keyboard_state[VirtualKeyCode::W as usize],
            self.keyboard_state[VirtualKeyCode::S as usize],
        ) {
            (true, false) => Some(1.0),
            (false, true) => Some(-1.0),
            _ => None,
        };
        let right = match (
            self.keyboard_state[VirtualKeyCode::D as usize],
            self.keyboard_state[VirtualKeyCode::A as usize],
        ) {
            (true, false) => Some(1.0),
            (false, true) => Some(-1.0),
            _ => None,
        };
        let movement = match (forward, right) {
            (Some(forward), Some(right)) => Some(Movement { forward, right }),
            (Some(forward), None) => Some(Movement {
                forward,
                right: 0.0,
            }),
            (None, Some(right)) => Some(Movement {
                forward: 0.0,
                right,
            }),
            _ => None,
        };
        let orientation_change = match (
            &self.ig_current_mouse_pos,
            &self.ig_last_retrieved_mouse_pos,
        ) {
            (Some(current), Some(last)) => {
                let change = current - last;
                Some(OrientationChange {
                    pitch: change.y / 100.0,
                    yaw: -change.x / 100.0,
                })
            }
            _ => None,
        };
        self.ig_last_retrieved_mouse_pos = self.ig_current_mouse_pos.clone();

        InputState {
            movement,
            orientation_change,
        }
    }

    fn get_camera_toggled(&mut self) -> u32 {
        let result = self.toggle_camera;
        self.toggle_camera = 0;
        result
    }
}
