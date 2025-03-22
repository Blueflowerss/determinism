pub struct Tile {
    pub tiletype: TileType,
    pub pos_x: i32,
    pub pos_y: i32,
}

pub enum TileType {
    Ground,
    Water
}

impl Default for Tile {
    fn default() -> Self {
        Tile {
            pos_x: 0,
            pos_y: 0,
            tiletype:TileType::Ground,
        }
    }
}