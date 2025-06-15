//! # Interactive Number Guessing Game
//!
//! This module implements a graphical number guessing game where users select
//! a difficulty level and then attempt to guess a randomly generated number.
//!
//! ## Features
//!
//! - **Multiple Difficulty Levels**: Choose between Easy (1-10), Medium (1-100), or Hard (1-1000)
//! - **Visual Feedback**: Numbers change appearance based on guess result (too high/too low/correct)
//! - **Game State Management**: Automatically generates random targets and tracks user guesses
//! - **Interactive Grid Layout**: Numbers are displayed in a scrollable grid with 10 columns
//! - **Win Detection**: Shows a congratulatory popup when the correct number is guessed
//! - **Replayability**: Allows resetting the game to try again with a new target number
use eframe::egui::{self, ahash::HashMap};
use rand::Rng;
use std::fmt::Display;

#[derive(Debug, PartialEq, Default)]
enum Difficulty {
    #[default]
    Easy,
    Medium,
    Hard,
}

#[derive(Debug, Default)]
enum GuessResult {
    #[default]
    TooLow,
    TooHigh,
    Correct,
}

impl Display for GuessResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GuessResult::TooLow => write!(f, "Too Low"),
            GuessResult::TooHigh => write!(f, "Too High"),
            GuessResult::Correct => write!(f, "Correct"),
        }
    }
}

#[derive(Debug, Default)]
struct GuessingGame {
    difficulty: Option<Difficulty>,
    target: u32,
    guesses: HashMap<u32, GuessResult>,
}

impl GuessingGame {
    fn reset(&mut self) {
        self.difficulty = None;
        self.target = 0;
        self.guesses.clear();
    }

    fn is_game_over(&self) -> bool {
        self.guesses.contains_key(&self.target)
    }

    fn get_difficulty_range(&self) -> std::ops::RangeInclusive<u32> {
        match self.difficulty {
            Some(Difficulty::Easy) => 1..=10,
            Some(Difficulty::Medium) => 1..=100,
            Some(Difficulty::Hard) => 1..=1000,
            None => 1..=1, // Default range if no difficulty is selected
        }
    }

    fn evaluate_guess(&self, guess: u32) -> GuessResult {
        match guess.cmp(&self.target) {
            std::cmp::Ordering::Less => GuessResult::TooLow,
            std::cmp::Ordering::Greater => GuessResult::TooHigh,
            std::cmp::Ordering::Equal => GuessResult::Correct,
        }
    }
}

impl eframe::App for GuessingGame {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(_difficulty) = &self.difficulty {
                // Construct a range based on the selected difficulty
                let range = self.get_difficulty_range();

                egui::ScrollArea::both()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        egui::Grid::new("guessing_game_table")
                            .spacing([10.0, 10.0])
                            .striped(true)
                            .min_col_width(30.0)
                            .show(ui, |ui| {
                                let mut current_col = 0;
                                for number in range {
                                    // Start a new row after every 10 columns
                                    if current_col == 10 {
                                        ui.end_row();
                                        current_col = 0;
                                    }

                                    let display_number = if self.guesses.contains_key(&number) {
                                        format!("{} ({:?})", number, self.guesses[&number])
                                    } else {
                                        number.to_string()
                                    };
                                    let response = ui.selectable_label(false, display_number);
                                    if !self.is_game_over() && response.clicked() {
                                        let result = self.evaluate_guess(number);
                                        match result {
                                            GuessResult::Correct => {
                                                self.guesses.insert(number, GuessResult::Correct);
                                            }
                                            GuessResult::TooLow => {
                                                self.guesses.insert(number, GuessResult::TooLow);
                                            }
                                            GuessResult::TooHigh => {
                                                self.guesses.insert(number, GuessResult::TooHigh);
                                            }
                                        }
                                    } else {
                                        ui.label("");
                                    }

                                    current_col += 1;
                                }
                                // End the last row if needed
                                if current_col > 0 {
                                    ui.end_row();
                                }
                            });
                        ui.with_layout(
                            egui::Layout::top_down_justified(egui::Align::Center),
                            |ui| {
                                if ui.button("Reset Game").clicked() {
                                    self.reset();
                                }
                            },
                        );
                    });
            } else {
                egui::ComboBox::from_label("Difficulty")
                    .selected_text(self.difficulty.as_ref().map_or(
                        "Select Difficulty".to_string(),
                        |d| match d {
                            Difficulty::Easy => "Easy".to_string(),
                            Difficulty::Medium => "Medium".to_string(),
                            Difficulty::Hard => "Hard".to_string(),
                        },
                    ))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.difficulty, Some(Difficulty::Easy), "Easy");
                        ui.selectable_value(
                            &mut self.difficulty,
                            Some(Difficulty::Medium),
                            "Medium",
                        );
                        ui.selectable_value(&mut self.difficulty, Some(Difficulty::Hard), "Hard");
                    });

                // Randomly select a target number within the range
                let mut rng = rand::rng();
                self.target = rng.random_range(self.get_difficulty_range());
            }
        });
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([400.0, 400.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Guessing Game",
        options,
        Box::new(|_| Ok(Box::<GuessingGame>::default())),
    )
}
