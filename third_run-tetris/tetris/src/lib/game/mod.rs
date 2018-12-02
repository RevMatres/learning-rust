pub mod game_types;

struct Block { // a single block
	x:i32,
	y:i32,
}

struct Blocks(Block, Block, Block, Block);

pub fn game() {
    println!("game was requested to send a message from game_types:");
    game_types::color_says_hello();
    println!("end of transmission");

    let mut b: Blocks = Blocks(Block{x:0,y:0},Block{x:1,y:0},Block{x:1,y:1},Block{x:0,y:1});
    println!("{}",b.2.x);
}
