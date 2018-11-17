mod game;
mod renderer;

pub fn honk() {
    println!("Honk.");
    renderer::render();
    game::game();
}
