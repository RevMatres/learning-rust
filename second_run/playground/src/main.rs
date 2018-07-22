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
            change(Arc::clone(&tetris_game), '!', GS::Active(i*9));
            tx.send(Arc::clone(&tetris_game)).unwrap();
            tx.send(Arc::clone(&tetris_game)).unwrap();
            tx.send(Arc::clone(&tetris_game)).unwrap();
            thread::sleep(Duration::from_secs(1));
        }

        // finally, set the game to be over
        // and send that back to main
        change(Arc::clone(&tetris_game), '?', GS::Over(10000));
        tx.send(Arc::clone(&tetris_game)).unwrap();

    });



    // receive messages from the tetris thread
    loop {
        
        let recv_stack = rx.try_recv();
        thread::sleep(Duration::from_millis(900));
        for (i, v) in recv_stack.iter().enumerate() {
            println!("{}", i);
        }
        if let Ok(ref game) = recv_stack {

            //thread::sleep(Duration::from_millis(900));

            if let GS::Active(score) = game.state.get_state() {
                println!("The current score is: {}", score);
                let rl = game.data.get_read();
                println!("{}", rl);
            };

            if let GS::Over(fscore) = game.state.get_state() {
                println!("The final score is: {}", fscore);
                let rl = game.data.get_read();
                println!("{}", rl);
                break;
            };

        }

    }

    tetris_handler.join().unwrap();

}


// OK

