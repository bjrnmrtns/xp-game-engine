pub use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum StrafeMovement {
    Left,
    Right,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ForwardMovement {
    Positive,
    Negative,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OrientationChange {
    pub horizontal: f32,
    pub vertical: f32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PlayerInputState {
    pub forward: Option<ForwardMovement>,
    pub strafe: Option<StrafeMovement>,
    pub orientation_change: Option<OrientationChange>,
}
