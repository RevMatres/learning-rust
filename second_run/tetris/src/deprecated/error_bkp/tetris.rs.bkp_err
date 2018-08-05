use std::thread;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::time::Duration;

pub fn tetris_setup() -> (thread::JoinHandle<()>, Sender<String>, Receiver<String>) {
    // Create message-channels
    let (event_tx, event_rx) = channel();
    let (gameS_tx, gameS_rx) = channel();

    // Create the Tetris thread
    let handle = thread::spawn(move || {
        let event_rx = event_rx;
        let gameS_tx = gameS_tx;
        loop {
            thread::sleep(Duration::from_secs(1));
            gameS_tx.send("wheeeeeee-zaaaaaaah".to_string()).unwrap();
            run();
        }
    });
    
    return (handle, event_tx, gameS_rx)
}

fn run() {
    println!("Wheeee")
}
