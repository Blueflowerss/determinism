use crate::{ Actor, Site };
use rand::prelude::*;

pub struct World {
    pub grid_size_x: i32,
    pub grid_size_y: i32,
    pub actors: Vec<Actor>,
    pub sites: Vec<Site>, 
}

impl Default for World {
    fn default() -> Self {
        World {
            grid_size_x: 20,
            grid_size_y: 20,
            actors: Vec::new(),
            sites: Vec::new(),
        }
    }
}

impl World {
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
}