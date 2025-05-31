//! # Simple Math Application
//!
//! This module implements a GUI calculator application for basic arithmetic operations.
//! Built with egui/eframe, it allows users to perform addition, subtraction, multiplication,
//! and division on two input values.
//!
//! ## Features
//!
//! - **Real-time Calculation**: Results update instantly as values are entered
//! - **Multiple Operations**: Performs addition, subtraction, multiplication, and division
//! - **Input Validation**: Handles invalid inputs gracefully
//! - **Division by Zero Protection**: Special handling for division by zero cases
use eframe::egui;

#[derive(Default)]
struct SimpleMathApp {
    value1: String,
    value2: String,
}

impl eframe::App for SimpleMathApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Enter value 1:");
            ui.text_edit_singleline(&mut self.value1);
            ui.label("Enter value 2:");
            ui.text_edit_singleline(&mut self.value2);

            let value1: f64 = self.value1.parse().unwrap_or(f64::NAN);
            let value2: f64 = self.value2.parse().unwrap_or(f64::NAN);

            if value1.is_nan() || value2.is_nan() {
                ui.label("Please enter valid numbers.");
                return;
            }

            ui.label(format!(
                "{:.2} + {:.2} = {:.2}",
                value1,
                value2,
                value1 + value2
            ));
            ui.label(format!(
                "{:.2} - {:.2} = {:.2}",
                value1,
                value2,
                value1 - value2
            ));
            ui.label(format!(
                "{:.2} * {:.2} = {:.2}",
                value1,
                value2,
                value1 * value2
            ));
            if value2 != 0.0 {
                ui.label(format!(
                    "{:.2} / {:.2} = {:.2}",
                    value1,
                    value2,
                    value1 / value2
                ));
            } else {
                ui.label("Division by zero is undefined.");
            }
        });
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([400.0, 250.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Simple Math",
        options,
        Box::new(|_| Ok(Box::<SimpleMathApp>::default())),
    )
}
