//! The simplest possible example that does something.
#![allow(clippy::unnecessary_wraps)]

use ggez::{
    conf, event, glam::*, graphics::{self, Color, Rect}, timer, Context, GameResult
};
use rand::prelude::*;
enum ActorType {
    Figure,
    Deity
}

enum SiteType {
    Default
}

enum DestinationType {
    None,
    Figure,
    Site
}

struct ActorPersonality {
    compassion: i32,
}

impl Default for ActorPersonality {
    fn default() -> Self {
        ActorPersonality {
            compassion: 100,
        }
    }
}


struct World {
    grid_size_x: i32,
    grid_size_y: i32,
    actors: Vec<Actor>,
    sites: Vec<Site>, 
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

struct Site {
    pos_x: i32,
    pos_y: i32,
    destroyed: bool,
}

impl Default for Site {
    fn default() -> Self {
        Site {
            //FIXME
            pos_x: random::<i32>()%20,
            pos_y: random::<i32>()%20,
            destroyed: false,
        }
    }
}

struct Actor {
    actor_type: ActorType,
    current_site: i32,
    pos_x: i32,
    pos_y: i32,
    destination: i32,
    destination_type: DestinationType,
    personality: ActorPersonality,
    text_blurb: String,
}

impl Default for Actor {
    fn default() -> Self {
        Actor {
            actor_type: ActorType::Figure,
            current_site: -1,
            pos_x: 0,
            pos_y: 0,
            destination: -1,
            destination_type: DestinationType::None,
            personality: ActorPersonality::default(),
            text_blurb: "I don't exist and i'm not okay with this!".to_string(),
        }
    }
}

struct MainState {
    square: graphics::Mesh,
    world: World,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let square = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            Rect::new(15.,15.,10.,10.),
            Color::WHITE,
        )?;
        let mut _world = World::default();
        Ok(MainState { 
            square,
            world: _world, 
            })
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if !_ctx.time.check_update_time(60){
        }
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        for x in 1..self.world.grid_size_x {
            for y in 1..self.world.grid_size_y {
                canvas.draw(&self.square, Vec2::new((x*15) as f32, (y*15) as f32));
            } 
        }
        for site in &self.world.sites {
            println!("{}",site.destroyed);
        }
        canvas.finish(ctx)?;
        //vsync
        timer::yield_now();
        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("Determinism", "Blueflowers")
    .window_setup(conf::WindowSetup::default().title("Determinism"))
    .window_mode(conf::WindowMode::default().dimensions(800., 600.));
    let (mut ctx, event_loop) = cb.build()?;
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}