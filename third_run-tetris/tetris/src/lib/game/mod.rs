pub mod game_types;

pub fn game() {
    println!("game was requested to send a message from game_types:");
    game_types::color_says_hello();
    println!("end of transmission");
}
