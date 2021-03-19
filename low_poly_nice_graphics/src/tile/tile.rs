pub enum TileType {
    Grass,
    Stone,
}

pub enum TileConfiguration {
    NoSides,
    OneSide,
    BothSides,
    Corner,
    USide,
    All,
}

pub enum TileOrientation {
    Zero,
    One,
    Two,
    Three,
}

pub struct Tile {
    pub tile_type: TileType,
    pub configuration: TileConfiguration,
    pub orientation: TileOrientation,
}
