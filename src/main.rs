//! The simplest possible example that does something.
#![allow(clippy::unnecessary_wraps)]

use ggez::{
    conf, event, glam::*, graphics::{self, Color, Rect}, timer, Context, GameResult
};

enum ActorType {
    Figure,
    Deity
}

enum SiteType {
    Default
}

enum DestinationType {
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

struct Actor {
    actor_type: ActorType,
    current_site: i32,
    destination: i32,
    destination_type: DestinationType,
    personality: ActorPersonality,
    text_blurb: String,
}

struct MainState {
    grid_size_x: i32,
    grid_size_y: i32, 
    square: graphics::Mesh,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let square = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            Rect::new(15.,15.,15.,15.),
            Color::WHITE,
        )?;
        Ok(MainState { square,
            grid_size_x: 20, 
            grid_size_y: 20,
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

        for x in 1..self.grid_size_x {
            for y in 1..self.grid_size_y {
                canvas.draw(&self.square, Vec2::new((x*15) as f32, (y*15) as f32));
            } 
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