use std::sync::Arc;
use std::sync::{RwLock, RwLockWriteGuard, RwLockReadGuard};

/// Container for all of the Game's data
pub struct Game {
    pub data: GameData,
    pub state: GameState,
}



/// Container for the Game's actual state data
///
/// To moderate read and write access across threads GameData contains an RwLock and provides the
/// private get_write() and the public get_read() methods, which return RwLock-Guards.
pub struct GameData(RwLock<String>);

impl GameData {
    /// creates a new GameData object
    pub fn new(string: &str) -> GameData {
        GameData(RwLock::new(string.to_string()))
    }

    /// get write access to the game data in the GameData's RwLock
    fn get_write(&self) -> RwLockWriteGuard<String> {
        let w_lock = self.0.write().expect("GameData has been poisoned!");
        return w_lock
    }

    /// get read access to the game data in the GameData's RwLock
    pub fn get_read(&self) -> RwLockReadGuard<String> {
        let r_lock = self.0.read().expect("GameData has been poisoned!");
        return r_lock
    }
}



/// the actual State the Game is in
///
/// Can be ::Active(Score) and ::Over(FinalScore)
#[derive(Clone)]
pub enum GS {
    Active(Score),
    Over(FinalScore),
}

/// wrapper for the game state encoded with GS, provides the public API
///
/// this is needed for interior mutability
pub struct GameState (RwLock<GS>);

/// Score and FinalScore alias types
type Score = i32;
type FinalScore = i32;



impl GameState {
    /// create a new GameState object
    ///
    /// This returns a GameState set to GS::Active, because a game doesn't start at Game Over
    pub fn new(score: Score) -> GameState {
        GameState (
            RwLock::new(GS::Active(score))
        )
    }

    /// change the game state to a new state and score
    pub fn set_state(&self, state: GS) {
        // get a write lock
        let mut l = self.0.write().expect("GameState has been poisoned!");

        // set the state
        *l = state
    }

    /// returns just the game state, as in the GS inside GameState
    pub fn get_state(&self) -> GS {
        // get a read lock on the game state
        let gs = self.0.read().expect("GameSate has been poisoned!");

        // return a duplicate of the game state
        return gs.clone()
    }
}



/// a testing function, that changes the gamedata and state
/// NOTE: this function can be shitty, cause ultimately I'm only gonna use private tetris functions
/// to actually change the game data and state. I need a testing function in this module to do this
/// from main.rs though...
pub fn change(g: Arc<Game>, a: char, b: GS) {
    let mut l = g.data.get_write();
    l.push(a);

    g.state.set_state(b);
}

