use crate::window_input::Position;

#[derive(Clone)]
pub enum UserInterfaceEvent {
    LeftClick(Position),
}

pub struct UserInterfaceEvents {
    pub events: Vec<UserInterfaceEvent>,
}

impl UserInterfaceEvents {
    pub fn new() -> Self {
        Self { events: vec![] }
    }
}
