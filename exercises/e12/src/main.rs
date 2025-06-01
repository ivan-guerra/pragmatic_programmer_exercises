//! # Interest Calculator
//!
//! This module implements an interactive interest calculator application that
//! computes simple interest and provides a year-by-year breakdown of investment growth.
//!
//! ## Features
//!
//! - **Investment Analysis**: Calculates the future value of an investment with interest
//! - **Year-by-Year Breakdown**: Shows interest earned for each individual year
//! - **User Interaction**: Prompts for principal amount, interest rate, and time period
//! - **Formatted Output**: Displays results with proper currency formatting and decimal precision
//! - **Input Validation**: Ensures valid numeric inputs through robust error handling
use std::io::Write;

fn prompt_for_float(prompt: &str) -> f64 {
    loop {
        print!("{prompt} ");
        let mut input = String::new();
        if let Err(e) = std::io::stdout().flush() {
            eprintln!("Error: {}", e);
            continue;
        }

        if let Err(e) = std::io::stdin().read_line(&mut input) {
            eprintln!("Error: {}", e);
            continue;
        }

        if let Ok(value) = input.trim().parse::<f64>() {
            return value;
        } else {
            println!("Invalid input. Please enter a valid number.");
        }
    }
}

fn compute_simple_interest(principal: f64, rate: f64, years: f64) -> f64 {
    principal * (rate / 100.0) * years
}

fn main() {
    let principal = prompt_for_float("Enter the principal amount:");
    let rate = prompt_for_float("Enter the annual interest rate (as a percentage):");
    let years = prompt_for_float("Enter the number of years:");
    println!(
        "After {} years at at {:.2}%, the investment will be worth: ${:.2}.",
        years,
        rate,
        principal + compute_simple_interest(principal, rate, years)
    );

    let yearly_simple_interest: Vec<f64> = (1..=years as u32)
        .map(|year| compute_simple_interest(principal, rate, year as f64))
        .collect();
    println!("Here's the breakdown of interest earned each year:");
    for (year, interest) in yearly_simple_interest.iter().enumerate() {
        println!("Year {}: ${:.2}", year + 1, interest);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_simple_interest_calculates_correctly() {
        // Test with standard values
        assert_eq!(compute_simple_interest(1000.0, 5.0, 1.0), 50.0); // $1000 at 5% for 1 year = $50
        assert_eq!(compute_simple_interest(1500.0, 4.3, 3.0), 193.5); // $1500 at 4.3% for 3 years = $193.5
    }

    #[test]
    fn compute_simple_interest_handles_zero_values() {
        // Test edge cases with zero values
        assert_eq!(compute_simple_interest(0.0, 5.0, 1.0), 0.0); // $0 principal should result in $0 interest
        assert_eq!(compute_simple_interest(1000.0, 0.0, 1.0), 0.0); // 0% rate should result in $0 interest
        assert_eq!(compute_simple_interest(1000.0, 5.0, 0.0), 0.0); // 0 years should result in $0 interest
    }

    #[test]
    fn compute_simple_interest_handles_large_values() {
        // Test with larger numbers
        assert_eq!(compute_simple_interest(100000.0, 2.5, 10.0), 25000.0); // $100,000 at 2.5% for 10 years = $25,000
        assert_eq!(compute_simple_interest(50000.0, 7.5, 5.0), 18750.0); // $50,000 at 7.5% for 5 years = $18,750
    }
}
