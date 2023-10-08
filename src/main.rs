mod game;

use crate::game::{Mode, Game, Cell, Turn};
use eframe::egui;
use eframe::egui::{FontFamily, FontId, TextStyle};

const SIDE_PANEL_WIDTH: f32 = 60.0;
const MAX_BOARD_SIZE: usize = 10;
const BUTTON_SIZE: f32 = 30.0;
// WIDTH = board-size number of buttons + 8 pixels between each button + side panels + 45 pixels padding
const WIDTH: f32 = MAX_BOARD_SIZE as f32 * (BUTTON_SIZE + 8.0) + 2.0 * SIDE_PANEL_WIDTH + 45.0;
// HEIGHT = board-size number of buttons + 2 pixels between each button + top and bottom panels
const HEIGHT: f32 = MAX_BOARD_SIZE as f32 * (BUTTON_SIZE + 2.0) + 100.0;

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
        Box::new(|_cc| Box::<SosGame>::default()),
    )
}

struct SosGame {
    /// Decides board size next time player clicks Start or Reset
    next_board_size: usize,
    mode: Mode,
    p1move: Cell,
    p2move: Cell,
    /// Game logic object
    game: Game,
    /// True if the player has clicked Start
    playing: bool
}

impl Default for SosGame {
    fn default() -> Self {
        Self {
            next_board_size: 5,
            mode: Mode::CLASSIC,
            p1move: Cell::S,
            p2move: Cell::S,
            game: Game::new(5, Mode::CLASSIC),
            playing: false
        }
    }
}

impl eframe::App for SosGame {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Top panel contains board size and game mode select
        egui::TopBottomPanel::top("top")
            .resizable(false)
            .show_separator_line(true)
            .show(ctx, |ui| {
            ui.horizontal_top(|ui| {
                ui.vertical(|ui| {
                    ui.label("Board Size");
                    ui.add(egui::Slider::new(&mut self.next_board_size, 3..=MAX_BOARD_SIZE));
                });
                ui.vertical(|ui| {
                    ui.label("Mode");
                    if !self.playing {
                        egui::ComboBox::from_id_source("mode")
                            .selected_text(match self.mode {
                                Mode::CLASSIC => "Classic",
                                Mode::SIMPLE => "Simple"
                            }).show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.mode, Mode::CLASSIC, "Classic");
                            ui.selectable_value(&mut self.mode, Mode::SIMPLE, "Simple");
                        });
                    } else {
                        let _ = ui.button(match self.mode {
                            Mode::CLASSIC => "Classic",
                            Mode::SIMPLE => "Simple"
                        });
                    }
                });
                ui.vertical(|ui| {
                    ui.label("");
                    if !self.playing {
                        if ui.button("Start").clicked() {
                            self.game.clear_grid();
                            self.game = Game::new(self.next_board_size.clone(), self.mode.clone());
                            self.playing = true;
                        }
                    } else {
                        if ui.button("Reset").clicked() {
                            self.game.clear_grid();
                            self.playing = false;
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
                ui.radio_value(&mut self.p1move, Cell::S, "S");
                ui.radio_value(&mut self.p1move, Cell::O, "O");
        });

        // Right panel contains Player 2's controls
        egui::SidePanel::right("right")
            .resizable(false)
            .exact_width(SIDE_PANEL_WIDTH + 20.0) // I have no idea why, but this has to be larger than the left panel
            .show(ctx, |ui| {
                ui.label("Player 2");
                ui.radio_value(&mut self.p2move, Cell::S, "S");
                ui.radio_value(&mut self.p2move, Cell::O, "O");
        });

        // Bottom panel contains turn information and start/reset buttons
        egui::TopBottomPanel::bottom("bottom").show_separator_line(false).show(ctx, |ui| {
            ui.label(format!("Turn: {}", match self.game.turn {
                Turn::LEFT => "Player 1",
                Turn::RIGHT => "Player 2"
            }));
        });

        // Central panel contains game board
        egui::CentralPanel::default().show(ctx, |ui| {
            let style = ui.style_mut();
            style.text_styles.insert(TextStyle::Button, FontId::new(BUTTON_SIZE * 0.75, FontFamily::Proportional));
            for y in 0..self.game.board_size {
                ui.horizontal(|ui| {
                    for x in 0..self.game.board_size {
                        if ui.add(egui::Button::new(match self.game.get_cell(x, y).unwrap() {
                            Cell::EMPTY => "",
                            Cell::O => "O",
                            Cell::S => "S"
                        }).min_size(egui::vec2(BUTTON_SIZE, BUTTON_SIZE))).clicked()
                        && self.playing {
                            // The minimum size above is used so the buttons don't scaled differently between letters
                            let pmove = match self.game.turn {
                                Turn::LEFT => &self.p1move,
                                Turn::RIGHT => &self.p2move
                            };
                            self.game.make_move(x, y, pmove.clone());
                        }
                    }
                });
            }
        });
    }
}
