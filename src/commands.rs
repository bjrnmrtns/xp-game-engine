use serde::{Serialize, Deserialize};
use std::iter::Sum;

pub trait Merge {
    fn merge_command(commands: &[Self]) -> Self where Self: Sized;
}


#[derive(Serialize, Deserialize, Clone)]
pub struct CameraMove {
    pub forward: bool,
    pub back: bool,
    pub left: bool,
    pub right: bool,
}

impl Merge for CameraMove {
    fn merge_command(commands: &[Self]) -> Self {
        let mut forward = false;
        let mut back = false;
        let mut left = false;
        let mut right = false;

        for c in commands {
            if c.forward {
                forward = true;
            }
            if c.back {
                back = true;
            }
            if c.left {
                left = true;
            }
            if c.right {
                right = true;
            }
        }
/*        CameraMove {
            forward: commands.iter().any(|c| c.forward),
            back: commands.iter().any(|c| c.back),
            left: commands.iter().any(|c| c.left),
            right: commands.iter().any(|c| c.right),
        }*/
                CameraMove {
                    forward: forward,
                    back: back,
                    left: left,
                    right: right,
                }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CameraRotation {
    pub around_local_x: i32,
    pub around_global_y: i32,
}

impl Merge for CameraRotation {
    fn merge_command(commands: &[Self]) -> Self {
        let mut x = 0;
        let mut y = 0;
        for c in commands {
            x = x + c.around_local_x;
            y = y + c.around_global_y;
        }
/*        CameraRotation {
            around_local_x: commands.iter().map(|c| c.around_local_x).sum(),
            around_global_y: commands.iter().map(|c| c.around_global_y).sum(),
        }*/
        CameraRotation {
            around_local_x: x,
            around_global_y: y,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Command {
    camera_move(CameraMove),
    camera_rotate(CameraRotation),
}

