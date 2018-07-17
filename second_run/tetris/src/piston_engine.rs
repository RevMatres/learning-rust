extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use self::piston::window::WindowSettings;
use self::piston::event_loop::*;
use self::piston::input::*;
use self::glutin_window::GlutinWindow;
use self::opengl_graphics::{ GlGraphics, OpenGL };

use std::sync::mpsc::Receiver;
use std::sync::{RwLock, LockResult, RwLockWriteGuard};
use std::thread;
use std::time::Duration;



/*
 * Making a Glutin Window
 *
 */


pub struct Window {
    pub window: GlutinWindow
}

impl Window {

    // Creates a new Window containing a Glutin OpenGL context
    pub fn new(title: &str, size: [u32; 2], opengl: OpenGL) -> Window {
        let mut win = WindowSettings::new(title, size)
            .opengl(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();

        Window {
            window: win
        }
    }
    
    // Returns the Glutin Window
    // Note: This Method moves the window out of the Window struct
    pub fn get_window(self) -> GlutinWindow {
        self.window
    }
} 

// Make Window Thread-safe!
unsafe impl Send for Window {}


/*
 * Making a Rendering Thread
 *
 */

pub struct Engine { }

impl Engine {

    // Create a new thread, in which Piston's Eventhandler is run
    pub fn new(mut window: Window, gamestate: Receiver<String>) -> thread::JoinHandle<()> {
        let piston_thread = thread::spawn(move || {

            // Get the Glutin Window
            let mut glutWin = window.get_window();

            // Create an Event Loop
            let mut eventqeue = setup_eventloop();

            // Run the Event Handler
            handle_events(&mut eventqeue, &mut glutWin, &gamestate)
        });

        return piston_thread
    }
}


// Make a new Eventloop
fn setup_eventloop() -> Events {
    let mut e = Events::new(EventSettings::new());
    return e
}

// Eventhandler to handle Events
fn handle_events(event_qeue: &mut Events, window: &mut GlutinWindow, gamestate: &Receiver<String>) {
    while let Some(event) = event_qeue.next(window) {
        // If the returned event is an Input::Render(RenderArgs)
        if let Some(render) = event.render_args() {
            if let Ok(value) = gamestate.try_recv() {
                println!("{:?}", value);
                //render(&render);
                continue;
            }
        }
    }
}
