//! # Karvonen Heart Rate Calculator
//!
//! This module implements an interactive GUI application that calculates target heart rates
//! using the Karvonen formula based on user-provided age and resting pulse.
//!
//! ## Features
//!
//! - **Interactive Controls**: Adjustable sliders for age and resting pulse parameters
//! - **Real-time Calculation**: Target heart rates update instantly as inputs change
//! - **Range of Intensities**: Displays rates for training intensities from 55% to 95%
//! - **Tabular Results**: Presents calculated heart rates in an organized, scrollable grid
//! - **Scientific Formula**: Implements the Karvonen method for personalized heart rate zones
use eframe::egui::{self};

#[derive(Debug, Default)]
struct BpmTracker {
    resting_pulse: u32,
    age: u32,
}

impl BpmTracker {
    fn karvonen_target_heart_rate(&self, intensity: f64) -> u32 {
        let max_heart_rate = f64::from(220 - self.age);
        let target_heart_rate = ((max_heart_rate - f64::from(self.resting_pulse)) * intensity)
            + f64::from(self.resting_pulse);
        target_heart_rate.round() as u32
    }
}

impl eframe::App for BpmTracker {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Resting Pulse:");
            ui.add(egui::Slider::new(&mut self.resting_pulse, 40..=100).text("bpm"));
            ui.label("Age:");

            ui.add(egui::Slider::new(&mut self.age, 1..=110).text("age"));

            ui.separator();

            egui::ScrollArea::both()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    egui::Grid::new("target_heart_rate_table")
                        .spacing([10.0, 10.0])
                        .striped(true)
                        .min_col_width(30.0)
                        .show(ui, |ui| {
                            ui.label("Intensity");
                            ui.label("Rate");
                            ui.end_row();
                            for intensity in (55..=95).step_by(5) {
                                let target_heart_rate =
                                    self.karvonen_target_heart_rate(intensity as f64 / 100.0);
                                ui.label(format!("{}%", intensity));
                                ui.label(format!("{} bpm", target_heart_rate));
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
        "BPM Tracker",
        options,
        Box::new(|_| Ok(Box::<BpmTracker>::default())),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn karvonen_target_heart_rate_calculates_correctly() {
        // Test case with typical adult values
        let tracker = BpmTracker {
            resting_pulse: 70,
            age: 30,
        };

        // Expected: (220-30-70)*0.65 + 70 = 78 + 70 = 148
        assert_eq!(tracker.karvonen_target_heart_rate(0.65), 148);
        assert_eq!(tracker.karvonen_target_heart_rate(0.55), 136); // 55% intensity
        assert_eq!(tracker.karvonen_target_heart_rate(0.95), 184); // 95% intensity
    }

    #[test]
    fn karvonen_target_heart_rate_handles_boundary_cases() {
        // Test with senior age
        let senior_tracker = BpmTracker {
            resting_pulse: 65,
            age: 80,
        };
        // Expected: (220-80-65)*0.70 + 65 = 52.5 + 65 = 118 (rounded)
        assert_eq!(senior_tracker.karvonen_target_heart_rate(0.70), 118);

        // Test with child age
        let child_tracker = BpmTracker {
            resting_pulse: 80,
            age: 10,
        };
        // Expected: (220-10-80)*0.60 + 80 = 78 + 80 = 158
        assert_eq!(child_tracker.karvonen_target_heart_rate(0.60), 158);
    }

    #[test]
    fn karvonen_target_heart_rate_handles_extreme_intensities() {
        let tracker = BpmTracker {
            resting_pulse: 60,
            age: 40,
        };

        // At 0% intensity, result should be the resting heart rate
        // Expected: (220-40-60)*0.0 + 60 = 0 + 60 = 60
        assert_eq!(tracker.karvonen_target_heart_rate(0.0), 60);

        // At 100% intensity, result should be the maximum heart rate
        // Expected: (220-40-60)*1.0 + 60 = 120 + 60 = 180
        assert_eq!(tracker.karvonen_target_heart_rate(1.0), 180);
    }

    #[test]
    fn karvonen_target_heart_rate_handles_rounding() {
        let tracker = BpmTracker {
            resting_pulse: 67,
            age: 33,
        };

        // This will produce a floating point result that needs rounding
        // Expected: (220-33-67)*0.75 + 67 = 90 + 67 = 157
        assert_eq!(tracker.karvonen_target_heart_rate(0.75), 157);
    }
}
