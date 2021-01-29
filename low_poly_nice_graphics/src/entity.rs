use crate::{assets::Handle, renderer::Mesh};
use nalgebra_glm::Mat4;

pub struct Entity {
    pub mesh_handle: Handle<Mesh>,
    pub model: Mat4,
}
