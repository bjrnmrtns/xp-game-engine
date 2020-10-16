pub use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct OrientationChange {
    pub pitch: f32, // left/right
    pub yaw: f32,   // up/down
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Movement {
    pub forward: f32, // value between -1.0 and 1.0
    pub right: f32,   // value between -1.0 and 1.0
}

#[derive(Serialize, Deserialize, Clone)]
pub struct InputState {
    // value between -1.0 and 1.0
    pub movement: Option<Movement>,
    pub orientation_change: Option<OrientationChange>,
}
