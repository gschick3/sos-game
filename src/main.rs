mod game;

use crate::game::{Mode, Game, Cell, Turn};
use eframe::egui;

const WIDTH: f32 = 600.0;
const HEIGHT: f32 = 500.0;
const MIN_BUTTON_SIZE: f32 = 30.0;

fn main() -> Result<(), eframe::Error> {
    let _g = Game::new(10, Mode::CLASSIC);
    // Configure eframe window
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(WIDTH, HEIGHT)),
        resizable: false,
        ..Default::default() // set everything else to default
    };
    eframe::run_native(
        "SOS",
        options,
        Box::new(|_cc| Box::<SosGame>::default()),
    )
}

struct SosGame {
    /// Decides board size next time player clicks Start or Reset
    next_board_size: usize,
    mode: Mode,
    p1move: Cell,
    p2move: Cell,
    /// Player view of the board (mirrors logical view from Game object)
    board: Vec<Vec<char>>,
    /// Game logic object
    game: Game,
    /// True if the player has clicked Start
    playing: bool
}

impl SosGame {
    fn reset_board(&mut self) {
        self.board = vec![vec![' '; self.next_board_size]; self.next_board_size];
    }
}

impl Default for SosGame {
    fn default() -> Self {
        Self {
            next_board_size: 5,
            mode: Mode::CLASSIC,
            p1move: Cell::S,
            p2move: Cell::S,
            board: vec![vec![' '; 5]; 5],
            game: Game::new(5, Mode::CLASSIC),
            playing: false
        }
    }
}

impl eframe::App for SosGame {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if !self.playing {
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
                        egui::ComboBox::from_id_source("mode")
                            .selected_text(match self.mode {
                                Mode::CLASSIC => "Classic",
                                Mode::SIMPLE => "Simple"
                            }).show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.mode, Mode::CLASSIC, "Classic");
                            ui.selectable_value(&mut self.mode, Mode::SIMPLE, "Simple");
                        });
                    });
                });
            });
        }

        egui::SidePanel::left("left")
            .resizable(false)
            .show_separator_line(false)
            .exact_width(WIDTH / 8.0)
            .show(ctx, |ui| {
            ui.label("Player 1");
            ui.radio_value(&mut self.p1move, Cell::S, "S");
            ui.radio_value(&mut self.p1move, Cell::O, "O");
        });

        egui::SidePanel::right("right")
            .resizable(false)
            .show_separator_line(false)
            .exact_width(WIDTH / 8.0)
            .show(ctx, |ui| {
            ui.label("Player 2");
            ui.radio_value(&mut self.p2move, Cell::S, "S");
            ui.radio_value(&mut self.p2move, Cell::O, "O");
        });

        egui::TopBottomPanel::bottom("bottom").show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                ui.label(format!("Turn: {}", match self.game.turn {
                    Turn::LEFT => "Player 1",
                    Turn::RIGHT => "Player 2"
                }));
                if !self.playing {
                    if ui.button("Start").clicked() {
                        self.reset_board();
                        self.game = Game::new(self.next_board_size.clone(), self.mode.clone());
                        self.playing = true;
                    }
                }
                else {
                    if ui.button("Reset").clicked() {
                        self.reset_board();
                        self.playing = false;
                    }
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            for i in 0..self.board.len() {
                ui.horizontal(|ui| {
                    for j in 0..self.board.len() {
                        match &self.board[i][j] {
                            ' ' => {
                                // Make sure the game is playing before letting players make moves
                                if ui.add(
                                    egui::Button::new("")
                                        .min_size(egui::vec2(MIN_BUTTON_SIZE, MIN_BUTTON_SIZE))
                                ).clicked() && self.playing {
                                    // The minimum size above is used so the buttons don't scaled differently between letters
                                    let pmove = match self.game.turn {
                                        Turn::LEFT => &self.p1move,
                                        Turn::RIGHT => &self.p2move
                                    };
                                    self.game.make_move(j, i, pmove.clone());
                                    self.board[i][j] = match pmove {
                                        Cell::S => 'S',
                                        Cell::O => 'O',
                                        _ => ' '
                                    }
                                }
                            }
                            other => {
                                // The minimum size is used so the buttons don't scaled differently between letters
                                ui.add(egui::Button::new(format!("{}", other))
                                    .min_size(egui::vec2(MIN_BUTTON_SIZE, MIN_BUTTON_SIZE)));
                            }
                        }
                    }
                });
            }
        });
    }
}
