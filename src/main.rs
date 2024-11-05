
mod actor;
mod world;
mod site;

use ggez::{
    conf, event, glam::*, graphics::{self, Color, Rect}, timer, Context, GameResult
};

use actor::*;
use world::*;
use site::*;

struct MainState {
    square: graphics::Mesh,
    world: World,
    camera_zoom: f32,
    camera_pan: Vec2
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
        _world.add_actor("bob", None,None);
        _world.add_site("test", Some(10), Some(10));
        Ok(MainState { 
            square,
            world: _world,
            camera_zoom: 0.,
            camera_pan: vec2(0.,0.)
            })
    }
    fn game_to_screen_vector2(&self, pos_x: i32, pos_y: i32) -> Vec2 {
        vec2( (self.camera_pan.x+pos_x as f32)*self.camera_zoom,(self.camera_pan.y+pos_y as f32)*self.camera_zoom )
    }
    fn game_to_screen_i32(&self, pos_x: i32, pos_y: i32) -> Vec2 {
        vec2 ( 
            ( self.camera_pan.x+pos_x as f32)*self.camera_zoom,
            ( self.camera_pan.y+pos_y as f32)*self.camera_zoom)
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
                canvas.draw(&self.square,
                MainState::game_to_screen_i32(self,x,y));
            } 
        }
        for site in &self.world.sites {
            canvas.draw(&self.square, MainState::game_to_screen_vector2(self,site.pos_x, site.pos_y));
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