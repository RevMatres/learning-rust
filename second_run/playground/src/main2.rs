use std::thread;
use std::time::Duration;
use std::sync::Arc;
use std::sync::mpsc::channel;

mod modulize_2;
use modulize_2::*;

fn main() {

    // create message channels for communicating with the tetris thread
    let (tx, rx) = channel();

    // create the tetris thread
    let tetris_handler = thread::spawn(move || {

        // move the trasmitter into the tetris thread
        let tx = tx;

        // create a Game object for tetris,
        // put it in an Arc for sending across threads
        let tetris_game: Arc<Game> = Arc::new(
            Game {
                data: GameData::new("wheee"),
                state: GameState::new(0),
            }
        );


        // do a couple changes to the game data and state
        // and then send them on their merry way back to the main thread
        for i in 0..5 {
            change(Arc::clone(&tetris_game), '!', "Active", 9*i);
            tx.send(Arc::clone(&tetris_game)).unwrap();
            thread::sleep(Duration::from_secs(1));
        }

        // finally, set the game to be over
        // and send that back to main
        change(Arc::clone(&tetris_game), '?', "Over", 10000);
        tx.send(Arc::clone(&tetris_game));

    });



    // receive messages from the tetris thread
    loop {
        
        let recv_stack = rx.try_recv();
        if let Ok(Game) = recv_stack {

            if let GS::Active(score) = Game.state.get_state() {
                println!("The current score is: {}", score);
                let rl = Game.data.get_read();
                println!("{}", rl);
            };

            if let GS::Over(fscore) = Game.state.get_state() {
                println!("The final score is: {}", fscore);
                let rl = Game.data.get_read();
                println!("{}", rl);
            };

        }

    }

    tetris_handler.join();

}


