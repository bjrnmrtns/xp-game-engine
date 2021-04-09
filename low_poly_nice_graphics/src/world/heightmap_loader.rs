use crate::{mesh::Mesh, registry::Handle};

pub struct Heightmap;

impl Heightmap {
    pub fn load_heightmap(add_mesh: impl FnMut(Mesh) -> Handle<Mesh>) {}
}
