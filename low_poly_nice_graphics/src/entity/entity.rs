use crate::{mesh::Mesh, physics::CollisionShape, registry::Handle, transform::Transform};

pub struct Entity {
    pub mesh_handle: Handle<Mesh>,
    pub collision_shape: Option<CollisionShape>,
    pub transform: Transform,
}
