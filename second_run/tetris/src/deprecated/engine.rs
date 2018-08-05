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
use std::sync::mpsc::Receiver;
use std::sync::RwLockReadGuard;



/*
 * Making a Window
 *
 */

/// represents a Glutin OpenGL Window
pub struct Window {
    title: String,
    size: [u32; 2],
    opengl: OpenGL,
}

impl Window {

    /// creates a wrapper for the settings passed to Glutin when making the OpenGL context  
    ///
    /// **This doesn't directly create a Glutin window, merely a collection of arguments to make on
    /// with!**
    ///
    /// This function only exists to make the API of creating a Glutin Window a little more easy to
    /// read, so the Engine::new() function won't also have to take all the parameters this
    /// function takes.
    pub fn new(title: &str, size: [u32; 2], opengl: OpenGL) -> Window {
        Window {
            title: title.to_string(),
            size,
            opengl
        }
    }

    /// create a Glutin window / OpenGL context with the settings contained in an instance of
    /// Window
    ///
    /// **This creates a Glutin Window and a GlGraphics object.** Those two need to be created in
    /// the same thread and can't really be moved accross threads, if you don't want segmentation
    /// faults!
    pub fn create(self) -> (GlutinWindow, GlGraphics) {
        let glutin_window = WindowSettings::new(self.title, self.size)
            .opengl(self.opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();
        let glgraphics = GlGraphics::new(self.opengl);
        (glutin_window, glgraphics)
    }

} 



/*
 * Making a Renderer and Event-Handler
 *
 */

/// Represents an instance of the Piston engine with a Glutin OpenGL Context and a GlGraphics
/// object for rendering.
pub struct Engine {
    glutin_window: GlutinWindow,
    glgraphics: GlGraphics,
    event_qeue: Events,
    gamestate: Receiver<RwLockReadGuard<String>>,
}

impl Engine {

    /// creates an instance of `Engine`
    ///
    /// ## The Input Parameters
    /// `window` takes a Window containing the desired settings for the Glutin Window  
    /// `gamestate` takes the receiving end of a channel via which some other part of the program
    /// sends a reference to a Game-State object to the Engine, so it can be rendered
    pub fn new(window: Window, gamestate: Receiver<RwLockReadGuard<String>>) -> Engine {

        // create the actual Glutin Window from the provided settings
        let (glutin_window, glgraphics) = window.create();

        Engine {
            glutin_window,
            glgraphics,
            event_qeue: Engine::setup_eventloop(),
            gamestate,
        }
    }


    /// Make a new Piston Eventloop
    fn setup_eventloop() -> Events {
        let e = Events::new(EventSettings::new());
        return e
    }

    /// Handle incoming OpenGL Events
    ///
    /// This thing is literally just the piston event-handler. It looks for incoming OpenGL events
    /// and responds with specified functions.
    pub fn handle_events(&mut self) {
        while let Some(event) = &mut self.event_qeue.next(&mut self.glutin_window) {

            if let Some(render_args) = event.render_args() {
                use self::graphics::*;
                self.glgraphics.draw(render_args.viewport(), |c, gl|{
                    clear([ 0f32, 0f32, 0f32, 0f32], gl)
                });
                if let Ok(board) = self.gamestate.try_recv() {
                    println!("{}", board);
                    drop(board);
                    continue;
                }
            }

        }
    }

}
