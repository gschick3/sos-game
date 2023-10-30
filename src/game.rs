//!
//! Game logic behind the SOS game
//!

use std::fmt::Error;

/// Enumerates the possible SOS cell values
#[derive(Clone, PartialEq, Debug)]
pub enum Cell { Empty, S, O}

/// Enumerates the different game modes
#[derive(Clone, PartialEq, Debug)]
pub enum Mode { Classic, Simple }

/// Enumerates player turns
#[derive(PartialEq, Debug)]
pub enum Player { Left, Right }

#[derive(PartialEq, Debug)]
pub enum State { LeftWin, RightWin, Draw, Playing, NotStarted }

/// Contains game data such as board state, game mode, and player turn
pub struct Game {
    board: Vec<Vec<Cell>>, // TODO: If cell history required (i.e. who placed/scored), encapsulate Cells into structs
    pub turn: Player,
    // example trait usage: https://doc.rust-lang.org/book/ch17-03-oo-design-patterns.html
    game_type: Option<Box<dyn WinCondition>>,
    cells_filled: usize,
    pub left_score: u32,
    pub right_score: u32,
    pub game_state: State
}

impl Game {
    pub fn new(board_size: usize, mode: Mode) -> Self {
        Self {
            board: vec![vec![Cell::Empty; board_size]; board_size],
            turn: Player::Left,
            game_type: match mode {
                Mode::Classic => Some(Box::new(ClassicGame {})),
                Mode::Simple => Some(Box::new(SimpleGame {}))
            },
            cells_filled: 0,
            left_score: 0,
            right_score: 0,
            game_state: State::NotStarted
        }
    }

    pub fn get_board_size(&self) -> usize {
        return self.board.len();
    }

    pub fn clear_grid(&mut self) {
        let board_len = self.board.len();
        self.board.clear();
        self.board.resize(board_len, vec![Cell::Empty; board_len]);
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
        if self.valid_cell(x, y) && self.board[y][x] == Cell::Empty && self.game_state == State::Playing {
            self.board[y][x] = input;
            self.cells_filled += 1;
            match self.turn {
                Player::Left => self.left_score += self.sos_made(x, y),
                Player::Right => self.right_score += self.sos_made(x, y)
            }
            self.game_state = self.game_type.as_ref().unwrap().get_game_state(self);
            self.switch_turn();
        }
    }

    fn valid_cell(&mut self, x: usize, y: usize) -> bool {
        x < self.board.len() && y < self.board.len()
    }

    pub fn get_cell(&mut self, x: usize, y: usize) -> Result<&Cell, Error> {
        match self.valid_cell(x, y) {
            true => Ok(&self.board[y][x]),
            false => Err(Error)
        }
    }

    fn switch_turn(&mut self) {
        self.turn = match self.turn {
            Player::Left => Player::Right,
            Player::Right => Player::Left
        };
    }

    fn board_full(&self) -> bool {
        self.cells_filled == self.board.len().pow(2)
    }

    fn sos_made(&mut self, x: usize, y: usize) -> u32 {
        let mut count: u32 = 0;

        match self.board[y][x] {
            Cell::O => {
                if y > 0 && y < self.board.len()-1 {
                    if self.board[y-1][x] == Cell::S && self.board[y+1][x] == Cell::S {
                        count += 1;
                    }
                    if x > 0 && x < self.board.len()-1 {
                        if self.board[y-1][x+1] == Cell::S && self.board[y+1][x-1] == Cell::S {
                            count += 1;
                        }
                        if self.board[y+1][x+1] == Cell::S && self.board[y-1][x-1] == Cell::S {
                            count += 1;
                        }
                    }
                }
                if x > 0 && x < self.board.len()-1 {
                    if self.board[y][x+1] == Cell::S && self.board[y][x-1] == Cell::S {
                        count += 1;
                    }
                }
            },
            Cell::S => {
                if y > 1 {
                    if self.board[y-1][x] == Cell::O && self.board[y-2][x] == Cell::S {
                        count += 1;
                    }
                    if x > 1 {
                        if self.board[y-1][x-1] == Cell::O && self.board[y-2][x-2] == Cell::S {
                            count += 1;
                        }
                    }
                    if x < self.board.len()-2 {
                        if self.board[y-1][x+1] == Cell::O && self.board[y-2][x+2] == Cell::S {
                            count += 1;
                        }
                    }
                }
                if y < self.board.len()-2 {
                    if self.board[y+1][x] == Cell::O && self.board[y+2][x] == Cell::S {
                        count += 1;
                    }
                    if x > 1 {
                        if self.board[y+1][x-1] == Cell::O && self.board[y+2][x-2] == Cell::S {
                            count += 1;
                        }
                    }
                    if x < self.board.len()-2 {
                        if self.board[y+1][x+1] == Cell::O && self.board[y+2][x+2] == Cell::S {
                            count += 1;
                        }
                    }
                }
                if x > 1 {
                    if self.board[y][x-1] == Cell::O && self.board[y][x-2] == Cell::S {
                        count += 1;
                    }
                }
                if x < self.board.len()-2 {
                    if self.board[y][x+1] == Cell::O && self.board[y][x+2] == Cell::S {
                        count += 1;
                    }
                }
            },
            _ => ()
        }
        count
    }
}

