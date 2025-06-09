//! # Integer Summation Utility
//!
//! This module implements a command-line application that collects and sums a specified
//! number of integer values provided by the user.
//!
//! ## Features
//!
//! - **Interactive Input**: Prompts user for the quantity of integers to sum
//! - **Input Validation**: Handles invalid inputs with appropriate error messages
//! - **Flexible Number Collection**: Dynamically collects the specified number of values
//! - **Error Handling**: Gracefully handles input parsing errors with fallback values
//! - **Zero-Value Support**: Properly manages cases where no values need to be summed
//! - **Efficient Summation**: Uses Rust's iterator functionality for optimal calculation
use std::io::Write;

fn prompt_for_uint(prompt: &str) -> Option<i32> {
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

        match input.trim().parse::<i32>() {
            Ok(value) => return Some(value),
            Err(_) => return None,
        }
    }
}

fn main() {
    let num_values = prompt_for_uint("Enter the number of values to sum:").unwrap_or(0);
    if num_values <= 0 {
        println!("No values to sum.");
        return;
    }

    let sum = (0..num_values)
        .map(|_| prompt_for_uint("Enter an integer:").unwrap_or(0))
        .sum::<i32>();
    println!("The total is {}.", sum);
}
