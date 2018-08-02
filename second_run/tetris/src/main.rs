extern crate piston;
extern crate glutin_window;
extern crate opengl_graphics;

mod engine;
mod tetris;

use opengl_graphics::{GlGraphics, OpenGL};
use self::piston::window::WindowSettings;
use engine::*;
use self::glutin_window::GlutinWindow;
use tetris::tetris_setup;
use std::thread;


fn main() {
    // Create a Tetris Thread
    let (tetris_thread_handler, event_tx, gameS_rx) = tetris_setup();

    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a window (this only contains the settings for a window)
    let window = Window::new("TETRIIIIIS!", [500, 600], opengl);

    // Create a Piston Renderer Thread
    let piston_thread_handler = thread::spawn(move || {
        let mut piston_renderer = Engine::new(window, gameS_rx);
        piston_renderer.handle_events()
    });

    // Collect the Threads
    tetris_thread_handler.join().unwrap();
    piston_thread_handler.join().unwrap();

    /*
    // Run the Renderer in the main()-thread
    let mut piston_renderer = Engine::new(window, gameS_rx);
    piston_renderer.handle_events()
    */
}
