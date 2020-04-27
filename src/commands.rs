use serde::{Serialize, Deserialize};
use std::iter::Sum;

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
    camera_move(CameraMove),
    camera_rotate(CameraRotation),
}

