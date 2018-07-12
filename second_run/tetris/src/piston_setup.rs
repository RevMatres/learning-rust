extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use self::piston::window::WindowSettings;
use self::piston::event_loop::*;
use self::piston::input::*;
use self::glutin_window::GlutinWindow as Window;
use self::opengl_graphics::{ GlGraphics, OpenGL };

use std::sync::mpsc::Receiver;

/// Basic structure for an OpenGL window
pub struct App {
    gl: GlGraphics  // OpenGL drawing backend.
}

impl App {
    /// Function to create a new OpenGL thing
    pub fn new(opengl: OpenGL) -> App {
        let app = App {
            gl: GlGraphics::new(opengl)
        };
        
        app
    }

    /// Method to Render, duh
    fn render(&mut self, args: &RenderArgs) {
        use self::graphics::*;

        // Draw the Window Background
        self.gl.draw(args.viewport(), |c, gl| {
            clear([ 0.5, 0.5, 0.5, 0f32 ], gl)
        });
    }
    
    fn render_game(&mut self, args: &RenderArgs) {
        use self::graphics::*;

        // Draw the Window Background
        self.gl.draw(args.viewport(), |c, gl| {
            clear([ 0f32, 0f32, 0f32, 0f32 ], gl)
        });
    }
}


/// Return a Glutin OpenGL context.
///
/// ### Input Parameters
/// `tile`: the window's title  
/// `size`: an i32-array of 2 values, width and height
pub fn setup_glutin_window(title: &str, size: [u32; 2], opengl: OpenGL) -> Window {
    WindowSettings::new(title, size)
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap()
}

pub fn setup_eventloop() -> Events {
    let mut e = Events::new(EventSettings::new());
    return e
}

pub fn handle_events(event_qeue: &mut Events, window: &mut Window, app: &mut App, gamestate: &Receiver<String>) {
    while let Some(event) = event_qeue.next(window) {
        // If the returned event is an Input::Render(RenderArgs)
        if let Some(render) = event.render_args() {
            if let Ok(value) = gamestate.try_recv() {
                app.render_game(&render);
                continue;
            }

            app.render(&render);
        }
    }
}



/*
/*
 * CREATE AN INSTANCE OF THE GAME
 *
 */

// Create a new game instance
let mut app = App::new(opengl);



/*
 * EVENT HANDLING
 *
 */

// Create a message for exit =)
let msg = "\n__        ___   _ _____ _____ _____ _____ _____ _____ _ 
\\ \\      / / | | | ____| ____| ____| ____| ____| ____| |
\\ \\ /\\ / /| |_| |  _| |  _| |  _| |  _| |  _| |  _| | |
\\ V  V / |  _  | |___| |___| |___| |___| |___| |___|_|
\\_/\\_/  |_| |_|_____|_____|_____|_____|_____|_____(_)";

// Create an Event Loop
// that's an event qeue
let mut event_qeue = Events::new(EventSettings::new());

// Work through the event qeue
// each event is of the Piston::Input-type
while let Some(event) = event_qeue.next(&mut window) {

    // Check which Variant of Piston::Input exists
    // Since the event is the enum-variant, we can check for its fields.
    // Each variant has an associated function, that returns an Option containing its arguments
    // in the Some-variant. That means, all methods, that belong to a variant, that isn't the
    // current event's variant, will simply return None, only the one of the active type, will
    // return a Some. That way the Input-variant can be found and appropriately handled.

    // You would think, you could also match event against the Variants of Input. This way is
    // less convenient here, because the App-instance app, whose methods handle the render and
    // update events, would have to be brought into the scope of the match-block. 'if let'
    // doesn't create a local scope, so there's no extra problem.
    // Also, you can't actually use a match-block, because for some reason, it won't find the
    // Variants of Input you try to match against... IDK.

    // If the returned event is an Input::Render(RenderArgs)
    if let Some(render) = event.render_args() {
        // Render!
        app.render(&render);
    }

    // If the returned event is an Input::Update(UpdateArgs)
    if let Some(update) = event.update_args() {
        let r = rand::random::<f32>();
        let g = rand::random::<f32>();
        let b = rand::random::<f32>();
        app.update(&update, [ r, g, b, 1f32])
    }

    // If the returned event is an Input::Press(Button)
    if let Some(press) = event.press_args() {
        if let Button::Keyboard(key) = press {
            println!("Key Down on {:?}", key);

            // If the Key-down is Escape, print a message for the exit =)
            if let Key::Escape = key {
                println!("{}", msg);
            }
        }
    }

    // If the returned event is an Input::Release(Button)
    if let Some(release) = event.release_args() {
        if let Button::Keyboard(key) = release {
            println!("Key Up on   {:?}\n", key);
        }
    }

    // If the returned event is an Input::Close(CloseArgs)
    if let Some(close) = event.close_args() {
        println!("{}", msg);
    }

}
*/
