//!
//! Game logic behind the SOS game
//!

mod board;
use board::Board;

/// Enumerates the possible SOS cell values
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Cell {EMPTY, S, O}

/// Enumerates the different game modes
pub enum Mode {CLASSIC, SIMPLE}

/// Enumerates player turns
#[derive(Eq, PartialEq, Debug)]
pub enum Turn {LEFT, RIGHT}

/// Contains game data such as board state, game mode, and player turn
pub struct Game {
    board: Board<Cell>,
    mode: Mode,
    turn: Turn
}

impl Game {
    pub fn new(board_size: usize, mode: Mode) -> Self {
        Self {
            board: Board::new(board_size, board_size, Cell::EMPTY),
            mode,
            turn: Turn::LEFT
        }
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
        if self.board.get_value(x, y) == Ok(&Cell::EMPTY) {
            if self.board.set_cell(x, y, input) == Ok(()) {
                self.switch_turn();
            }
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
        assert_eq!(g.board.get_value(4, 6), Ok(&Cell::S));
    }

    #[test]
    fn make_move_does_nothing_when_coord_not_empty() {
        let mut g = Game::new(10, Mode::SIMPLE);
        g.make_move(4, 6, Cell::S);
        g.make_move(4, 6, Cell::O);
        assert_eq!(g.board.get_value(4, 6), Ok(&Cell::S));
    }
}