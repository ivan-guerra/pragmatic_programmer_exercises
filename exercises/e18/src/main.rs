//! # Temperature Converter
//!
//! This module implements an interactive temperature conversion application that
//! allows users to convert between Celsius, Fahrenheit, and Kelvin temperature scales.
//!
//! ## Features
//!
//! - **Interactive Interface**: GUI for entering and viewing temperatures in different scales
//! - **Real-time Conversion**: Results update automatically as values are changed
//! - **Multiple Temperature Scales**: Support for Celsius, Fahrenheit and Kelvin
//! - **Input Flexibility**: Users can input temperature in any supported scale
//! - **Scientific Accuracy**: Uses standard temperature conversion formulas
use eframe::egui::{self};

#[derive(Debug, Default)]
struct TemperatureCalculator {
    celsius: f64,
    fahrenheit: f64,
    kelvin: f64,
}

impl TemperatureCalculator {
    fn celsius_to_fahrenheit(&self, celsius: f64) -> f64 {
        celsius * 9.0 / 5.0 + 32.0
    }

    fn fahrenheit_to_celsius(&self, fahrenheit: f64) -> f64 {
        (fahrenheit - 32.0) * 5.0 / 9.0
    }

    fn celsius_to_kelvin(&self, celsius: f64) -> f64 {
        celsius + 273.15
    }
}

impl eframe::App for TemperatureCalculator {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Temperature in Celsius:");
            if ui
                .add(egui::DragValue::new(&mut self.celsius).speed(0.1))
                .changed()
            {
                self.fahrenheit = self.celsius_to_fahrenheit(self.celsius);
                self.kelvin = self.celsius_to_kelvin(self.celsius);
            }
            ui.label("Temperature in Fahrenheit:");
            if ui
                .add(egui::DragValue::new(&mut self.fahrenheit).speed(0.1))
                .changed()
            {
                self.celsius = self.fahrenheit_to_celsius(self.fahrenheit);
                self.kelvin = self.celsius_to_kelvin(self.celsius);
            }
            ui.label("Temperature in Kelvin:");
            if ui
                .add(egui::DragValue::new(&mut self.kelvin).speed(0.1))
                .changed()
            {
                self.celsius = self.kelvin - 273.15;
                self.fahrenheit = self.celsius_to_fahrenheit(self.celsius);
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
        "Temperature Calculator",
        options,
        Box::new(|_| Ok(Box::<TemperatureCalculator>::default())),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn celsius_to_fahrenheit_calculates_correctly() {
        let calculator = TemperatureCalculator::default();
        assert_eq!(calculator.celsius_to_fahrenheit(0.0), 32.0); // freezing point
        assert_eq!(calculator.celsius_to_fahrenheit(100.0), 212.0); // boiling point
        assert_eq!(calculator.celsius_to_fahrenheit(25.0), 77.0); // room temperature
        assert_eq!(calculator.celsius_to_fahrenheit(-40.0), -40.0); // equal point
    }

    #[test]
    fn fahrenheit_to_celsius_calculates_correctly() {
        let calculator = TemperatureCalculator::default();
        assert_eq!(calculator.fahrenheit_to_celsius(32.0), 0.0); // freezing point
        assert_eq!(calculator.fahrenheit_to_celsius(212.0), 100.0); // boiling point
        assert_eq!(calculator.fahrenheit_to_celsius(77.0), 25.0); // room temperature
        assert_eq!(calculator.fahrenheit_to_celsius(-40.0), -40.0); // equal point
    }

    #[test]
    fn celsius_to_kelvin_calculates_correctly() {
        let calculator = TemperatureCalculator::default();
        assert_eq!(calculator.celsius_to_kelvin(0.0), 273.15); // freezing point
        assert_eq!(calculator.celsius_to_kelvin(100.0), 373.15); // boiling point
        assert_eq!(calculator.celsius_to_kelvin(-273.15), 0.0); // absolute zero
    }

    #[test]
    fn temperature_conversions_are_consistent() {
        let calculator = TemperatureCalculator::default();

        // Test round-trip conversions
        let original_celsius = 25.0;
        let fahrenheit = calculator.celsius_to_fahrenheit(original_celsius);
        let back_to_celsius = calculator.fahrenheit_to_celsius(fahrenheit);

        // Allow for small floating-point differences
        assert!((original_celsius - back_to_celsius).abs() < 0.0001);

        // Test consistency between all units
        let celsius = 15.0;
        let fahrenheit = calculator.celsius_to_fahrenheit(celsius);
        let kelvin = calculator.celsius_to_kelvin(celsius);

        assert_eq!(celsius, calculator.fahrenheit_to_celsius(fahrenheit));
        assert_eq!(kelvin - 273.15, celsius);
    }
}
