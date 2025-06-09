//! # Rate of Return Calculator
//!
//! This module implements a command-line application that calculates how many years
//! it will take to double an investment based on the Rule of 72.
//!
//! ## Features
//!
//! - **Interactive Input**: Prompts user for the rate of return
//! - **Input Validation**: Ensures valid numeric rate values through robust error handling
//! - **Investment Rule Application**: Implements the Rule of 72 financial formula
//! - **Zero Value Protection**: Prevents division by zero with appropriate validation
//! - **User-Friendly Output**: Displays the calculated doubling time in years
//! - **Error Recovery**: Allows users to retry after invalid inputs
use std::io::Write;

fn prompt_for_rate_of_return() -> f64 {
    loop {
        print!("Enter the rate of return (as a percentage): ");
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
            if value == 0.0 {
                println!("Rate of return cannot be zero. Please enter a valid number.");
                continue;
            }
            return value;
        } else {
            println!("Invalid input. Please enter a valid number.");
        }
    }
}

fn main() {
    let rate_of_return = prompt_for_rate_of_return();
    println!(
        "It will take {} years to double your intial investement.",
        (72.0 / rate_of_return).ceil() as u32
    );
}
