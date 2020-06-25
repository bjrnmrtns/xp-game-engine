use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct CameraMove {
    pub forward: bool,
    pub back: bool,
    pub left: bool,
    pub right: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CameraRotation {
    pub around_local_x: i32,
    pub around_global_y: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Command {
    CameraMove(CameraMove),
    CameraRotate(CameraRotation),
}

