use notan::prelude::*;
use notan::draw::*;

const GRIDSIZE_X:i32 = 100;
const GRIDSIZE_Y:i32 = 100;

#[notan_main]
fn main() -> Result<(), String> {
    notan::init().draw(draw)
    .add_config(DrawConfig)
    .build()
}
fn draw(gfx: &mut Graphics) {
    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);
    for x in 1..GRIDSIZE_X{
        for y in 1..GRIDSIZE_Y{
            draw.rect((x as f32 * 20.0,y as f32 * 20.0), (10.0,10.0));
        } 
    }
    //draw.triangle((400.0, 100.0), (100.0, 500.0), (700.0, 500.0));
    gfx.render(&draw);
}