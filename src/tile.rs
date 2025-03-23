use std::collections::hash_map;

use ggez::graphics::Color;

pub struct Tile {
    pub tiletype: TileType,
    pub pos_x: i32,
    pub pos_y: i32,
}

pub struct TileType {
    pub ground: bool,
    pub water: bool,
    pub obstructed_ground_travel:bool,
    pub obstructed_travel:bool,
    pub color:Color,
}
impl TileType {
    pub const fn new(ground:bool,water:bool,obstructed_ground_travel:bool,obstructed_travel:bool,color:Color) -> Self {
        Self { ground, water, obstructed_ground_travel, obstructed_travel,color }
    }
    pub const PLAINS: Self = Self::new(
        true, 
        false, 
        false, 
        false,
        Color::GREEN);
    pub const WATER: Self = Self::new(
        false,
        true,
        false,
        false,
        Color::BLUE);
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