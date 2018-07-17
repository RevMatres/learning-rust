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



/*
 * Making a Glutin Window
 *
 */


pub struct Window {
    title: String,
    size: [u32; 2],
    opengl: OpenGL,
}

impl Window {

    pub fn new(title: &str, size: [u32; 2], opengl: OpenGL) -> Window {
        Window {
            title: title.to_string(),
            size,
            opengl
        }
    }

    pub fn create(self) -> (GlutinWindow, GlGraphics) {
        let glutin_window = WindowSettings::new(self.title, self.size)
            .opengl(self.opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();
        let glgraphics = GlGraphics::new(self.opengl);
        (glutin_window, glgraphics)
    }

    /*
    // Creates a new Window containing a Glutin OpenGL context
    pub fn new(title: &str, size: [u32; 2], opengl: OpenGL) -> Window {
        let win = WindowSettings::new(title, size)
            .opengl(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();

        Window {
            window: win,
            gl: GlGraphics::new(opengl),
        }
    }
    
    // Returns the Glutin Window
    // Note: This Method moves the window out of the Window struct
    pub fn get_window(self) -> (GlutinWindow, GlGraphics) {
        (self.window, self.gl)
    }
    */

} 

// Make Window Thread-safe!
//unsafe impl Send for Window {}


/*
 * Making a Renderer and Event-Handler
 *
 */

pub struct Engine {
    glutin_window: GlutinWindow,
    glgraphics: GlGraphics,
    event_qeue: Events,
    gamestate: Receiver<String>,
}

impl Engine {

    // Create a new Engine Instance
    //pub fn new(window: Window, gamestate: Receiver<String>) -> Engine {
    /*
    pub fn new(gamestate: Receiver<String>) -> Engine {
    //pub fn new(window: Window) -> Engine {
        let window = Window::new("TETRIIIIIS!", [500, 600], OpenGL::V3_2);
        let (w, g) = window.get_window();
        Engine {
            glutin_window: w,
            gl: g,
            event_qeue: Engine::setup_eventloop(),
            gamestate,
        }
    }
    */

    pub fn new(window: Window, gamestate: Receiver<String>) -> Engine {
        let (glutin_window, glgraphics) = window.create();
        Engine {
            glutin_window,
            glgraphics,
            event_qeue: Engine::setup_eventloop(),
            gamestate,
        }
    }


    // Make a new Piston Eventloop
    fn setup_eventloop() -> Events {
        let e = Events::new(EventSettings::new());
        return e
    }

    // Handle incoming OpenGL Events
    //pub fn handle_events(&mut self, app: &mut App) {
    pub fn handle_events(&mut self) {
        while let Some(event) = &mut self.event_qeue.next(&mut self.glutin_window) {

            if let Some(render_args) = event.render_args() {
                use self::graphics::*;
                self.glgraphics.draw(render_args.viewport(), |c, gl|{
                    clear([ 0f32, 0f32, 0f32, 0f32], gl)
                });
                if let Ok(value) = self.gamestate.try_recv() {
                    println!("{:?}", value);
                    continue;
                }
            }

        }
    }

}


/*
 * Making a Container of Rendering Functions aka the OpenGL-App
 *
 */

pub struct App {
    gl: GlGraphics,
}

impl App {
    pub fn new(opengl: OpenGL) -> App {
        App {
            gl: GlGraphics::new(opengl)
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        use self::graphics::*;

        self.gl.draw(args.viewport(), |c, gl| {
            clear([ 0f32, 0f32, 0f32, 0f32], gl)
        });
    }

}
