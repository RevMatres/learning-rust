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

    /*
     * SETUP THE WINDOW
     *
     */

    // Create a Tetris Thread
    let (tetris_thread_handler, event_tx, gameS_rx) = tetris_setup();

    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin Window
    let mut window = Window::new("TETRIIIIIS!", [500, 600], opengl);
    let piston_thread_handler = Engine::new(window, gameS_rx);

    tetris_thread_handler.join().unwrap();
    piston_thread_handler.join().unwrap();

    // Create the Rendering Thread
    //let piston_thread_handler = piston_setup(opengl, "TETRIIIIIS!", [500, 600], gameS_rx);
    //let optional_gamestate = piston_thread_handler.join();

    //match optional_gamestate {
    //    Ok(gameState) => {
    //        loop {
    //            if let Ok(value) = gameState.try_recv() {
    //                println!("{}", value);
    //            }
    //        }
    //    },
    //    _ => {}
    //}
/*
    // Listen to the Game State channel
    loop {
        if let Ok(value) = gameS_rx.try_recv() {
            println!("{}", value);
        }
    }


    // Collect ya threads, bro
    tetris_thread_handler.join().unwrap();
    */
}
