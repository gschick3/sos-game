//!
//! Game logic behind the SOS game
//!

use std::fmt::Error;

/// Enumerates the possible SOS cell values
#[derive(Clone, PartialEq, Debug)]
pub enum Cell {EMPTY, S, O}

/// Enumerates the different game modes
#[derive(Clone, PartialEq)]
pub enum Mode {CLASSIC, SIMPLE}

/// Enumerates player turns
#[derive(PartialEq, Debug)]
pub enum Turn {LEFT, RIGHT}

/// Contains game data such as board state, game mode, and player turn
pub struct Game {
    board: Vec<Vec<Cell>>,
    pub board_size: usize,
    mode: Mode,
    pub turn: Turn
}

impl Game {
    pub fn new(board_size: usize, mode: Mode) -> Self {
        Self {
            board: vec![vec![Cell::EMPTY; board_size]; board_size],
            board_size,
            mode,
            turn: Turn::LEFT
        }
    }

    pub fn clear_grid(&mut self) {
        self.board.clear();
        self.board.resize(self.board_size, vec![Cell::EMPTY; self.board_size]);
    }

    /// Make a move on the game board
    ///
    /// # Example
    ///
    /// ```
    /// use game::{Game, Mode, Cell};
    ///
    /// let mut g = Game::new(10, Mode::Classic);
    /// g.make_move(4, 3, Cell::S);
    /// ```
    pub fn make_move(&mut self, x: usize, y: usize, input: Cell) {
        if x < self.board.len() && y < self.board.len()
        && self.board[y][x] == Cell::EMPTY {
            self.board[y][x] = input;
            self.switch_turn();
        }
    }

    pub fn get_cell(&mut self, x: usize, y: usize) -> Result<&Cell, Error> {
        match x < self.board.len() && y < self.board.len() {
            true => Ok(&self.board[y][x]),
            false => Err(Error)
        }
    }

    fn switch_turn(&mut self) {
        self.turn = match self.turn {
            Turn::LEFT => Turn::RIGHT,
            Turn::RIGHT => Turn::LEFT
        };
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn turn_starts_on_left() {
        let g = Game::new(10, Mode::SIMPLE);
        assert_eq!(g.turn, Turn::LEFT);
    }



    #[test]
    fn switch_turn_left_to_right() {
        let mut g = Game::new(10, Mode::SIMPLE);
        g.switch_turn();
        assert_eq!(g.turn, Turn::RIGHT);
    }

    #[test]
    fn switch_turn_right_to_left() {
        let mut g = Game::new(10, Mode::SIMPLE);
        g.switch_turn();
        g.switch_turn();
        assert_eq!(g.turn, Turn::LEFT);
    }

    #[test]
    fn make_move_changes_board_when_coord_empty() {
        let mut g = Game::new(10, Mode::SIMPLE);
        g.make_move(4, 6, Cell::S);
        assert_eq!(g.board[6][4], Cell::S);
    }

    #[test]
    fn make_move_does_nothing_when_coord_not_empty() {
        let mut g = Game::new(10, Mode::SIMPLE);
        g.make_move(4, 6, Cell::S);
        g.make_move(4, 6, Cell::O);
        assert_eq!(g.board[6][4], Cell::S);
    }
    #[test]
    fn clear_grid_without_changing_size() {
        let mut g = Game::new(10, Mode::SIMPLE);
        g.make_move(4, 6, Cell::S);
        g.make_move(5, 5, Cell::O);
        g.clear_grid();
        assert_eq!(g.board, vec![vec![Cell::EMPTY; 10]; 10]);
    }
}