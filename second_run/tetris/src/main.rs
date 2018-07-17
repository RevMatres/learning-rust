extern crate piston;
extern crate opengl_graphics;
use opengl_graphics::OpenGL;

//mod piston_setup;
mod piston_engine;
mod tetris;

//use piston_setup::*;
use piston::event_loop::*;
use piston_engine::*;
use tetris::tetris_setup;


fn main() {
    // Create a Tetris Thread
    let (tetris_thread_handler, event_tx, gameS_rx) = tetris_setup();

    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin Window
    let mut window = Window::new("TETRIIIIIS!", [500, 600], opengl);

    // Create a Piston Renderer Thread
    let piston_thread_handler = Engine::new(window, gameS_rx);

    // Collect the Threads
    tetris_thread_handler.join().unwrap();
    piston_thread_handler.join().unwrap();
}
