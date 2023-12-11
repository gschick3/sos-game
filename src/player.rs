use crate::game_enums::Cell;

#[derive(Clone)]
pub struct Player {
    pub pmove: Cell,
    pub computer: bool
}

impl Player {
    pub fn new(initial_move: Cell, is_computer: bool) -> Self {
        Self {
            pmove: initial_move,
            computer: is_computer
        }
    }
}