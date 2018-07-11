// Import the Piston Game Engine
extern crate piston;
extern crate graphics;

// Import the Glutin-Plugin for the Piston Engine
// Glutin is an OpenGL context creation library, but doesn't directly provide OpenGL bindings
extern crate glutin_window;

// Import the OpenGL-Plugin for the Piston Engine
// OpenGL is used as the renderer here; this crate provides the bindings
extern crate opengl_graphics;

// Note: Piston only functions as an API for calculating game logic, but needs a windowing,
// rendering and event handling plugin. Here Glutin and OpenGL are used. Glutin creates OpenGL
// contexts, which basically means 'window'. OpenGL offers access to the manipulations of that
// context.

extern crate rand;

// Use statements for piston
use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };



pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    color: [f32; 4] // the background color of the window
}

impl App {
    fn new(opengl: OpenGL) -> App {

        // Create the App
        let app = App {
            gl: GlGraphics::new(opengl),
            color: [ 0.5, 0.5, 0.5, 1.0 ]   // initialize on grey
        };
        
        app
    }

    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        // Make a copy of the color-attribute of App
        // needed, cause of the closure below
        let color = self.color;

        // Draw the Window Background
        self.gl.draw(args.viewport(), |c, gl| {

            // Fill background with color
            clear(color, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs, color: [f32; 4]) {

        // Update color
        self.color = color;
    }
}



fn main() {

    /*
     * SETUP THE WINDOW
     *
     */

    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new(
            "spinning-square",
            [500, 1000]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();



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
}
