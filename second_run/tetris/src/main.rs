//! # Tetris 
//! This is an implementation of the legendary game Tetris. It is structured in a somewhat unusual
//! way -- the render engine, event handler and game logic are completely separate from each other.
//! In a more sensible implementation the game engine would also provide the game loop, for
//! performance and syncronization reasons.
//!
//! This is done differently here, because there is another part to this game. The tetris game is
//! build, so that it can be played by an Artificial Intelligence. In order to make this easier
//! the game engine is removed from the game, is actually completely optional and replacable with
//! for example a file descriptor. A version using the piston game engine exists in this crate, so
//! not only an end-user can/could play the game, but one can also watch an AI play tetris.
//!
//! Oh boy, why am I doing this to myself?


// IMPORTS

extern crate piston;
extern crate glutin_window;
extern crate opengl_graphics;

mod engine;
mod tetris;

use opengl_graphics::OpenGL;
use tetris::tetris_setup;
use engine::*;


fn main() {

    // Create a Tetris Thread
    let (tetris_thread_handler, events_tx, game_state_rx) = tetris_setup();

    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a window (this only contains the settings for a window)
    let window = Window::new("TETRIIIIIS!", [500, 600], opengl);

    /*
    // Create a Piston Renderer Thread
    let piston_thread_handler = thread::spawn(move || {
        let mut piston_renderer = Engine::new(window, game_state_rx);
        piston_renderer.handle_events()
    });

    // Collect the Threads
    tetris_thread_handler.join().unwrap();
    piston_thread_handler.join().unwrap();
    */

    // Run the Renderer in the main()-thread
    let mut piston_renderer = Engine::new(window, game_state_rx);
    piston_renderer.handle_events()

}