trait WinCondition {
    fn get_game_state(&self, game: &Game) -> State;
}

struct ClassicGame {}
impl WinCondition for ClassicGame {
    fn get_game_state(&self, game: &Game) -> State {
        // Game not yet over
        if !game.board_full() {
            return State::Playing;
        }

        // If game board is full, game is over
        if game.left_score > game.right_score {
            State::LeftWin
        } else if game.right_score > game.left_score {
            State::RightWin
        } else {
            State::Draw
        }
    }
}

struct SimpleGame {}
impl WinCondition for SimpleGame {
    fn get_game_state(&self, game: &Game) -> State {
        if game.left_score > game.right_score {
            return State::LeftWin;
        } else if game.right_score > game.left_score {
            return State::RightWin;
        } else if game.board_full() {
            return State::Draw;
        }

        State::Playing
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn game_starts_at_given_size() {
        let g = Game::new(10, Mode::Classic);
        assert!(g.board.len() == 10 && g.board[0].len() == 10);
    }

    #[test]
    fn turn_starts_on_left() {
        let g = Game::new(10, Mode::Simple);
        assert_eq!(g.turn, Player::Left);
    }

    #[test]
    fn switch_turn_left_to_right() {
        let mut g = Game::new(10, Mode::Simple);
        g.switch_turn();
        assert_eq!(g.turn, Player::Right);
    }

    #[test]
    fn switch_turn_right_to_left() {
        let mut g = Game::new(10, Mode::Simple);
        g.switch_turn();
        g.switch_turn();
        assert_eq!(g.turn, Player::Left);
    }

    #[test]
    fn can_make_move_when_coord_empty() {
        let mut g = Game::new(10, Mode::Simple);
        g.game_state = State::Playing; // must be in Playing state before make_move is called
        g.make_move(4, 6, Cell::S);
        assert_eq!(g.board[6][4], Cell::S);
    }

    #[test]
    fn switches_turn_when_valid_move_made() {
        // Game starts on Turn::LEFT
        let mut g = Game::new(10, Mode::Simple);
        g.game_state = State::Playing;
        g.make_move(4, 6, Cell::S);
        assert_eq!(g.turn, Player::Right);
    }

    #[test]
    fn do_not_make_move_when_coord_not_empty() {
        let mut g = Game::new(10, Mode::Simple);
        g.game_state = State::Playing;
        g.make_move(4, 6, Cell::S);
        g.make_move(4, 6, Cell::O);
        assert_eq!(g.board[6][4], Cell::S);
    }

    #[test]
    fn does_not_switch_turn_when_coord_not_empty() {
        // Game starts on Turn::LEFT
        let mut g = Game::new(10, Mode::Simple);
        g.game_state = State::Playing;
        g.make_move(4, 6, Cell::S);
        g.make_move(4, 6, Cell::O);
        assert_eq!(g.turn, Player::Right);
    }

    #[test]
    fn do_not_make_move_when_coord_invalid() {
        let mut g = Game::new(5, Mode::Simple);
        g.game_state = State::Playing;
        g.make_move(4, 6, Cell::S);
        assert_eq!(g.board, vec![vec![Cell::Empty; 5]; 5]);
    }

    #[test]
    fn does_not_switch_turn_when_invalid_move_made() {
        // Game starts on Turn::LEFT
        let mut g = Game::new(10, Mode::Simple);
        g.game_state = State::Playing;
        g.make_move(10, 6, Cell::S);
        assert_eq!(g.turn, Player::Left);
    }

    #[test]
    fn clear_grid_does_not_change_size() {
        let mut g = Game::new(10, Mode::Simple);
        g.game_state = State::Playing;
        g.make_move(4, 6, Cell::S);
        g.make_move(5, 5, Cell::O);
        g.clear_grid();
        assert_eq!(g.board, vec![vec![Cell::Empty; 10]; 10]);
    }

    #[test]
    fn get_cell_out_of_bounds_creates_error() {
        let mut g = Game::new(10, Mode::Simple);
        let result = g.get_cell(9, 10);
        assert_eq!(result, Err(Error));
    }

    #[test]
    fn get_cell_in_bounds_returns_correct_value() {
        let mut g = Game::new(10, Mode::Simple);
        g.game_state = State::Playing;
        g.make_move(5, 4, Cell::S);
        let result = g.get_cell(5, 4);
        assert_eq!(result, Ok(&Cell::S));
    }

    #[test]
    fn left_player_wins_simple_game() {
        let mut g = Game::new(10, Mode::Simple);
        g.game_state = State::Playing;
        g.make_move(1, 2, Cell::O); // Left
        g.make_move(0, 1, Cell::S); // Right

        g.make_move(1, 1, Cell::O); // Left
        g.make_move(2, 2, Cell::S); // Right

        g.make_move(2, 3, Cell::S); // Left

        assert_eq!(g.game_state, State::LeftWin);
    }

    #[test]
    fn right_player_wins_simple_game() {
        let mut g = Game::new(10, Mode::Simple);
        g.game_state = State::Playing;
        g.make_move(1, 2, Cell::O); // Left
        g.make_move(0, 1, Cell::S); // Right

        g.make_move(1, 1, Cell::O); // Left
        g.make_move(2, 3, Cell::S); // Right

        assert_eq!(g.game_state, State::RightWin);
    }

    #[test]
    fn players_draw_simple_game() {
        let mut g = Game::new(3, Mode::Simple);
        g.game_state = State::Playing;

        g.make_move(0, 0, Cell::S);
        g.make_move(0, 1, Cell::S);
        g.make_move(0, 2, Cell::S);
        g.make_move(1, 0, Cell::S);
        g.make_move(1, 1, Cell::S);
        g.make_move(1, 2, Cell::S);
        g.make_move(2, 0, Cell::S);
        g.make_move(2, 1, Cell::S);
        g.make_move(2, 2, Cell::S);

        assert_eq!(g.game_state, State::Draw);
    }

    #[test]
    fn left_player_wins_classic_game() {
        let mut g = Game::new(3, Mode::Classic);
        g.game_state = State::Playing;
        g.make_move(0, 0, Cell::S); // Left
        g.make_move(0, 1, Cell::O); // Right

        g.make_move(0, 2, Cell::S); // Left
        g.make_move(1, 0, Cell::O); // Right

        g.make_move(1, 1, Cell::S); // Left
        g.make_move(1, 2, Cell::S); // Right

        g.make_move(2, 0, Cell::O); // Left
        g.make_move(2, 1, Cell::S); // Right

        g.make_move(2, 2, Cell::S); // Left

        assert_eq!(g.game_state, State::LeftWin);
    }

    #[test]
    fn right_player_wins_classic_game() {
        let mut g = Game::new(3, Mode::Classic);
        g.game_state = State::Playing;
        g.make_move(2, 2, Cell::S); // Left
        g.make_move(0, 1, Cell::O); // Right

        g.make_move(0, 2, Cell::S); // Left
        g.make_move(1, 2, Cell::S); // Right

        g.make_move(1, 1, Cell::S); // Left
        g.make_move(1, 0, Cell::O); // Right

        g.make_move(2, 0, Cell::O); // Left
        g.make_move(0, 0, Cell::S); // Right

        g.make_move(2, 1, Cell::S); // Left

        assert_eq!(g.game_state, State::RightWin);
    }

    #[test]
    fn players_draw_classic_game() {
        let mut g = Game::new(3, Mode::Classic);
        g.game_state = State::Playing;

        g.make_move(0, 0, Cell::S);
        g.make_move(0, 1, Cell::S);

        g.make_move(0, 2, Cell::S);
        g.make_move(1, 0, Cell::O);

        g.make_move(1, 1, Cell::O);
        g.make_move(1, 2, Cell::S);

        g.make_move(2, 1, Cell::O);
        g.make_move(2, 0, Cell::S);

        g.make_move(2, 2, Cell::S);

        assert_eq!(g.game_state, State::Draw);
    }
}