// IMPORTS

use std::thread;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::time::Duration;
use std::sync::{RwLock, RwLockReadGuard};

/// Function to create a thread, in wich the Tetris logic runs
///
/// ### But what exactly does this do?
/// `tetris_setup` first creates to channels, one for passing events to the Tetris game and one for
/// the game to return a reference to its game-state object, so that a renderer or AI can access
/// this info.
///
/// ### Event-handling channel
/// Via this channel one can send Events to the Tetris game. The sent events must be a Variant of
/// the `Tetris_Event` type.
///
/// ### Game-State channel
/// Via this channel the game sends an immutable borrow to its Game-State object. This object is a
/// two-dimensional collection (outer collection contains the rows, each row contains a 'field')
/// representing the current state of the tetris game.
///
/// **After using the Game-State the reference *must* be dropped, or the Tetris game can't continue
/// its operation!**
pub fn tetris_setup() -> (thread::JoinHandle<()>, Sender<String>, Receiver<RwLockReadGuard<String>>) {
    // Create message-channels
    let (events_tx, events_rx) = channel();
    let (game_state_tx, game_state_rx) = channel();

    // Create the Tetris thread
    let handle = thread::spawn(move || {
        run(events_rx, game_state_tx);
    });
    
    return (handle, events_tx, game_state_rx)
}

/// This function contains and executes the Tetris game logic
fn run(events_rx: Receiver<String>, game_state_tx: Sender<RwLockReadGuard<String>>) {
    
    // Game Logic Setup
    let board_lock = RwLock::new(String::new());

    // Game Loop
    loop {
        // make a block
        // add block to board

        // Block Loop
        loop {
            // acquire write lock on board
            let board = board_lock.write().unwrap();

            board.push('a');
            // listen to timer
            // listen to events
            // transform, check for collisions
            // remove block from board
            // add block to board

            drop(board);

            // SEND GAMESTATE
            game_state_tx.send(board_lock.read().unwrap());
        }
        
        // handle scoring
        // SEND GAMESTATE

    }
    // logic setup: makes a board
    // game loop: makes a new block, handles merging and the scores, etc
    // block loop: while a block is moving, ya check for times and events etc
}
