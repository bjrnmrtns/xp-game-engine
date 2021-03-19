#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum TileType {
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
