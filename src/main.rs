mod game;
mod player;
mod game_enums;
mod recording;

use std::process;
use std::thread;
use std::time::Duration;
use crate::game::Game;
use crate::recording::Recording;
use crate::game_enums::{Mode, Cell, Turn, State};
use eframe::egui;
use eframe::egui::{FontFamily, FontId, TextStyle};
use crate::player::Player;

const SIDE_PANEL_WIDTH: f32 = 80.0;
const BOARD_SIZE: f32 = 600.0;
// WIDTH = board length + side panels + 45 pixels padding
const WIDTH: f32 = BOARD_SIZE + 2.0 * SIDE_PANEL_WIDTH + 45.0;
// HEIGHT = board length + top and bottom panels
const HEIGHT: f32 = BOARD_SIZE + 75.0;

fn main() -> Result<(), eframe::Error> {
    // Configure eframe window
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(WIDTH, HEIGHT)),
        resizable: false,
        ..Default::default() // set everything else to default
    };
    eframe::run_native(
        "SOS",
        options,
        Box::new(|_cc| Box::<GameInterface>::default()),
    )
}

struct GameInterface {
    /// Decides board size next time player clicks Start or Reset
    next_board_size: usize,
    mode: Mode,
    player1: Player,
    player2: Player,
    game: Game,
    recording: Option<Recording>
}

impl Default for GameInterface {
    fn default() -> Self {
        Self {
            next_board_size: 5,
            mode: Mode::Classic,
            player1: Player::new(Cell::S, false),
            player2: Player::new(Cell::S, false),
            game: Game::new(Mode::Classic, 5),
            recording: None
        }
    }
}

