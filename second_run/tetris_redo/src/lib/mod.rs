// IMPORTS
mod tetris;
mod engine;

pub fn lib_hello() {
    engine::engine_hello();
    println!("Hello from LIB");

}

// ultimately create unified setup and run function called go()
// that's an abstraction for when everything works!
//
// ultimately I also still need to write proper documentation for all of this
// and I also need to format my comments with headers and stuff, but now let's start
// by making the tetris module and implementing the tetris logic,
// and adding the code for the eventloop-tetris interaction,
// then comments
// then the rendering
// then comments
// then go() abstraction
// then comments
// then ai input modularization
// then ocmments
// then the genetic alg
// then comments
pub fn make_window() {
    let w = engine::Window::new("Tetris!", [400, 800]);
    let mut e = engine::Engine::new(w);
    e.handle_events();
}
