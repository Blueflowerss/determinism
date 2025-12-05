use crate::{ tile::{self, Tile, TileType}, Actor, Site };
use ggez::context::Has;
use noise::{core::open_simplex::open_simplex_2d, permutationtable::{NoiseHasher, PermutationTable}, NoiseFn, Perlin, ScalePoint, Simplex, Vector2};
use rand::prelude::*;
use pathfinding::prelude::*;
use std::{collections::hash_map::*, vec};

pub struct World {
    pub grid_size_x: i32,
    pub grid_size_y: i32,
    pub actors: Vec<Actor>,
    pub sites: Vec<Site>, 
    pub terrain: HashMap<(i32,i32),Tile>,
}

impl World {
    pub fn new(grid_size_x:i32,grid_size_y:i32) -> World{
        let mut world_struct = World { grid_size_x: grid_size_x, 
            grid_size_y: grid_size_y, 
            actors: Vec::new(), 
            sites: Vec::new(),
            terrain: HashMap::new(),
            new_actor_id: 0,
            new_site_id: 0, };
        let hasher = PermutationTable::new(10);
        let zoom_factor = 0.1;
        for x in 1..grid_size_x {
            for y in 1..grid_size_y {
                let tile_noise = open_simplex_2d(Vector2 {x: (x as f64)*zoom_factor,y: (y as f64)*zoom_factor }, &hasher);
                if (tile_noise > 0.) {
                    world_struct.terrain.insert((x,y),Tile{ tiletype: TileType::PLAINS, pos_x: x, pos_y: y });
                }else{
                    world_struct.terrain.insert((x,y),Tile{ tiletype: TileType::WATER, pos_x: x, pos_y: y });
                }
            }
        }
        world_struct
    }
    pub fn add_actor(&mut self, name: &str, pos_x: Option<i32>,pos_y: Option<i32>){
        let mut new_actor = Actor::default();
        new_actor.first_name = name.to_string();
        new_actor.pos_x = pos_x.unwrap_or(new_actor.pos_x);
        new_actor.pos_y = pos_y.unwrap_or(new_actor.pos_y);
        new_actor.id = self.new_actor_id;
        self.new_actor_id += 1;
        self.actors.push(new_actor);
    }
    pub fn add_site(&mut self, name: &str, pos_x: Option<i32>, pos_y: Option<i32> ){
        let mut new_site = Site::default();
        new_site.name = name.to_string();
        new_site.pos_x = pos_x.unwrap_or(random::<i32>()%self.grid_size_x);
        new_site.pos_y = pos_y.unwrap_or(random::<i32>()%self.grid_size_y);
        new_site.id = self.new_site_id;
        self.new_site_id += 1;
        self.sites.push(new_site);
    }
    pub fn get_terrain(&mut self, grid_x:i32, grid_y:i32) -> &Tile{
        self.terrain.get(&(grid_x,grid_y)).expect("couldn't access terrain Tile element")
    }
    pub fn set_terrain_type(&mut self, grid_x:i32, grid_y:i32,set_to_tile:tile::TileType){
        self.terrain.entry((grid_x,grid_y)).and_modify(|e| e.tiletype = set_to_tile);
    }
    pub fn update(&mut self){
        for site in &self.sites {

        }
        for actor in self.actors.iter_mut() {
            actor.update();
        }
    }
/*     pub fn find_closest_site_to(&self, start_x: i32, start_y: i32) -> &Site{
    
    } */
}