impl eframe::App for GameInterface {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Top panel contains board size and game mode select
        egui::TopBottomPanel::top("top")
            .resizable(false)
            .show_separator_line(true)
            .show(ctx, |ui| {
            ui.horizontal_top(|ui| {
                ui.vertical(|ui| {
                    ui.label("Board Size");
                    ui.add(egui::Slider::new(&mut self.next_board_size, 3..=10));
                });
                ui.vertical(|ui| {
                    ui.label("Mode");
                    if self.game.state != State::Playing {
                        egui::ComboBox::from_id_source("mode")
                            .selected_text(match self.mode {
                                Mode::Classic => "Classic",
                                Mode::Simple => "Simple"
                            }).show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.mode, Mode::Classic, "Classic");
                            ui.selectable_value(&mut self.mode, Mode::Simple, "Simple");
                        });
                    } else {
                        let _ = ui.button(match self.mode {
                            Mode::Classic => "Classic",
                            Mode::Simple => "Simple"
                        });
                    }
                });
                ui.vertical(|ui| {
                    ui.label("");
                    if self.game.state == State::NotStarted {
                        if ui.button("Start").clicked() {
                            self.game = Game::new(self.mode.clone(), self.next_board_size.clone());
                            self.game.state = State::Playing;
                        }
                    } else {
                        if ui.button("Reset").clicked() {
                            self.game.clear_grid();
                            self.recording = None;
                            self.game.state = State::NotStarted;
                        }
                    }
                });
                ui.vertical(|ui| {
                    ui.label("");
                    if self.game.state == State::NotStarted {
                        if ui.button("Load").clicked() {
                            let open_file: String;
                            match tinyfiledialogs::open_file_dialog("Open", "", Some((&["*.sos"], ".sos"))) {
                                Some(file) => open_file = file,
                                None => open_file = "null".to_string(),
                            }
                            let recording = Recording::read_from_file(open_file).unwrap_or_else(|| {
                                eprintln!("Error opening file.");
                                process::exit(1);
                            });
                            self.next_board_size = recording.board_size;
                            self.mode = recording.mode.clone();
                            self.recording = Some(recording);
                            self.player1.computer = true;
                            self.player2.computer = true;

                            self.game = Game::new(self.mode.clone(), self.next_board_size);
                            self.game.state = State::Playing;
                        }
                    }
                    else if self.game.state != State::Playing {
                        if ui.button("Save").clicked() {
                            let save_file: String;
                            match tinyfiledialogs::save_file_dialog("Save", "recording.sos") {
                                Some(file) => save_file = file,
                                None => save_file = "null".to_string(),
                            }
                            self.game.recording.write_to_file(save_file);
                        }
                    }
                });
            });
        });

        // Left panel contains Player 1's controls
        egui::SidePanel::left("left")
            .resizable(false)
            .exact_width(SIDE_PANEL_WIDTH)
            .show(ctx, |ui| {
                ui.label("Player 1");
                if self.game.state != State::Playing {
                    ui.checkbox(&mut self.player1.computer, "Computer");
                } else {
                    ui.label(
                        match self.player1.computer {
                            true => "Computer",
                            false => "Human"
                        }
                    );
                }
                if !self.player1.computer {
                    ui.radio_value(&mut self.player1.pmove, Cell::S, "S");
                    ui.radio_value(&mut self.player1.pmove, Cell::O, "O");
                }
                ui.label(format!("Score: {}", self.game.left_score));
        });

        // Right panel contains Player 2's controls
        egui::SidePanel::right("right")
            .resizable(false)
            .exact_width(SIDE_PANEL_WIDTH + 20.0) // I have no idea why, but this has to be larger than the left panel
            .show(ctx, |ui| {
                ui.label("Player 2");
                if self.game.state != State::Playing {
                    ui.checkbox(&mut self.player2.computer, "Computer");
                } else {
                    ui.label(
                        match self.player2.computer {
                            true => "Computer",
                            false => "Human"
                        }
                    );
                }
                if !self.player2.computer {
                    ui.radio_value(&mut self.player2.pmove, Cell::S, "S");
                    ui.radio_value(&mut self.player2.pmove, Cell::O, "O");
                }
                ui.label(format!("Score: {}", self.game.right_score));
        });

        // Bottom panel contains turn information and start/reset buttons
        egui::TopBottomPanel::bottom("bottom").show_separator_line(false).show(ctx, |ui| {
            ui.horizontal(|ui| {
                if self.game.state == State::Playing {
                    ui.label(format!("Turn: {}", match self.game.turn {
                        Turn::Left => "Player 1",
                        Turn::Right => "Player 2"
                    }));
                }
                else {
                    ui.label(match self.game.state {
                        State::LeftWin => "Player 1 Wins!",
                        State::RightWin => "Player 2 Wins!",
                        State::Draw => "Tie Game",
                        _ => ""
                    });
                }
            });
        });

        let current_turn = match self.game.turn {
            Turn::Left => &self.player1,
            Turn::Right => &self.player2
        };

        if current_turn.computer && self.game.state == State::Playing {
            match &mut self.recording {
                None => self.game.make_random_move(),
                Some(recording) => {
                    let next_move = recording.next_move();
                    match next_move {
                        // Somehow the recording ended before the game was finished
                        None => self.game.state = State::Draw,
                        Some(m) => {
                            self.game.make_move(m.cell, m.row, m.col);
                            thread::sleep(Duration::from_millis(1500));
                        }
                    }
                }
            }
            ctx.request_repaint(); // otherwise, requires mouse movement
        }

        // Central panel contains game board
        egui::CentralPanel::default().show(ctx, |ui| {
            // button_size = measured board size / unit board size - button padding
            let button_size = BOARD_SIZE / self.game.get_board_size() as f32 - 8.0;
            let style = ui.style_mut();
            style.text_styles.insert(TextStyle::Button, FontId::new(button_size * 0.75, FontFamily::Proportional));

            for y in 0..self.game.get_board_size() {
                ui.horizontal(|ui| {
                    for x in 0..self.game.get_board_size() {
                        if ui.add(egui::Button::new(match self.game.get_cell(x, y).unwrap() {
                            Cell::Empty => "",
                            Cell::O => "O",
                            Cell::S => "S"
                            // The minimum size below is used so the buttons don't scale differently between letters
                        }).min_size(egui::vec2(button_size, button_size))).clicked()
                            && self.game.state == State::Playing
                            && !current_turn.computer {
                            self.game.make_move(current_turn.pmove.clone(), y, x);
                        }
                    }
                });
            }
        });
    }
}
