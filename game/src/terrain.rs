use noise::NoiseFn;
use std::collections::{HashMap, HashSet};

pub const TILE_SIZE: usize = 65;
const TILE_PITCH_PER_LOD: [f64; 10] = [0.1, 0.2, 0.4, 0.8, 1.6, 3.2, 6.4, 12.8, 25.6, 51.2];
const TILE_SIZE_PER_LOD: [f64; 10] = [6.4, 12.8, 25.6, 51.2, 102.4, 204.8, 409.6, 819.2, 1638.4, 3276.8];

pub struct TileCache<T> {
    generated: HashSet<TileHeader>,
    tiles: HashMap<TileHeader, T>,
}

impl<T> TileCache<T> {
    pub fn new() -> Self {
        Self {
            generated: HashSet::new(),
            tiles: HashMap::new(),
        }
    }
    pub fn update(&mut self, pos: [f32; 3], mut tiles: Vec<(TileHeader, T)>) {
        let tile_nr = pos_to_tile_nr(&pos, 0);
        let current = TileHeader::new(tile_nr.0, tile_nr.1, 0).view_around();
        let to_replace: Vec<TileHeader> = self.generated.difference(&current).map(|v| v.clone()).collect();
        for r in to_replace {
            self.tiles.remove(&r);
            self.generated.remove(&r);
        }
        while !tiles.is_empty() {
            let t = tiles.remove(tiles.len() - 1);
            self.generated.insert(t.0.clone());
            self.tiles.insert(t.0, t.1);
        }
    }
    pub fn what_needs_update(&self, pos: [f32; 3]) -> HashSet<TileHeader> {
        let tile_nr = pos_to_tile_nr(&pos, 0);
        let current = TileHeader::new(tile_nr.0, tile_nr.1, 0);
        current.view_around().difference(&self.generated).map(|v| v.clone()).collect()
    }
    pub fn view(&self, pos: [f32; 3]) -> Vec<&T> {
        let tile_nr = pos_to_tile_nr(&pos, 0);
        let current = TileHeader::new(tile_nr.0, tile_nr.1, 0).view_around();
        self.tiles.iter().filter(|v| current.contains(v.0)).map(|v| v.1).collect()
    }
}

#[derive(Clone)]
pub struct Element {
    pub p: [f32; 3],
}

impl Element {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            p: [x, y, z],
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TileHeader {
    pub x: i32,
    pub z: i32,
    pub lod: usize,
}

impl TileHeader {
    pub fn new(x: i32, z: i32, lod: usize) -> Self {
        Self {
            x,
            z,
            lod
        }
    }

    pub fn view_around(&self) -> HashSet<TileHeader> {
        (self.x, self.z).gen_range(1, 0).iter().map(|v| TileHeader {
            x: v.0,
            z: v.1,
            lod: 0
        }).collect::<HashSet<_>>()
    }
}

fn noise(x: f64, z: f64, fbm: &noise::Fbm) -> f64 {
    fbm.get([x / 10.0, z / 10.0])
}

pub fn tile_and_index_to_coord(tile_x: i32, tile_z: i32, x_index: usize, z_index: usize, lod: usize) -> (f64, f64) {
    let x = tile_x as f64 * TILE_SIZE_PER_LOD[lod] + x_index as f64 * TILE_PITCH_PER_LOD[lod];
    let z = tile_z as f64 * TILE_SIZE_PER_LOD[lod] + z_index as f64 * TILE_PITCH_PER_LOD[lod];
    (x, z)
}

fn value_to_tile_nr(value: f32, lod: usize) -> i32 {
    let tile_size = TILE_SIZE_PER_LOD[lod] as f32;
    if value < 0.0 {
        -1 + (value / tile_size) as i32
    } else {
        (value / tile_size) as i32
    }
}

pub fn pos_to_tile_nr(pos: &[f32; 3], lod: usize) -> (i32, i32) {
    let x = pos[0];
    let z = pos[2];
    (value_to_tile_nr(x, lod), value_to_tile_nr(z, lod))
}



pub trait TileIndexGen {
    fn gen_range(&self, offset: i32, lod: usize) -> Vec<Self> where Self: Sized;
}

impl TileIndexGen for i32 {
    fn gen_range(&self, offset: i32, lod: usize) -> Vec<Self> {
        let step_size = (lod + 1).pow(2) as i32;
        (-offset..=offset).map(|v| v * step_size + self).collect()
    }
}
impl TileIndexGen for (i32, i32) {
    fn gen_range(&self, offset: i32, lod: usize) -> Vec<Self> where Self: Sized {
        let mut output = Vec::new();
        for z in self.1.gen_range(offset, lod) {
            for x in self.0.gen_range(offset, lod) {
                output.push((x, z));
            }
        }
        output
    }
}

pub struct Tile {
    header: TileHeader,
    pub elements: Vec<Element>,
}

impl Tile {
    pub fn new_from_header(tile_header: TileHeader) -> Self {
        Self::new(tile_header.x, tile_header.z, tile_header.lod)
    }
    pub fn new(tile_x: i32, tile_z: i32, lod: usize) -> Self {
        let fbm = noise::Fbm::new();
        let mut elements = vec!(Element::new(0.0, 0.0, 0.0); TILE_SIZE * TILE_SIZE);
        for z_index in 0..TILE_SIZE {
            for x_index in 0..TILE_SIZE {
                let (x, z) = tile_and_index_to_coord(tile_x, tile_z, x_index, z_index, lod);
                elements[z_index * TILE_SIZE + x_index] = Element::new(x as f32,noise(x, z, &fbm) as f32, z as f32);
            }
        }
        Self {
            header: TileHeader {
                x: tile_x,
                z: tile_z,
                lod,
            },
            elements,
        }
    }
    pub fn color(&self) -> [f32; 3] {
        const LOD_COLOR: [[f32; 3]; 3] = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
        LOD_COLOR[self.header.lod]
    }
    pub fn get_element(&self, x_index: usize, z_index: usize) -> &Element {
        &self.elements[z_index * TILE_SIZE + x_index]
    }
    pub fn set_height(&mut self, x_index: usize, z_index: usize, height: f32) {
        self.elements[z_index * TILE_SIZE + x_index].p[1] = height;
    }
}

