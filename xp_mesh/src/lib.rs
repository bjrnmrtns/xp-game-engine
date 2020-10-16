mod error;
mod obj;

pub mod mesh {
    pub use crate::error::*;
    pub use crate::obj::Obj;
}

#[derive(Clone)]
pub struct Triangle<T> {
    pub positions: [T; 3],
    pub diffuse_color: Option<T>,
}
