mod game;

use crate::game::{Mode, Game, Cell, Turn};
use eframe::egui;

const WIDTH: f32 = 580.0;
const HEIGHT: f32 = 410.0;
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
            .exact_width(WIDTH / 6.5) // I have no idea why, but this has to be larger than the left panel
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

        egui::CentralPanel::default().show(ctx, |ui| {
            for y in 0..self.game.board_size {
                ui.horizontal(|ui| {
                    for x in 0..self.game.board_size {
                        if ui.add(egui::Button::new(match self.game.get_cell(x, y).unwrap() {
                            Cell::EMPTY => "",
                            Cell::O => "O",
                            Cell::S => "S"
                        }).min_size(egui::vec2(MIN_BUTTON_SIZE, MIN_BUTTON_SIZE))).clicked()
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
