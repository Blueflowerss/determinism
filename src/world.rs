use crate::{ tile::{self, Tile, TileType}, Actor, Site };
use rand::prelude::*;
use pathfinding::prelude::*;

pub struct World {
    pub grid_size_x: i32,
    pub grid_size_y: i32,
    pub actors: Vec<Actor>,
    pub sites: Vec<Site>, 
    pub terrain: Vec<Tile>,
}

impl World {
    pub fn new(grid_size_x:i32,grid_size_y:i32) -> World{
        let mut world_struct = World { grid_size_x: grid_size_x, 
            grid_size_y: grid_size_y, 
            actors: Vec::new(), 
            sites: Vec::new(),
            terrain: Vec::new()};
        for x in 1..grid_size_x {
            for y in 1..grid_size_y {
                world_struct.terrain.push(Tile{ tiletype: TileType::Ground, pos_x: x, pos_y: y });
            }
        }
        world_struct
    }
    pub fn add_actor(&mut self, name: &str, pos_x: Option<i32>,pos_y: Option<i32>){
        let mut new_actor = Actor::default();
        new_actor.first_name = name.to_string();
        new_actor.pos_x = pos_x.unwrap_or(new_actor.pos_x);
        new_actor.pos_y = pos_y.unwrap_or(new_actor.pos_y);
        self.actors.push(new_actor);
    }
    pub fn add_site(&mut self, name: &str, pos_x: Option<i32>, pos_y: Option<i32> ){
        let mut new_site = Site::default();
        new_site.name = name.to_string();
        new_site.pos_x = pos_x.unwrap_or(random::<i32>()%self.grid_size_x);
        new_site.pos_y = pos_y.unwrap_or(random::<i32>()%self.grid_size_y);
        self.sites.push(new_site);
    }
    pub fn update(&mut self){
        for site in &self.sites {

        }
        for actor in &self.actors {
            
        }
    }
/*     pub fn find_closest_site_to(&self, start_x: i32, start_y: i32) -> &Site{
    
    } */
}
