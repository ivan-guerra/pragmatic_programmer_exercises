//! # Credit Card Payment Calculator
//!
//! This module implements an interactive GUI application that calculates how long it will take
//! to pay off a credit card balance based on payment amount and APR.
//!
//! ## Features
//!
//! - **Interactive Interface**: Real-time calculation as values are adjusted
//! - **Financial Formula**: Implements standard credit card payment duration formula
//! - **Key Parameters**: Takes into account balance, APR, and monthly payment amount
//! - **Daily Rate Calculation**: Correctly converts annual percentage rate to daily rate
//! - **Visual Feedback**: Displays number of months until the balance is paid off
use eframe::egui::{self};

#[derive(Debug, Default)]
struct PaymentCalculator {
    daily_rate: f64,
    apr: f64,
    balance: f64,
    monthly_payment: f64,
}

impl PaymentCalculator {
    fn calculate_months_until_paid_off(&self) -> u32 {
        let term1 = -(1.0 / 30.0);
        let numerator = (1.0
            + (self.balance / self.monthly_payment) * (1.0 - (1.0 + self.daily_rate).powf(30.0)))
        .log10();
        let denominator = (1.0 + self.daily_rate).log10();

        (term1 * (numerator / denominator)).ceil() as u32
    }
}

impl eframe::App for PaymentCalculator {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("What is your balance:");
            ui.add(egui::DragValue::new(&mut self.balance).speed(0.01));
            ui.label("What is the APR on the card:");
            if ui
                .add(egui::DragValue::new(&mut self.apr).speed(0.1))
                .changed()
            {
                self.daily_rate = self.apr / 100.0 / 365.0;
            }
            ui.label("What is the monthly payment you can make:");
            ui.add(egui::DragValue::new(&mut self.monthly_payment).speed(0.1));

            ui.label(format!(
                "Months until paid off: {}",
                self.calculate_months_until_paid_off()
            ));
        });
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([400.0, 150.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Monthly Payment Calculator",
        options,
        Box::new(|_| Ok(Box::<PaymentCalculator>::default())),
    )
}
