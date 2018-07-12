extern crate opengl_graphics;
use opengl_graphics::OpenGL;

mod piston_setup;
mod tetris;

use piston_setup::*;
use tetris::tetris_setup;


fn main() {

    /*
     * SETUP THE WINDOW
     *
     */

    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window = setup_glutin_window("TETRIIIIIS!", [500, 600], opengl);

    // Create a Tetris Thread
    let (tetris_thread_handler, event_tx, gameS_rx) = tetris_setup();


    let mut app = App::new(opengl);
    let mut eventqeue = setup_eventloop();
    handle_events(&mut eventqeue, &mut window, &mut app, &gameS_rx);


    // Listen to the Game State channel
    loop {
        if let Ok(value) = gameS_rx.try_recv() {
            println!("{}", value);
        }
    }


    // Collect ya threads, bro
    tetris_thread_handler.join().unwrap();
}
