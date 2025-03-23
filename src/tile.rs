use std::collections::hash_map;

pub struct Tile {
    pub tiletype: TileType,
    pub pos_x: i32,
    pub pos_y: i32,
}

pub struct TileType {
    ground: bool,
    water: bool,
    obstructed_ground_travel:bool,
    obstructed_travel:bool,
}
impl TileType {
    pub const fn new(ground:bool,water:bool,obstructed_ground_travel:bool,obstructed_travel:bool) -> Self {
        Self { ground, water, obstructed_ground_travel, obstructed_travel }
    }
    pub const PLAINS: Self = Self::new(true, false, false, false);
    pub const WATER: Self = Self::new(false, true, false, false);
}
impl Default for Tile {
    fn default() -> Self {
        Tile {
            pos_x: 0,
            pos_y: 0,
            tiletype:TileType::PLAINS,
        }
    }
}