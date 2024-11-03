use std::collections::HashMap;

//code tastefully inspired from https://bfnightly.bracketproductions.com/chapter_3.html

pub const GRIDSIZE_X: i32 = 80;
pub const GRIDSIZE_Y: i32 = 80;



#[derive(PartialEq, Copy, Clone)]
pub enum WorldTileType {
    Water,Shore,Land,Mountain
} 

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * GRIDSIZE_X as usize) + x as usize
}

pub fn new_map()->Vec<WorldTileType> {
    let mut map = vec![WorldTileType::Land; (GRIDSIZE_X*GRIDSIZE_Y) as usize];
    for x in 0..GRIDSIZE_X {
        if let 0=x%2{
            map[xy_idx(x, 2)] = WorldTileType::Water;
        }
        map[xy_idx(x, 2)] = WorldTileType::Mountain;
    }
    map
}