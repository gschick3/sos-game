use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use crate::game_enums::{Cell, Mode};

#[derive(Clone, Debug, PartialEq)]
pub struct Move {
    pub cell: Cell,
    pub row: usize,
    pub col: usize
}

#[derive(Debug, PartialEq)]
pub struct Recording {
    pub mode: Mode,
    pub board_size: usize,
    pub moves: Vec<Move>,
    current_move: usize,
}

impl Recording {
    pub fn new(mode: Mode, board_size: usize) -> Self {
        Self {
            mode,
            board_size,
            moves: Vec::new(),
            current_move: 0
        }
    }
    pub fn add_move(&mut self, cell: Cell, row: usize, col: usize) {
        self.moves.push(Move { cell, row, col});
    }
    pub fn next_move(&mut self) -> Option<&Move> {
        if self.moves.len() > self.current_move {
            self.current_move += 1;
            return Some(&self.moves[self.current_move - 1]);
        }
        None
    }
    pub fn reset(&mut self) {
        self.current_move = 0;
    }
    pub fn as_string(&self) -> String {
        let mut string = match self.mode {
            Mode::Classic => "C",
            Mode::Simple => "S"
        }.to_string() + "," + &*self.board_size.to_string();

        for m in self.moves.clone() {
            string += "\n";
            string += match m.cell {
                Cell::S => "S",
                Cell::O => "O",
                Cell::Empty => ""
            };
            string += &*(",".to_owned() + &*m.row.to_string() + "," + &*m.col.to_string());
        }
        return string;
    }
    pub fn write_to_file(&self, file_name: String) {
        let mut f = BufWriter::new(File::create(file_name).unwrap());
        f.write_all(self.as_string().as_bytes()).unwrap()
    }
    pub fn read_from_file(file_name: String) -> Option<Self> {
        let mut first_line = String::new();
        let f = File::open(file_name);
        match f {
            Err(..) => return None,
            _ => ()
        }

        let mut br = BufReader::new(f.unwrap());
        match br.read_line(&mut first_line) {
            Err(..) => return None,
            _ => ()
        };

        // read_line keeps trailing \r\n
        // we need to remove that to parse the usize
        if first_line.ends_with('\n') {
            first_line.pop();
            if first_line.ends_with('\r') {
                first_line.pop();
            }
        }

        let first_line_vec:Vec<&str> = first_line.split(",").collect();

        let board_size = first_line_vec[1].parse::<usize>();
        match board_size {
            Err(..) => return None,
            _ => ()
        };

        let mut new_record = Self::new(
            match first_line_vec[0] {
                "C" => Mode::Classic,
                "S" => Mode::Simple,
                _ => Mode::Classic
            },
            board_size.unwrap(),
        );

        for line in br.lines() {
            let line_str = line;
            match line_str {
                Err(..) => return None,
                _ => ()
            };
            // This avoids a "temporary value dropped while borrowed" error
            let line_str = line_str.unwrap();

            let line_vec:Vec<&str> = line_str.split(",").collect();
            let row = line_vec[1].parse::<usize>();
            match row {
                Err(..) => return None,
                _ => ()
            };

            let col = line_vec[2].parse::<usize>();
            match col {
                Err(..) => return None,
                _ => ()
            };

            new_record.add_move(
                match line_vec[0] {
                    "S" => Cell::S,
                    "O" => Cell::O,
                    _ => Cell::Empty
                },
                row.unwrap(),
                col.unwrap()
            );
        }
        return Some(new_record);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add_move_increases_vector_size() {
        let mut recording = Recording::new(Mode::Simple, 5);
        recording.add_move(Cell::S, 1, 1);
        assert_eq!(recording.moves.len(), 1);
    }

    #[test]
    fn next_move_reads_first_move() {
        let mut recording = Recording::new(Mode::Simple, 5);
        recording.add_move(Cell::S, 1, 2);
        let m = recording.next_move().unwrap();
        assert!(m.row == 1 && m.col == 2);
    }

    #[test]
    fn next_move_reads_second_move() {
        let mut recording = Recording::new(Mode::Simple, 5);
        recording.add_move(Cell::S, 1, 2);
        recording.add_move(Cell::S, 3, 4);
        recording.next_move();
        let m = recording.next_move().unwrap();
        assert!(m.row == 3 && m.col == 4);
    }

    #[test]
    fn next_move_returns_none_if_empty() {
        let mut recording = Recording::new(Mode::Simple, 5);
        let m = recording.next_move();
        assert_eq!(m, None);
    }

    #[test]
    fn next_move_returns_none_if_end_reached() {
        let mut recording = Recording::new(Mode::Simple, 5);
        recording.add_move(Cell::S, 1, 2);
        recording.next_move();
        let m = recording.next_move();
        assert_eq!(m, None);
    }

    #[test]
    fn read_file_returns_none_if_not_found() {
        let recording = Recording::read_from_file(String::from("this_file_does_not_exist"));
        assert_eq!(recording, None);
    }
}