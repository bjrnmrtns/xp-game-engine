const TILE_SIZE: usize = 64;

#[derive(Clone)]
pub struct Element {
    x: i32,
    y: i32,
    z: i32,
}

impl Element {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self {
            x,
            y,
            z,
        }
    }
}

pub struct Tile {
    x: i32,
    z: i32,
    elements: Vec<Element>,
}

impl Tile {
    pub fn new() -> Self {
        let mut elements = vec!(Element::new(0, 0, 0); (TILE_SIZE + 1) * (TILE_SIZE + 1));
        let pitch = 1;
        for z in 0..TILE_SIZE + 1 {
            for x in 0..TILE_SIZE + 1 {
                elements[z * (TILE_SIZE + 1) + x] = Element::new(x as i32 * pitch, 0, z as i32 * pitch);
            }
        }
        Self {
            x: 0,
            z: 0,
            elements,
        }
    }
}

fn to_index_of_lod(index: i32, lod: u32, lod_size: i32) -> i32 {
    if index < 0 {
        let pos_value_mapped = (index * -1 - 1) / lod_size.pow(lod);
        return (pos_value_mapped + 1) * -1;
    } else {
        return index / lod_size.pow(lod);
    }
}

fn tiles_around_tile(tile: &[i32; 3]) -> [[i32; 3]; 9] {
     [[tile[0] - 1, tile[1], tile[2] - 1],
      [tile[0] - 1, tile[1], tile[2]],
      [tile[0] - 1, tile[1], tile[2] + 1],
      [tile[0], tile[1], tile[2] - 1],
      [tile[0], tile[1], tile[2]],
      [tile[0], tile[1], tile[2] + 1],
      [tile[0] + 1, tile[1], tile[2] - 1],
      [tile[0] + 1, tile[1], tile[2]],
      [tile[0] + 1, tile[1], tile[2] + 1]]
}
pub fn which_tiles(tile: [i32; 3], max_lod: u32) -> Vec<[i32; 3]> {
    assert_eq!(tile[1], 0);
    let mut tiles = Vec::new();
    for lod in 0..max_lod + 1 {
        let lod_tile = [to_index_of_lod(tile[0], lod, 2), lod as i32, to_index_of_lod(tile[2], lod, 2)];
        tiles.extend(tiles_around_tile(&lod_tile).iter());
    }
    tiles
}

#[test]
fn test_which_tiles_1() {
    let tiles = which_tiles([0, 0, 0], 0);
    assert_eq!(tiles.len(), 9);
    assert!(tiles.contains(&[-1, 0, -1]));
    assert!(tiles.contains(&[0, 0, -1]));
    assert!(tiles.contains(&[1, 0, -1]));
    assert!(tiles.contains(&[-1, 0, 0]));
    assert!(tiles.contains(&[0, 0, 0]));
    assert!(tiles.contains(&[1, 0, 0]));
    assert!(tiles.contains(&[-1, 0, 1]));
    assert!(tiles.contains(&[0, 0, 1]));
    assert!(tiles.contains(&[1, 0, 1]));
}

#[test]
fn test_which_tiles_2() {
    let tiles = which_tiles([-7, 0, 5], 2);
    assert_eq!(tiles.len(), 27);
    assert!(tiles.contains(&[-8, 0, 4]));
    assert!(tiles.contains(&[-7, 0, 4]));
    assert!(tiles.contains(&[-6, 0, 4]));
    assert!(tiles.contains(&[-8, 0, 5]));
    assert!(tiles.contains(&[-7, 0, 5]));
    assert!(tiles.contains(&[-6, 0, 5]));
    assert!(tiles.contains(&[-8, 0, 6]));
    assert!(tiles.contains(&[-7, 0, 6]));
    assert!(tiles.contains(&[-6, 0, 6]));

    assert!(tiles.contains(&[-4, 1, 2]));
    assert!(tiles.contains(&[-4, 1, 2]));
    assert!(tiles.contains(&[-3, 1, 2]));
    assert!(tiles.contains(&[-4, 1, 2]));
    assert!(tiles.contains(&[-4, 1, 2]));
    assert!(tiles.contains(&[-3, 1, 2]));
    assert!(tiles.contains(&[-4, 1, 3]));
    assert!(tiles.contains(&[-4, 1, 3]));
    assert!(tiles.contains(&[-3, 1, 3]));

    assert!(tiles.contains(&[-2, 2, 1]));
    assert!(tiles.contains(&[-2, 2, 1]));
    assert!(tiles.contains(&[-2, 2, 1]));
    assert!(tiles.contains(&[-2, 2, 1]));
    assert!(tiles.contains(&[-2, 2, 1]));
    assert!(tiles.contains(&[-2, 2, 1]));
    assert!(tiles.contains(&[-2, 2, 1]));
    assert!(tiles.contains(&[-2, 2, 1]));
    assert!(tiles.contains(&[-2, 2, 1]));
}


#[test]
fn test_to_index_of_lod1() {
    assert_eq!(to_index_of_lod(0, 1, 2), 0);
    assert_eq!(to_index_of_lod(1, 1, 2), 0);
    assert_eq!(to_index_of_lod(2, 1, 2), 1);
    assert_eq!(to_index_of_lod(3, 1, 2), 1);
    assert_eq!(to_index_of_lod(4, 1, 2), 2);
    assert_eq!(to_index_of_lod(5, 1, 2), 2);
    assert_eq!(to_index_of_lod(6, 1, 2), 3);
    assert_eq!(to_index_of_lod(-3, 1, 2), -2);
    assert_eq!(to_index_of_lod(-2, 1, 2), -1);
    assert_eq!(to_index_of_lod(-1, 1, 2), -1);

}

#[test]
fn test_to_index_of_lod2() {
    assert_eq!(to_index_of_lod(0, 2, 2), 0);
    assert_eq!(to_index_of_lod(1, 2, 2), 0);
    assert_eq!(to_index_of_lod(2, 2, 2), 0);
    assert_eq!(to_index_of_lod(3, 2, 2), 0);
    assert_eq!(to_index_of_lod(4, 2, 2), 1);
    assert_eq!(to_index_of_lod(-5, 2, 2), -2);
    assert_eq!(to_index_of_lod(-4, 2, 2), -1);
    assert_eq!(to_index_of_lod(-3, 2, 2), -1);
    assert_eq!(to_index_of_lod(-2, 2, 2), -1);
    assert_eq!(to_index_of_lod(-1, 2, 2), -1);
}

#[test]
fn test_to_index_of_lod3() {
    assert_eq!(to_index_of_lod(7, 3, 2), 0);
    assert_eq!(to_index_of_lod(8, 3, 2), 1);
    assert_eq!(to_index_of_lod(-8, 3, 2), -1);
    assert_eq!(to_index_of_lod(-9, 3, 2), -2);
}
