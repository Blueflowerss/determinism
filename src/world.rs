use crate::{ Actor, Site, actor, tile::{self, Tile, TileType} };
use ggez::{context::Has, glam::vec2};
use noise::{core::open_simplex::open_simplex_2d, permutationtable::{NoiseHasher, PermutationTable}, NoiseFn, Perlin, ScalePoint, Simplex, Vector2};
use rand::prelude::*;
use pathfinding::{directed::bfs, grid, prelude::bfs_reach};
use std::{collections::hash_map::*, vec,collections::HashSet};

pub struct World {
    pub grid_size_x: i32,
    pub grid_size_y: i32,
    pub actors: Vec<Actor>,
    pub new_actor_id: i32,
    pub sites: Vec<Site>,
    pub need_world_update: bool,
    pub new_site_id: i32, 
    pub terrain: HashMap<(i32,i32),Tile>,
    pub island_collection: IslandCollection,
}
//navigation stuff
pub struct Island {
    pub id: i32,
    pub size: i32,
    pub tiles: HashSet<(i32,i32)>,
}
pub struct IslandCollection {
    pub islands: Vec<Island>,
    pub next_island: i32,
    //dynamic references to sites and actors currently on the island
    pub which_sites_does_island_have : HashMap<i32,HashSet<i32>>,
    pub which_actors_does_island_have : HashMap<i32,HashSet<i32>>,
    pub tile_to_island_index: HashMap<(i32,i32),i32>,
}
impl IslandCollection {
    pub fn new() -> IslandCollection {
        let mut island_collection = IslandCollection {
            islands: Vec::new(),
            next_island: 0,
            which_sites_does_island_have: HashMap::new(),
            which_actors_does_island_have: HashMap::new(),
            tile_to_island_index: HashMap::new(),
        };
        island_collection.which_sites_does_island_have.insert(-1,HashSet::new());
        island_collection.which_actors_does_island_have.insert(-1,HashSet::new());
        island_collection
    }
    pub fn insert(&mut self, island : Island, island_index : i32){
        for tile in &island.tiles {
            self.tile_to_island_index.insert(*tile,island_index);
        }
        self.islands.push(island);
    }
    pub fn get_island_index_at_tile(&self, cell_x : i32, cell_y : i32) -> i32{
        if self.tile_to_island_index.contains_key(&(cell_x,cell_y)){
            self.tile_to_island_index[&(cell_x,cell_y)]
        } else {
            -1
        }
    }
    pub fn clear(&mut self){
        self.islands.clear();
        self.next_island = 0;
        self.tile_to_island_index.clear();
    }
}

impl World {
    pub fn new(grid_size_x:i32,grid_size_y:i32,seed:i32) -> World{
        let mut world_struct = World { grid_size_x: grid_size_x, 
            grid_size_y: grid_size_y, 
            actors: Vec::new(), 
            sites: Vec::new(),
            need_world_update: false,
            terrain: HashMap::new(),
            island_collection: IslandCollection::new(),
            new_actor_id: 0,
            new_site_id: 0, };
        let hasher = PermutationTable::new(seed as u32);
        let zoom_factor = 0.1;
        for x in 1..=grid_size_x {
            for y in 1..=grid_size_y {
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
    pub fn get_successors(&mut self, grid_x:i32, grid_y:i32) -> Vec<(i32,i32)> {
        let mut successors = Vec::new();
        for x in -1..=1 {
            for y in -1..=1 {
                let x_within_bounds:bool = (1..=self.grid_size_x).contains(&(grid_x+x));
                let y_within_bounds:bool = (1..=self.grid_size_y).contains(&(grid_y+y));
                if (x_within_bounds && y_within_bounds){        
                    if self.get_terrain(grid_x+x, grid_y+y).tiletype.ground {
                        successors.push((grid_x+x,grid_y+y));
                    }
            }
            }
        }
        successors
    }
    pub fn set_terrain_type(&mut self, grid_x:i32, grid_y:i32,set_to_tile:tile::TileType){
        self.terrain.entry((grid_x,grid_y)).and_modify(|e| e.tiletype = set_to_tile);
        self.need_world_update = true;
    }
    pub fn map_islands(&mut self){
        let mut visited:HashSet<(i32,i32)> = HashSet::new();
        self.island_collection.clear();
        for x in 1..=self.grid_size_x {
            for y in 1..=self.grid_size_y {
                if (!visited.contains(&(x,y))){
                    if (self.get_terrain(x, y).tiletype.ground){
                        let island_id = self.island_collection.next_island;
                        let reachable = bfs_reach((x,y), 
                        |(x,y)| self.get_successors(*x, *y));
                        let mut island_tiles: HashSet<(i32,i32)> = HashSet::new();
                        let mut reachable_count = 0;
                        for tile in reachable {
                            visited.insert(tile);
                            island_tiles.insert(tile);
                            reachable_count += 1;
                        }
                        let island: Island = Island { id: self.island_collection.next_island,
                             size: reachable_count,
                              tiles: island_tiles };
                              self.island_collection.insert(island, self.island_collection.next_island);
                        self.island_collection.next_island += 1;
                    }
                }
            }
        }
    }
    pub fn update(&mut self){
        if self.need_world_update {
            self.update_world();
        }
        self.update_entities();
    }
    pub fn update_world(&mut self){
        // -1 island is probably the ocean
        for site in &self.sites{
            let site_position = (site.pos_x,site.pos_y);
            let site_island = self.island_collection.get_island_index_at_tile(
                site_position.0,
                site_position.1);
            if let Some(val) = self.island_collection.which_sites_does_island_have.get_mut(&site_island) { val.insert(site.id); };
        }
        for actor in &self.actors{
            let actor_position = (actor.pos_x,actor.pos_y);
            let actor_island = self.island_collection.get_island_index_at_tile(
                actor_position.0,
                actor_position.1);
            if let Some(val) = 
            self.island_collection.which_actors_does_island_have
            .get_mut(&actor_island) { val.insert(actor.id); };
        }
        self.need_world_update = false;
    }
    pub fn update_entities(&mut self){
        for site in &self.sites {

        }
        for actor in self.actors.iter_mut() {
            actor.update();
        }
    }
/*     pub fn find_closest_site_to(&self, start_x: i32, start_y: i32) -> &Site{
    
    } */
}
