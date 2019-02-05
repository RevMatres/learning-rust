//IMPORTS

extern crate piston;
extern crate graphics;
use self::piston::*;
use self::graphics::*;

pub fn render_hello() {
    println!("Hello from RENDER")
}

// render function, yay
pub fn render(engine: &mut super::Engine, render_args: input::RenderArgs) {
    engine.glgraphics.draw(render_args.viewport(), |c, gl|{
        clear([ 0.5f32, 0.5f32, 0.5f32, 0.5f32], gl)
    });
}
