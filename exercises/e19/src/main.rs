//! # BMI Calculator
//!
//! This module implements an interactive Body Mass Index (BMI) calculator application that
//! computes and categorizes BMI based on user's weight and height measurements.
//!
//! ## Features
//!
//! - **Interactive Interface**: GUI for entering weight and height measurements
//! - **Real-time Calculation**: BMI updates automatically as input values change
//! - **Health Classification**: Categorizes BMI into underweight, healthy, or overweight
//! - **Visual Feedback**: Color-coded results to indicate different BMI categories
//! - **Standard Formula**: Uses the standard BMI formula with US measurements (lbs/inches)
//! - **Zero-Value Protection**: Prevents division by zero when height is not provided
use eframe::egui::{self};

#[derive(Debug, Default)]
struct BMICalculator {
    weight_lbs: f64,
    height_in: f64,
}

impl BMICalculator {
    fn calculate_bmi(&self) -> f64 {
        if self.height_in == 0.0 {
            return 0.0; // Avoid division by zero
        }
        (self.weight_lbs / (self.height_in * self.height_in)) * 703.0
    }
}

impl eframe::App for BMICalculator {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Weight (lb):");
            ui.add(egui::DragValue::new(&mut self.weight_lbs).speed(0.5));
            ui.label("Height (in):");
            ui.add(egui::DragValue::new(&mut self.height_in).speed(0.5));

            const HEALTHY_BMI: (f64, f64) = (18.0, 25.0);
            let bmi = self.calculate_bmi();
            match bmi {
                bmi if bmi < HEALTHY_BMI.0 => {
                    ui.colored_label(
                        egui::Color32::from_rgb(135, 206, 250), // Light blue color
                        format!("Your BMI is {:.2}. You are underweight.", bmi),
                    );
                }
                bmi if bmi > HEALTHY_BMI.1 => {
                    ui.colored_label(
                        egui::Color32::from_rgb(255, 165, 0), // Orange color
                        format!("Your BMI is {:.2}. You are overweight.", bmi),
                    );
                }
                bmi => {
                    ui.colored_label(
                        egui::Color32::from_rgb(50, 205, 50), // Green color
                        format!("Your BMI is {:.2}. You are healthy.", bmi),
                    );
                }
            }
        });
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([400.0, 150.0]),
        ..Default::default()
    };
    eframe::run_native(
        "BMI Calculator",
        options,
        Box::new(|_| Ok(Box::<BMICalculator>::default())),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_bmi_calculates_correctly() {
        // Test with common values
        let calculator = BMICalculator {
            weight_lbs: 150.0,
            height_in: 70.0,
        };
        // BMI = (150 / (70 * 70)) * 703 = 21.52
        assert!((calculator.calculate_bmi() - 21.52).abs() < 0.01);

        // Test with different values
        let calculator = BMICalculator {
            weight_lbs: 180.0,
            height_in: 68.0,
        };
        // BMI = (180 / (68 * 68)) * 703 = 27.36
        assert!((calculator.calculate_bmi() - 27.36).abs() < 0.01);
    }

    #[test]
    fn calculate_bmi_handles_underweight() {
        // Setup values that would result in underweight BMI (< 18.5)
        let calculator = BMICalculator {
            weight_lbs: 110.0,
            height_in: 72.0,
        };
        // BMI = (110 / (72 * 72)) * 703 = 14.92
        let bmi = calculator.calculate_bmi();
        assert!(bmi < 18.0);
        assert!((bmi - 14.92).abs() < 0.01);
    }

    #[test]
    fn calculate_bmi_handles_healthy_weight() {
        // Setup values that would result in healthy BMI (18.5-25)
        let calculator = BMICalculator {
            weight_lbs: 150.0,
            height_in: 70.0,
        };
        // BMI = (150 / (70 * 70)) * 703 = 21.52
        let bmi = calculator.calculate_bmi();
        assert!((18.0..=25.0).contains(&bmi));
        assert!((bmi - 21.52).abs() < 0.01);
    }

    #[test]
    fn calculate_bmi_handles_overweight() {
        // Setup values that would result in overweight BMI (> 25)
        let calculator = BMICalculator {
            weight_lbs: 200.0,
            height_in: 68.0,
        };
        // BMI = (200 / (68 * 68)) * 703 = 30.40
        let bmi = calculator.calculate_bmi();
        assert!(bmi > 25.0);
        assert!((bmi - 30.40).abs() < 0.01);
    }

    #[test]
    fn calculate_bmi_handles_zero_height() {
        let calculator = BMICalculator {
            weight_lbs: 150.0,
            height_in: 0.0,
        };
        // Should return 0.0 to avoid division by zero
        assert_eq!(calculator.calculate_bmi(), 0.0);
    }
}
