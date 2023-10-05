//!
//! General game board implementation
//!

use std::fmt::Error;

pub struct Board<T> {
    grid: Vec<Vec<T>>
}

impl<T: Clone> Board<T> {       // T must implement Clone
    pub fn new(width: usize, height: usize, fill: T) -> Self {
        Self {
            grid: vec![vec![fill; width]; height]
        }
    }

    /// Get value of cell at (x, y)
    ///
    /// Returns Err(Error) if (x, y) is invalid
    ///
    /// # Example
    /// ```
    /// use board::Board;
    ///
    /// let b = Board::new(5, 10, 'X');
    /// let value = b.get_value(3, 3).unwrap(); // Equals 'X'
    /// ```
    pub fn get_value(&self, x: usize, y: usize) -> Result<&T, Error> {
        match self.is_valid(x, y) {
            true => Ok(&self.grid[y][x]),
            false => Err(Error)
        }
    }

    /// Set value of cell at (x, y)
    ///
    /// Returns Err(Error) if (x, y) is invalid
    ///
    /// # Example
    /// ```
    /// use board::Board;
    ///
    /// let b = Board::new(5, 10, 'X');
    /// b.set_value(3, 3, 'M').unwrap();
    /// ```
    pub fn set_cell(&mut self, x: usize, y: usize, input: T) -> Result<(), Error> {
        match self.is_valid(x, y) {
            true => {
                self.grid[y][x] = input;
                Ok(())
            }
            false => Err(Error),
        }
    }

    /// Change size of board and fill with value
    ///
    /// # Example
    /// ```
    /// use board::Board;
    ///
    /// let b = Board::new(5, 10, 'X');
    /// b.set_value(3, 3, 'M').unwrap();
    ///
    /// b.resize_reset(4, 4, 'O');
    /// ```
    fn resize_reset(&mut self, width: usize, height: usize, fill: T) {
        self.grid = vec![vec![fill; width]; height];
    }

    /// Check if x and y are within bounds
    fn is_valid(&self, x: usize, y: usize) -> bool {
        // x and y are unsigned, so they will always be above the lower bounds
        x < self.grid[0].len() && y < self.grid.len()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_value_gives_error_when_coord_out_of_bounds() {
        let b = Board::new(5, 10, 0);
        assert_eq!(b.get_value(5, 10), Err(Error));
    }

    #[test]
    fn get_value_returns_value_when_coord_in_bounds() {
        let b = Board::new(5, 10, 0);
        assert_eq!(b.get_value(4, 8), Ok(&0));
    }

    #[test]
    fn set_cell_gives_error_when_coord_out_of_bounds() {
        let mut b = Board::new(5, 10, 0);
        assert_eq!(b.set_cell(5, 6, 1), Err(Error));
    }

    #[test]
    fn set_cell_returns_ok_when_coord_in_bounds() {
        let mut b = Board::new(5, 10, 0);
        assert_eq!(b.set_cell(4, 6, 1), Ok(()));
    }

    #[test]
    fn coord_not_valid_when_x_out_of_bounds() {
        let b = Board::new(5, 10, 0);
        assert_eq!(b.is_valid(5, 5), false);
    }

    #[test]
    fn coord_not_valid_when_y_out_of_bounds() {
        let b = Board::new(5, 10, 0);
        assert_eq!(b.is_valid(4, 11), false);
    }

    #[test]
    fn coord_valid_when_in_bounds() {
        let b = Board::new(5, 10, 0);
        assert_eq!(b.is_valid(4, 5), true);
    }

    #[test]
    fn resize_changes_board_size() {
        let mut b = Board::new(5, 10, 0);

        let new_x = 10;
        let new_y = 5;

        b.resize_reset(new_x, new_y, 0);
        assert!(b.grid[0].len() == new_x && b.grid.len() == new_y);
    }

    #[test]
    fn resize_resets_board_fill() {
        let mut b = Board::new(5, 10, 0);
        b.set_cell(4, 6, 1).unwrap();
        b.resize_reset(10, 5, 0);
        assert_eq!(b.grid, vec![vec![0; 10]; 5]);
    }
}