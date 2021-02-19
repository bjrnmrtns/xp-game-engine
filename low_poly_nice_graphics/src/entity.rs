use crate::{registry::Handle, renderer::Mesh, transform::Transform};

pub struct Entity {
    pub mesh_handle: Handle<Mesh>,
    pub transform: Transform,
}
