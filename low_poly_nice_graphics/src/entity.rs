use crate::{registry::Handle, renderer::Mesh};
use glam::Mat4;

pub struct Entity {
    pub mesh_handle: Handle<Mesh>,
    pub model: Mat4,
}
