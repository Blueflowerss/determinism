
mod actor;
mod world;
mod site;
mod tile;

use ggez::{
    conf, event::{self, MouseButton, ScanCode}, glam::*, graphics::{self, Color, DrawParam, MeshBuilder, Rect}, input::{keyboard::{KeyCode, KeyInput, KeyMods, KeyboardContext}, mouse::MouseContext}, timer, Context, GameResult
};

use actor::*;
use pathfinding::num_traits::{clamp_min, ToPrimitive};
use world::*;
use site::*;
struct InputStruct {
    mouse_ctx: MouseContext,
    keyboard_ctx: KeyboardContext,
    left_mouse_down: bool,
    right_mouse_down: bool
}

impl Default for InputStruct {
    fn default() -> Self {
        InputStruct {
            mouse_ctx: MouseContext::default(),
            keyboard_ctx: KeyboardContext::default(),
            left_mouse_down: false,
            right_mouse_down: false
        }
    }
}

struct MainState {
    input_struct: InputStruct,
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
        let mut _world = World::new(32, 32);
        _world.add_actor("bob", None,None);
        _world.add_site("test", Some(10), Some(10));
        Ok(MainState { 
            input_struct: InputStruct::default(),
            square,
            world: _world,
            camera_zoom: 1.,
            camera_pan: vec2(0.,0.)
            })
    }
    fn game_to_screen_vector2(&self, pos_x: i32, pos_y: i32) -> Vec2 {
        vec2(
             (self.camera_pan.x+pos_x as f32)*self.camera_zoom,
             (self.camera_pan.y+pos_y as f32)*self.camera_zoom )
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
            let k_ctx = &_ctx.keyboard;
            if k_ctx.is_key_pressed(KeyCode::Escape) {

            }
        }
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));
        let mut mesh_builder = MeshBuilder::new();
        for (&(x,y),&ref Tile) in self.world.terrain.iter() {
            let tile_pos = MainState::game_to_screen_i32(&self, x*20, y*20);
            let err = mesh_builder.rectangle(ggez::graphics::DrawMode::fill(),
                Rect { x: tile_pos.x as f32, y: tile_pos.y as f32, w: 20., h: 20. }, 
                Tile.tiletype.color);
        }
        let mesh = graphics::Mesh::from_data(ctx,mesh_builder.build());
        canvas.draw(&mesh,DrawParam::new());
        for site in &self.world.sites {
            canvas.draw(&self.square, MainState::game_to_screen_vector2(self,site.pos_x, site.pos_y));
        }
        canvas.finish(ctx)?;
        //vsync
        timer::yield_now();
        Ok(())}
    fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult {
        match input.keycode {
            // Quit if Shift+Ctrl+Q is pressed.
            Some(KeyCode::Escape) => {
                println!("{}","dead");
                ctx.request_quit();
            }
            _ => (),
        }
    Ok(())
    }
    fn mouse_button_down_event(
            &mut self,
            _ctx: &mut Context,
            _button: event::MouseButton,
            _x: f32,
            _y: f32,
        ) -> Result<(), ggez::GameError> {
            if _button == MouseButton::Left {
                self.input_struct.left_mouse_down = true;
            }
            if _button == MouseButton::Right {
                self.input_struct.right_mouse_down = true;
            }
        Ok(())
    }
    fn mouse_button_up_event(
            &mut self,
            _ctx: &mut Context,
            _button: event::MouseButton,
            _x: f32,
            _y: f32,
        ) -> Result<(), ggez::GameError> {
            if _button == MouseButton::Left {
                self.input_struct.left_mouse_down = false;
            }
            if _button == MouseButton::Right {
                self.input_struct.right_mouse_down = false;
            }
        Ok(())
    }
    fn mouse_motion_event(
            &mut self,
            _ctx: &mut Context,
            _x: f32,
            _y: f32,
            _dx: f32,
            _dy: f32,
        ) -> Result<(), ggez::GameError> {
        if self.input_struct.right_mouse_down {
            self.camera_pan.x += _dx/self.camera_zoom;
            self.camera_pan.y += _dy/self.camera_zoom;
        }
        Ok(())
    }
    fn mouse_wheel_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32) -> Result<(), ggez::GameError> {
        self.camera_zoom = (self.camera_zoom+_y/2.).clamp(1., 10.);
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