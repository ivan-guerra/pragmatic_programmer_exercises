//! # Magic 8 Ball Simulator
//!
//! This module implements a graphical Magic 8 Ball simulator where users can ask
//! questions and receive random answers, simulating the classic fortune-telling toy.
//!
//! ## Features
//!
//! - **Question Input**: Type your question in a text field
//! - **Random Responses**: Get one of several possible answers when you "shake" the ball
//! - **Simple Interface**: Clean, intuitive UI for asking questions and viewing responses
//!
//! The simulator provides a virtual Magic 8 Ball experience with a set of predefined
//! responses that are randomly selected when the user submits a question.
use eframe::egui::{self};
use rand::seq::IndexedRandom;

#[derive(Debug)]
struct Magic8Ball {
    question: String,
    response: Option<&'static str>,
    responses: [&'static str; 4],
}

impl Default for Magic8Ball {
    fn default() -> Self {
        Magic8Ball {
            question: String::new(),
            response: None,
            responses: ["Yes", "No", "Ask again later", "Definitely not"],
        }
    }
}

impl Magic8Ball {
    fn set_rand_response(&mut self) {
        let mut rng = rand::rng();
        self.response = Some(
            self.responses
                .choose(&mut rng)
                .unwrap_or(&self.responses[0]),
        )
    }
}

impl eframe::App for Magic8Ball {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Ask a question and shake the Magic 8 Ball!");
            ui.horizontal(|ui| {
                ui.label("Your Question:");
                ui.text_edit_singleline(&mut self.question);
            });

            if !self.question.is_empty() && ui.button("Shake").clicked() {
                self.set_rand_response();
            }
            if let Some(response) = &self.response {
                ui.label(format!("Magic 8 Ball says: {}", response));
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
        "Magic 8 Ball",
        options,
        Box::new(|_| Ok(Box::<Magic8Ball>::default())),
    )
}
