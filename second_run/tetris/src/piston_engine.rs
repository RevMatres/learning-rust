// IMPORTS

extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use self::piston::window::WindowSettings;
use self::piston::event_loop::*;
use self::piston::input::*;
use self::glutin_window::GlutinWindow;
use self::opengl_graphics::{ GlGraphics, OpenGL };



/*
 * Making a Glutin Window
 *
 */

/// A structure that represents and instantiates a Glutin OpenGL Window
pub struct Window {
    pub window: GlutinWindow
}

impl Window {

    /// Creates a new Window 
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
 * Making a Renderer and Event-Handler
 *
 */

pub struct Engine {
    glutin_window: GlutinWindow,
    event_qeue: Events,
    gamestate: Receiver<String>,
}

impl Engine {

    // Create a new thread, in which Piston's Eventhandler is run
    pub fn new(mut window: Window, gamestate: Receiver<String>) -> Engine {
        Engine {
            glutin_window: window.get_window(),
            event_qeue: Engine::setup_eventloop(),
            gamestate
        }
    }

    // Make a new Eventloop
    fn setup_eventloop() -> Events {
        let mut e = Events::new(EventSettings::new());
        return e
    }

    // Handle incoming OpenGL Events
    pub fn handle_events(&mut self) {
        while let Some(event) = &mut self.event_qeue.next(&mut self.glutin_window) {

            if let Some(render) = event.render_args() {
                if let Ok(value) = self.gamestate.try_recv() {
                    println!("{:?}", value);
                    continue;
                }
            }

        }
    }

}
