#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum TileType {
    Empty,
    Test,
    Grass,
    Stone,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum TileConfiguration {
    NoSides,
    OneSide,
    BothSides,
    Corner,
    USide,
    All,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Tile {
    pub tile_type: TileType,
    pub configuration: TileConfiguration,
}

impl Tile {
    pub fn new(tile_type: TileType, configuration: TileConfiguration) -> Self {
        Self {
            tile_type,
            configuration,
        }
    }
}
