//!# Message Passing between the Threads of this thing
//!
//!Whenever the Tetris thread has changed something in the game state container, `Game`, it sends
//!a Reference to this `Game` down the game_state channel. Whenever a reference is received, new
//!values need to be send to the Player (ultimately AI) and Renderer, so new Inputs can be
//!generated and a rerender can happen.
//!
//!There's one peculiarity with all of this, though.  
//!All of the messages are references to the same Allocation, meaning all messages, **no matter
//!when they were send**, point to the newest data and such have the same actual content. That
//!means, that, if the Renderer is called, while more than one message is in its Receiver, it only
//!has to rerender *once* and can ignore all the other messages it just received.
//!
//!## How the handling of bubbling messages in a Receiver works
//!
//!You can create an iterator from a channel receiver. This iterator is open ended,
//!because the channel is going to add any new incoming messages to the iterator, even
//!if iteration is already in progress (iterators are lazy, so they won't block).
//!
//!Using `.recv()` moves the oldest received message out of the Receiver and returns
//!it. After the move it's not in the Receiver's buffer anymore.
//!
//!To keep the Tetris Game State Queues empty, I get the first incoming message,
//!acquire the `Read Lock` on the `Game` object, that is referenced in the message,
//!blocking the Tetris thread from acquiring the `Read Lock` and changing the `Game` in
//!the process (so no new messages being added to the iterator, while I'm not looking).
//!Then, keeping the `Read Lock`, I iterate over the rest of the messages
//!in the queue, without doing anything to them, emptying the Queue.
//!
//!There is still a possibility, that a change to the game state is missed, because
//!the message for it, while having been send, while nothing tried for a `Read Lock`,
//!still took time to arrive in the target `Receiver`. But since the Functions at the
//!Receiving ends of the channels block the Tetris thread from sending new messages.
//!
//!Such bad timing won't happen often and because the Queues are being emptied continuously, it
//!won't matter. Every time the Queue is emptied, the problem vanishes; the change in state will
//!just be handled by the next iteration of the loop, that keeps receiving and emptying.
//!
//!If I want to keep passing around references to `Game`, I can't sync the threads up
//!any better than this, without introducing explicit waiting periods on each of the
//!threads, which would slow operation down significantly.
//!It would probably better anyways, to just pass along 'update!' events and have the
//!threads keep one and the same `Arc` to `Game` through the entire lifetime of
//!Tetris. (I can't be bothered to rewrite it now, ¯\_(ツ)_/¯)
//!
//!**Note:** It's still necessary to suspend the various threads temporarily, like for a
//!millisecond. You need to give the CPU time to switch threads, otherwise you're definitely gonna
//!get out of sync with the messaging.

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


        // do a couple changes to the game data and state,
        // then send them on their merry way back to the main thread
        for i in 0..5 {
            change(Arc::clone(&tetris_game), '!', GS::Active(i*9));
            tx.send(Arc::clone(&tetris_game)).unwrap();

            // and then send some more messages to main, so we can watch the Receiver Queue being emptied
            tx.send(Arc::clone(&tetris_game)).unwrap();
            tx.send(Arc::clone(&tetris_game)).unwrap();
            tx.send(Arc::clone(&tetris_game)).unwrap();

            // give the CPU time to switch threads
            thread::sleep(Duration::from_millis(1));
        }

        // finally, set the game to be over,
        // send it to main
        change(Arc::clone(&tetris_game), '?', GS::Over(10000));
        tx.send(Arc::clone(&tetris_game)).unwrap();

    });



    // receive messages from the tetris thread
    loop {
        
            /* TODO: make this file sensible again, use try_iter(), commit
             *  - rename files and remove deprecations, commit
             *  - actually port this mess into the tetris thing, commit
             *  - put these files to rest in the playground, commit
             */

        // get the first message in the Receiver
        let game = rx.try_recv();

        // if that worked
        if let Ok(ref game) = game {

            // render the game state with this closure
            let p = |s, b| {
                println!("The score is: {}", s);
                println!("Board state:\n{}", b);
            };

            // handle the state the game is in
            match game.state.get_state() {

                // if the game is still happening
                GS::Active(score) => {
                    // print score and game state
                    p(score, game.data.get_read());

                    // empty the Receiver's message Queue
                    let messages = rx.try_iter();
                    for _m in messages {
                        println!("emptying message queue");
                        continue;
                    }
                },

                // if the game is over
                GS::Over(fscore) => {   // fscore for final score
                    // print final score and final game state
                    p(fscore, game.data.get_read());

                    // stop reading from the Receiver
                    break;
                },
            }

        }

    }

    tetris_handler.join().unwrap();

}
