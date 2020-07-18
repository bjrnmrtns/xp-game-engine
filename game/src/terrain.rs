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
        let mut current = TileHeader::new(pos.to_tile_index(0), 0).around(1);
        current.extend(TileHeader::new(pos.to_tile_index(1), 1).around(1));
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
        let mut current = TileHeader::new(pos.to_tile_index(0), 0).around(1);
        current.extend(TileHeader::new(pos.to_tile_index(1), 1).around(1));
        current.difference(&self.generated).map(|v| v.clone()).collect()
    }
    pub fn view(&self, pos: [f32; 3]) -> Vec<&T> {
        let mut current = TileHeader::new(pos.to_tile_index(0), 0).around(1);
        current.extend(TileHeader::new(pos.to_tile_index(1), 1).around(1));
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
    pub tile_nr: (i32, i32),
    pub lod: usize,
}

impl TileHeader {
    pub fn new(tile_nr: (i32, i32), lod: usize) -> Self {
        Self {
            tile_nr,
            lod
        }
    }

    pub fn around(&self, offset: usize) -> HashSet<TileHeader> {
        (self.tile_nr).to_tile_indices(offset, self.lod).iter().map(|v| TileHeader {
            tile_nr: v.clone(),
            lod: self.lod
        }).collect::<HashSet<_>>()
    }
}

fn noise(x: f64, z: f64, fbm: &noise::Fbm) -> f64 {
    fbm.get([x / 10.0, z / 10.0])
}

pub fn tile_and_index_to_coord(tile_nr: (i32, i32), x_index: usize, z_index: usize, lod: usize) -> (f64, f64) {
    let x = tile_nr.0 as f64 * TILE_SIZE_PER_LOD[lod] + x_index as f64 * TILE_PITCH_PER_LOD[lod];
    let z = tile_nr.1 as f64 * TILE_SIZE_PER_LOD[lod] + z_index as f64 * TILE_PITCH_PER_LOD[lod];
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

trait ToTileIndex {
    fn to_tile_index(self, lod: usize) -> (i32, i32);
}

impl ToTileIndex for [f32; 3] {
    fn to_tile_index(self, lod: usize) -> (i32, i32) {
        (value_to_tile_nr(self[0], lod), value_to_tile_nr(self[2], lod))
    }
}

trait ToTileIndices {
    fn to_tile_indices(self, offset: usize, lod: usize) -> Vec<Self> where Self: Sized;
}

impl ToTileIndices for i32 {
    fn to_tile_indices(self, offset: usize, lod: usize) -> Vec<Self> {
        let offset = offset as i32;
        let step_size = (lod + 1).pow(2) as i32;
        (-offset..=offset).map(|v| v * step_size + self).collect()
    }
}
impl ToTileIndices for (i32, i32) {
    fn to_tile_indices(self, offset: usize, lod: usize) -> Vec<Self> where Self: Sized {
        let mut output = Vec::new();
        for z in self.1.to_tile_indices(offset, lod) {
            for x in self.0.to_tile_indices(offset, lod) {
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
        Self::new(tile_header.tile_nr, tile_header.lod)
    }
    pub fn new(tile_nr: (i32, i32), lod: usize) -> Self {
        let fbm = noise::Fbm::new();
        let mut elements = vec!(Element::new(0.0, 0.0, 0.0); TILE_SIZE * TILE_SIZE);
        for z_index in 0..TILE_SIZE {
            for x_index in 0..TILE_SIZE {
                let (x, z) = tile_and_index_to_coord(tile_nr, x_index, z_index, lod);
                elements[z_index * TILE_SIZE + x_index] = Element::new(x as f32,noise(x, z, &fbm) as f32, z as f32);
            }
        }
        Self {
            header: TileHeader {
                tile_nr,
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

