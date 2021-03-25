use crate::{mesh::Mesh, registry::Handle, transform::Transform};

pub struct Entity {
    pub mesh_handle: Handle<Mesh>,
    pub transform: Transform,
}
