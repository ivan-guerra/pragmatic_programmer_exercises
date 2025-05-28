//! # Character Counter Application
//!
//! This module implements a simple GUI application that counts characters in text input.
//! Built with egui/eframe, it provides a real-time character count as the user types.
//!
//! ## Features
//!
//! - **Real-time Updates**: Character count updates instantly as text is entered
//! - **Simple Interface**: Clean, focused UI for text input and character counting
//! - **Unicode Support**: Properly counts all Unicode characters, not just ASCII
use eframe::egui;

#[derive(Default)]
struct CharCounterApp {
    input: String,
}

impl eframe::App for CharCounterApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Enter input string:");
            ui.text_edit_singleline(&mut self.input);
            let char_count = self.input.chars().count();
            ui.label(format!("Character Count: {}", char_count));
        });
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 90.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Character Counter",
        options,
        Box::new(|_| Ok(Box::<CharCounterApp>::default())),
    )
}
