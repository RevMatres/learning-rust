use std::thread;
use std::sync::Arc;
use std::sync::mpsc::channel;
use std::time::Duration;
mod modulize;
use modulize::*;

fn main() {


    /* TODO:
     *  - you can't borrow the GameState inside an Arc mutably... FIGURE THAT OUT
     *     ✗ there was a thing like this in the Rust Book
     *     - restructure API? -> GameState into a struct with a flag-field for game_over
     *  - implement this code in the actual tetris thingy
     *  - document the code: both in API docs and a detailed explanation with in-line comments,
     *    where necessery
     *  - your receivers need to make sure, that their stacks are always at 0 length, or the
     *    threads will go out of sync
     *  - backup files from playground, commit, reset playground to empty main.rs
     *
     */


    let (tx, rx) = channel();

    let tetris_handler = thread::spawn(move || {

        let tx = tx;

        let game_state: Arc<GameState> = Arc::new(
            GameState::new( TetrisBoard::new("wheee") )
        );

        for _i in 0..5 {
            let a = Arc::clone(&game_state);
            tx.send(a).unwrap();
            thread::sleep(Duration::from_secs(1));

            add_to_board('!', Arc::clone(&game_state));
        }


        game_state.game_over();

        tx.send(Arc::clone(&game_state)).unwrap();

    });

    loop {
        
        let received = rx.try_recv();
        if let Ok(gamestate) = received {
            match *gamestate {
                GameState::Over => {
                    println!("Game Over");
                    break
                },
                GameState::Active(ref b) => {
                    let board = b.get_read();
                    println!("{}", board);
                }
            }
        }

    }

    tetris_handler.join().unwrap();
}
