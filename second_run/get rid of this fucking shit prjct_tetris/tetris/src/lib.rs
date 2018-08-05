extern crate channels;
pub mod tetris;
pub mod game_types;

pub fn go() {
    println!("hi from tetris");

    let wicked = game_types::Hello::create("I'm a Hello Type");
    println!("{}\n", wicked.field);

    tetris::go();
}
