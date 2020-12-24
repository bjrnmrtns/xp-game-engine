use bevy::{prelude::*, utils::HashMap};

#[derive(Default)]
pub struct WorldGrid {
    pub grid: HashMap<(i32, i32, i32), Entity>,
}

#[derive(Default)]
pub struct MeshMap {
    pub handles: HashMap<String, Handle<Mesh>>,
}
