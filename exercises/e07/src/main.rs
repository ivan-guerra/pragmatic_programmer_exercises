//! # Area Calculator
//!
//! This module implements an interactive area calculation application that
//! converts between square feet and square meters based on room dimensions.
//!
//! ## Features
//!
//! - **Unit Selection**: Allows users to choose between feet and meters for input
//! - **Real-time Calculation**: Results update instantly as dimensions are entered
//! - **Dual Unit Display**: Shows area in both square feet and square meters simultaneously
//! - **Input Validation**: Gracefully handles invalid dimension inputs
//! - **Conversion Logic**: Accurately converts between imperial and metric measurement systems
use eframe::egui::{self, ComboBox};
use std::fmt::Display;

const FT_TO_METER: f64 = 0.09290304; // 1 square foot to square meters

#[derive(Debug, Clone, Copy, PartialEq)]
enum AreaUnit {
    Meters,
    Feet,
}

struct AreaCalculator {
    selected_unit: AreaUnit,
    length: String,
    width: String,
}

impl AreaCalculator {
    fn calculate_area(&self) -> Option<(f64, f64)> {
        let length: f64 = self.length.parse().unwrap_or(f64::NAN);
        let width: f64 = self.width.parse().unwrap_or(f64::NAN);

        if length.is_nan() || width.is_nan() {
            return None;
        }

        let area = length * width;
        match self.selected_unit {
            AreaUnit::Meters => Some((area, area / FT_TO_METER)),
            AreaUnit::Feet => Some((area * FT_TO_METER, area)),
        }
    }
}

impl Default for AreaCalculator {
    fn default() -> Self {
        Self {
            selected_unit: AreaUnit::Meters,
            length: String::new(),
            width: String::new(),
        }
    }
}

impl Display for AreaUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AreaUnit::Meters => write!(f, "meters"),
            AreaUnit::Feet => write!(f, "feet"),
        }
    }
}

impl eframe::App for AreaCalculator {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ComboBox::from_label("Choose an option")
                .selected_text(self.selected_unit.to_string())
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.selected_unit, AreaUnit::Meters, "meters");
                    ui.selectable_value(&mut self.selected_unit, AreaUnit::Feet, "feet");
                });

            ui.label(format!(
                "What is the length of the room in {}?",
                self.selected_unit
            ));
            ui.text_edit_singleline(&mut self.length);

            ui.label(format!(
                "What is the width of the room in {}?",
                self.selected_unit
            ));
            ui.text_edit_singleline(&mut self.width);

            let area = self.calculate_area();
            if let Some((area_meters, area_feet)) = area {
                ui.label("The area is:");
                ui.label(format!("{:.2} square feet", area_feet));
                ui.label(format!("{:.2} square meters", area_meters));
            } else {
                ui.label("Please enter valid numbers for length and width.");
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
        Box::new(|_| Ok(Box::<AreaCalculator>::default())),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_area_converts_meters_to_feet_correctly() {
        let calculator = AreaCalculator {
            selected_unit: AreaUnit::Meters,
            length: String::from("5"),
            width: String::from("4"),
        };

        if let Some((area_meters, area_feet)) = calculator.calculate_area() {
            assert_eq!(area_meters, 20.0);
            assert!((area_feet - 215.28).abs() < 0.01); // Approximately 20.0 / 0.09290304
        } else {
            panic!("calculate_area returned None when it should have returned Some");
        }
    }

    #[test]
    fn calculate_area_converts_feet_to_meters_correctly() {
        let calculator = AreaCalculator {
            selected_unit: AreaUnit::Feet,
            length: String::from("10"),
            width: String::from("10"),
        };

        if let Some((area_meters, area_feet)) = calculator.calculate_area() {
            assert_eq!(area_feet, 100.0);
            assert!((area_meters - 9.29).abs() < 0.01); // Approximately 100.0 * 0.09290304
        } else {
            panic!("calculate_area returned None when it should have returned Some");
        }
    }

    #[test]
    fn calculate_area_handles_invalid_inputs() {
        let calculator_invalid_length = AreaCalculator {
            selected_unit: AreaUnit::Meters,
            length: String::from("invalid"),
            width: String::from("5"),
        };

        assert!(calculator_invalid_length.calculate_area().is_none());

        let calculator_invalid_width = AreaCalculator {
            selected_unit: AreaUnit::Feet,
            length: String::from("10"),
            width: String::from("abc"),
        };

        assert!(calculator_invalid_width.calculate_area().is_none());
    }

    #[test]
    fn calculate_area_handles_zero_dimensions() {
        let calculator = AreaCalculator {
            selected_unit: AreaUnit::Meters,
            length: String::from("0"),
            width: String::from("0"),
        };

        if let Some((area_meters, area_feet)) = calculator.calculate_area() {
            assert_eq!(area_meters, 0.0);
            assert_eq!(area_feet, 0.0);
        } else {
            panic!("calculate_area returned None when it should have returned Some");
        }
    }
}
