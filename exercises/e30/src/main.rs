//! # Interactive Multiplication Table
//!
//! This module implements a graphical application that displays a customizable
//! multiplication table based on a user-selected base number.
//!
//! ## Features
//!
//! - **Interactive Base Selection**: Users can choose the table size from a dropdown menu
//! - **Dynamic Table Generation**: Table content updates instantly when base changes
//! - **Scrollable Interface**: Handles large tables with horizontal and vertical scrolling
//! - **Visual Formatting**: Uses grid layout with proper headers and striped rows
//! - **Educational Tool**: Provides clear visual representation of multiplication patterns
//! - **Responsive Design**: Adapts to window size and maintains usability for larger tables
use eframe::egui::{self};

#[derive(Debug)]
struct MultiplicationTableApp {
    base: u32,
}

impl Default for MultiplicationTableApp {
    fn default() -> Self {
        Self { base: 1 }
    }
}

impl eframe::App for MultiplicationTableApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Create a drop-down menu for selecting the base number
            ui.label("Select a base number for the multiplication table:");
            egui::ComboBox::from_label("Numbers")
                .selected_text(self.base.to_string())
                .show_ui(ui, |ui| {
                    for number in 1..=12 {
                        ui.selectable_value(&mut self.base, number, number.to_string());
                    }
                });

            ui.separator();

            egui::ScrollArea::both()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    egui::Grid::new("multiplication_table")
                        .spacing([10.0, 10.0])
                        .striped(true)
                        .min_col_width(30.0)
                        .show(ui, |ui| {
                            // Header row with column numbers
                            ui.label("×"); // Top-left corner indicator
                            for i in 1..=self.base {
                                ui.strong(i.to_string());
                            }
                            ui.end_row();

                            // Table body with row numbers and calculations
                            for i in 1..=self.base {
                                ui.strong(i.to_string()); // Row header
                                for j in 1..=self.base {
                                    ui.label((i * j).to_string());
                                }
                                ui.end_row();
                            }
                        });
                });
        });
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([400.0, 250.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Multiplication Table",
        options,
        Box::new(|_| Ok(Box::<MultiplicationTableApp>::default())),
    )
}
