//! # Compound Interest Calculator
//!
//! This module implements an interactive compound interest calculator application that
//! computes future investment value based on principal, rate, time, and compounding frequency.
//!
//! ## Features
//!
//! - **Interactive Interface**: GUI for entering investment parameters and viewing results
//! - **Real-time Calculation**: Results update automatically as values are changed
//! - **Compound Interest Formula**: Uses the standard formula P(1 + r/n)^(nt)
//! - **Input Validation**: Gracefully handles invalid numeric inputs
//! - **Edge Case Handling**: Properly manages zero values for all parameters
use eframe::egui::{self};

#[derive(Debug, Default)]
struct Investment {
    principal: String,
    rate: String,
    years: String,
    compound_frequency: String,
}

impl Investment {
    fn calculate_compound_interest(&self) -> Option<f64> {
        let principal: f64 = self.principal.parse().unwrap_or(f64::NAN);
        let rate: f64 = self.rate.parse().unwrap_or(f64::NAN);
        let years: f64 = self.years.parse().unwrap_or(f64::NAN);
        let compound_frequency: f64 = self.compound_frequency.parse().unwrap_or(f64::NAN);

        if principal.is_nan() || rate.is_nan() || years.is_nan() || compound_frequency.is_nan() {
            return None;
        }

        Some(
            principal
                * (1.0 + rate / (100.0 * compound_frequency)).powf(compound_frequency * years),
        )
    }
}

impl eframe::App for Investment {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("What is the principal amount?");
            ui.text_edit_singleline(&mut self.principal);
            ui.label("What is the rate?");
            ui.text_edit_singleline(&mut self.rate);
            ui.label("What is the number of years?");
            ui.text_edit_singleline(&mut self.years);
            ui.label("What is the number of times the interest is compounded per year?");
            ui.text_edit_singleline(&mut self.compound_frequency);

            if let Some(result) = self.calculate_compound_interest() {
                ui.label(format!(
                    "${} invested at {}% for {} years compounded {} times per year is ${:.2}.",
                    self.principal, self.rate, self.years, self.compound_frequency, result
                ));
            } else {
                ui.label("Please enter valid numbers for all fields.");
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
        "Compound Interest Calculator",
        options,
        Box::new(|_| Ok(Box::<Investment>::default())),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_compound_interest_calculates_correctly() {
        let investment = Investment {
            principal: String::from("1000"),
            rate: String::from("5"),
            years: String::from("10"),
            compound_frequency: String::from("12"),
        };

        if let Some(result) = investment.calculate_compound_interest() {
            // Expected: 1000 * (1 + 0.05/12)^(12*10) ≈ 1647.01
            assert!((result - 1647.01).abs() < 0.01);
        } else {
            panic!("calculate_compound_interest returned None when it should have returned Some");
        }
    }

    #[test]
    fn calculate_compound_interest_handles_invalid_inputs() {
        let investment_invalid_principal = Investment {
            principal: String::from("invalid"),
            rate: String::from("5"),
            years: String::from("10"),
            compound_frequency: String::from("12"),
        };

        assert!(
            investment_invalid_principal
                .calculate_compound_interest()
                .is_none()
        );

        let investment_invalid_rate = Investment {
            principal: String::from("1000"),
            rate: String::from("abc"),
            years: String::from("10"),
            compound_frequency: String::from("12"),
        };

        assert!(
            investment_invalid_rate
                .calculate_compound_interest()
                .is_none()
        );
    }

    #[test]
    fn calculate_compound_interest_handles_zero_values() {
        // Test with zero principal
        let investment_zero_principal = Investment {
            principal: String::from("0"),
            rate: String::from("5"),
            years: String::from("10"),
            compound_frequency: String::from("12"),
        };

        if let Some(result) = investment_zero_principal.calculate_compound_interest() {
            assert_eq!(result, 0.0);
        } else {
            panic!("calculate_compound_interest returned None when it should have returned Some");
        }

        // Test with zero rate
        let investment_zero_rate = Investment {
            principal: String::from("1000"),
            rate: String::from("0"),
            years: String::from("10"),
            compound_frequency: String::from("12"),
        };

        if let Some(result) = investment_zero_rate.calculate_compound_interest() {
            assert_eq!(result, 1000.0);
        } else {
            panic!("calculate_compound_interest returned None when it should have returned Some");
        }

        // Test with zero years
        let investment_zero_years = Investment {
            principal: String::from("1000"),
            rate: String::from("5"),
            years: String::from("0"),
            compound_frequency: String::from("12"),
        };

        if let Some(result) = investment_zero_years.calculate_compound_interest() {
            assert_eq!(result, 1000.0);
        } else {
            panic!("calculate_compound_interest returned None when it should have returned Some");
        }

        // Test with zero compound frequency (special case - avoid division by zero)
        let investment_zero_compound = Investment {
            principal: String::from("1000"),
            rate: String::from("5"),
            years: String::from("10"),
            compound_frequency: String::from("0"),
        };

        if let Some(result) = investment_zero_compound.calculate_compound_interest() {
            // With zero compounding, result should be principal (or we could define it as None)
            assert_eq!(result, 1000.0);
        } else {
            // Alternatively, the implementation might return None for this edge case
            // which would also be acceptable
        }
    }
}
