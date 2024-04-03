//!
//! Game logic behind the SOS game
//!

use std::fmt::Error;
use rand::Rng;
use crate::game_enums::{Mode, Cell, Turn, State};
use crate::recording::Recording;

/// Contains game data such as board state, game mode, and player turn
pub struct Game {
    board: Vec<Vec<Cell>>,
    pub turn: Turn,
    // example trait usage: https://doc.rust-lang.org/book/ch17-03-oo-design-patterns.html
    game_type: Option<Box<dyn WinCondition>>,
    cells_filled: usize,
    pub left_score: u32,
    pub right_score: u32,
    pub state: State,
    pub recording: Recording
}

impl Game {
    pub fn new(mode: Mode, board_size: usize) -> Self {
        Self {
            board: vec![vec![Cell::Empty; board_size]; board_size],
            turn: Turn::Left,
            game_type: match mode {
                Mode::Classic => Some(Box::new(ClassicGame {})),
                Mode::Simple => Some(Box::new(SimpleGame {}))
            },
            cells_filled: 0,
            left_score: 0,
            right_score: 0,
            state: State::NotStarted,
            recording: Recording::new(mode, board_size)
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
    pub fn make_move(&mut self, input: Cell, row: usize, col: usize) {
        if self.valid_cell(col, row) && self.board[row][col] == Cell::Empty && self.state == State::Playing {
            self.board[row][col] = input;
            self.cells_filled += 1;
            let sos_made = self.sos_made(col, row);
            match self.turn {
                Turn::Left => self.left_score += sos_made,
                Turn::Right => self.right_score += sos_made
            }
            self.recording.add_move(input, row, col);
            self.state = self.game_type.as_ref().unwrap().get_game_state(self);
            if sos_made == 0 {
                self.switch_turn();
            }
        }
    }

    pub fn make_random_move(&mut self) {
        if self.state != State::Playing {
            return
        }

        let mut rng = rand::thread_rng();

        let input = match rng.gen_range(0..=1) {
            0 => Cell::S,
            1 => Cell::O,
            _ => Cell::Empty
        };

        let mut row = rng.gen_range(0..self.board.len());
        let mut col = rng.gen_range(0..self.board.len());
        while self.board[row][col] != Cell::Empty {
            row = rng.gen_range(0..self.board.len());
            col = rng.gen_range(0..self.board.len());
        }
        self.make_move(input, row, col);
    }

    fn valid_cell(&mut self, col: usize, row: usize) -> bool {
        col < self.board.len() && row < self.board.len()
    }

    pub fn get_cell(&mut self, x: usize, y: usize) -> Result<&Cell, Error> {
        match self.valid_cell(x, y) {
            true => Ok(&self.board[y][x]),
            false => Err(Error)
        }
    }

    fn switch_turn(&mut self) {
        self.turn = match self.turn {
            Turn::Left => Turn::Right,
            Turn::Right => Turn::Left
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
        let g = Game::new(Mode::Classic, 10);
        assert!(g.board.len() == 10 && g.board[0].len() == 10);
    }

    #[test]
    fn turn_starts_on_left() {
        let g = Game::new(Mode::Simple, 10);
        assert_eq!(g.turn, Turn::Left);
    }

    #[test]
    fn switch_turn_left_to_right() {
        let mut g = Game::new(Mode::Simple, 10);
        g.switch_turn();
        assert_eq!(g.turn, Turn::Right);
    }

    #[test]
    fn switch_turn_right_to_left() {
        let mut g = Game::new(Mode::Simple, 10);
        g.switch_turn();
        g.switch_turn();
        assert_eq!(g.turn, Turn::Left);
    }

    #[test]
    fn can_make_move_when_coord_empty() {
        let mut g = Game::new(Mode::Simple, 10);
        g.state = State::Playing; // must be in Playing state before make_move is called
        g.make_move(Cell::S, 6, 4);
        assert_eq!(g.board[6][4], Cell::S);
    }

    #[test]
    fn make_random_move_makes_single_move() {
        let mut g = Game::new(Mode::Simple, 10);
        g.state = State::Playing; // must be in Playing state before make_move is called
        g.make_random_move();

        let mut count = 0;
        for line in g.board.clone() {
            for value in line {
                if value != Cell::Empty {
                    count += 1;
                }
            }
        }
        assert_eq!(count, 1);
    }

    #[test]
    fn make_random_move_does_not_make_move_when_game_not_started() {
        let mut g = Game::new(Mode::Simple, 10);
        g.make_random_move();

        assert_eq!(g.board, vec![vec![Cell::Empty; 10]; 10]);
    }

    #[test]
    fn switches_turn_when_valid_move_made() {
        // Game starts on Turn::LEFT
        let mut g = Game::new(Mode::Simple, 10);
        g.state = State::Playing;
        g.make_move(Cell::S, 6, 4);
        assert_eq!(g.turn, Turn::Right);
    }

    #[test]
    fn do_not_make_move_when_coord_not_empty() {
        let mut g = Game::new(Mode::Simple, 10);
        g.state = State::Playing;
        g.make_move(Cell::S, 6, 4);
        g.make_move(Cell::O, 6, 4);
        assert_eq!(g.board[6][4], Cell::S);
    }

    #[test]
    fn does_not_switch_turn_when_coord_not_empty() {
        // Game starts on Turn::LEFT
        let mut g = Game::new(Mode::Simple, 10);
        g.state = State::Playing;
        g.make_move(Cell::S, 6, 4);
        g.make_move(Cell::O, 6, 4);
        assert_eq!(g.turn, Turn::Right);
    }

    #[test]
    fn do_not_make_move_when_coord_invalid() {
        let mut g = Game::new(Mode::Simple, 5);
        g.state = State::Playing;
        g.make_move(Cell::S, 6, 4);
        assert_eq!(g.board, vec![vec![Cell::Empty; 5]; 5]);
    }

    #[test]
    fn does_not_switch_turn_when_invalid_move_made() {
        // Game starts on Turn::LEFT
        let mut g = Game::new(Mode::Simple, 10);
        g.state = State::Playing;
        g.make_move(Cell::S, 6, 10);
        assert_eq!(g.turn, Turn::Left);
    }

    #[test]
    fn clear_grid_does_not_change_size() {
        let mut g = Game::new(Mode::Simple, 10);
        g.state = State::Playing;
        g.make_move(Cell::S, 6, 4);
        g.make_move(Cell::O, 5, 5);
        g.clear_grid();
        assert_eq!(g.board, vec![vec![Cell::Empty; 10]; 10]);
    }

    #[test]
    fn get_cell_out_of_bounds_creates_error() {
        let mut g = Game::new(Mode::Simple, 10);
        let result = g.get_cell(9, 10);
        assert_eq!(result, Err(Error));
    }

    #[test]
    fn get_cell_in_bounds_returns_correct_value() {
        let mut g = Game::new(Mode::Simple, 10);
        g.state = State::Playing;
        g.make_move(Cell::S, 4, 5);
        let result = g.get_cell(5, 4);
        assert_eq!(result, Ok(&Cell::S));
    }

    #[test]
    fn left_player_wins_simple_game() {
        let mut g = Game::new(Mode::Simple, 10);
        g.state = State::Playing;
        g.make_move(Cell::O, 2, 1); // Left
        g.make_move(Cell::S, 1, 0); // Right

        g.make_move(Cell::O, 1, 1); // Left
        g.make_move(Cell::S, 2, 2); // Right

        g.make_move(Cell::S, 3, 2); // Left

        assert_eq!(g.state, State::LeftWin);
    }

    #[test]
    fn right_player_wins_simple_game() {
        let mut g = Game::new(Mode::Simple, 10);
        g.state = State::Playing;
        g.make_move(Cell::O, 2, 1); // Left
        g.make_move(Cell::S, 1, 0); // Right

        g.make_move(Cell::O, 1, 1); // Left
        g.make_move(Cell::S, 3, 2); // Right

        assert_eq!(g.state, State::RightWin);
    }

    #[test]
    fn players_draw_simple_game() {
        let mut g = Game::new(Mode::Simple, 3);
        g.state = State::Playing;

        g.make_move(Cell::S, 0, 0);
        g.make_move(Cell::S, 1, 0);
        g.make_move(Cell::S, 2, 0);
        g.make_move(Cell::S, 0, 1);
        g.make_move(Cell::S, 1, 1);
        g.make_move(Cell::S, 2, 1);
        g.make_move(Cell::S, 0, 2);
        g.make_move(Cell::S, 1, 2);
        g.make_move(Cell::S, 2, 2);

        assert_eq!(g.state, State::Draw);
    }

    #[test]
    fn left_player_wins_classic_game() {
        let mut g = Game::new(Mode::Classic, 3);
        g.state = State::Playing;
        g.make_move(Cell::S, 0, 0); // Left
        g.make_move(Cell::O, 1, 0); // Right

        g.make_move(Cell::S, 2, 0); // Left
        g.make_move(Cell::O, 0, 1); // Right

        g.make_move(Cell::S, 1, 1); // Left
        g.make_move(Cell::S, 2, 1); // Right

        g.make_move(Cell::O, 0, 2); // Left
        g.make_move(Cell::S, 1, 2); // Right

        g.make_move(Cell::S, 2, 2); // Left

        assert_eq!(g.state, State::LeftWin);
    }

    #[test]
    fn right_player_wins_classic_game() {
        let mut g = Game::new(Mode::Classic, 3);
        g.state = State::Playing;
        g.make_move(Cell::S, 2, 2); // Left
        g.make_move(Cell::O, 1, 0); // Right

        g.make_move(Cell::S, 2, 0); // Left
        g.make_move(Cell::S, 2, 1); // Right

        g.make_move(Cell::S, 1, 1); // Left
        g.make_move(Cell::O, 0, 1); // Right

        g.make_move(Cell::O, 0, 2); // Left
        g.make_move(Cell::S, 0, 0); // Right

        g.make_move(Cell::S, 1, 2); // Left

        assert_eq!(g.state, State::RightWin);
    }

    #[test]
    fn players_draw_classic_game() {
        let mut g = Game::new(Mode::Classic, 3);
        g.state = State::Playing;

        g.make_move(Cell::S, 0, 0);
        g.make_move(Cell::S, 1, 0);

        g.make_move(Cell::S, 2, 0);
        g.make_move(Cell::O, 0, 1);

        g.make_move(Cell::O, 1, 1);
        g.make_move(Cell::S, 2, 1);

        g.make_move(Cell::O, 1, 2);
        g.make_move(Cell::S, 0, 2);

        g.make_move(Cell::S, 2, 2);

        assert_eq!(g.state, State::Draw);
    }
}