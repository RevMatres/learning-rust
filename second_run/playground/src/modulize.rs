use std::sync::Arc;
use std::sync::{RwLock, RwLockWriteGuard, RwLockReadGuard};

/// the state the game is in: either Active or Over
pub enum GameState {
    Active(TetrisBoard),
    Over,
}

impl GameState {
    /// creates a new game instance containing a `TetrisBoard`
    ///
    /// initializes with a `TetrisBoard`, cause a game doesn't start Game Over
    pub fn new(board: TetrisBoard) -> GameState {
        GameState::Active(board)
    }

    /// change from Active to Game Over
    pub fn game_over(mut self) {
        self = GameState::Over
    }

}

/// contains a tetris board
///
/// This type provides the public API for the tetris board data. It contains an RwLock, that
/// contains the board's data. The type provides public and private methods to access the Lock.
pub struct TetrisBoard(RwLock<String>);

impl TetrisBoard {
    /// create a new tetris board
    pub fn new(string: &str) -> TetrisBoard {
        TetrisBoard(RwLock::new(string.to_string()))
    }

    /// get the write access to the RwLock, that contains the board's data
    fn get_write(&self) -> RwLockWriteGuard<String> {
        let lock = self.0.try_write();
        match lock {
            Ok(write_lock) => return write_lock,
            Err(_e) => self.get_write(),
        }
    }

    /// get the read lock to the RwLock, that contains the board's data
    pub fn get_read(&self) -> RwLockReadGuard<String> {
        let read_lock = self.0.read().expect("Couldn't acquire a read-lock on the board data. That means the lock's poisoned!");
        return read_lock
    }
}

pub fn add_to_board(a: char, gs: Arc<GameState>) {
    match *gs {     // you have to explicitely dereference here, cause auto deref of complex types in patterns only works, when you have access to the type's implementation; you don't have public access to the impl of Arc
        GameState::Over => {},
        GameState::Active(ref b) => {
            let mut board = b.get_write();
            board.push(a);
        }
    }
}

