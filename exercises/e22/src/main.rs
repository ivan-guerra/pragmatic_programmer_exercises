//! # Number Collection and Analysis
//!
//! This module implements an interactive command-line application that collects
//! a set of unique integers from the user and performs statistical analysis.
//!
//! ## Features
//!
//! - **Duplicate Detection**: Prevents duplicate numbers from being added to the collection
//! - **Interactive Input**: Allows users to enter numbers until they choose to quit
//! - **Input Validation**: Ensures valid integers through robust error handling
//! - **Statistical Analysis**: Identifies the maximum value in the collection
//! - **Graceful Handling**: Provides appropriate feedback when no numbers are entered
use std::{collections::HashSet, io::Write};

fn prompt_for_numbers() -> HashSet<i64> {
    let mut numbers = HashSet::new();
    loop {
        print!("Enter a number (or 'q' to quit): ");
        let mut input = String::new();
        if let Err(e) = std::io::stdout().flush() {
            eprintln!("Error: {}", e);
            continue;
        }

        if let Err(e) = std::io::stdin().read_line(&mut input) {
            eprintln!("Error: {}", e);
            continue;
        }

        let input = input.trim();
        if input.eq_ignore_ascii_case("q") {
            return numbers;
        }

        match input.parse::<i64>() {
            Ok(num) => {
                if !numbers.insert(num) {
                    println!("Number {} is already in the set.", num);
                }
            }
            Err(_) => {
                println!("Invalid input. Please enter a valid integer or 'q' to quit.");
            }
        }
    }
}

fn main() {
    let numbers = prompt_for_numbers();
    numbers.iter().max().map_or_else(
        || println!("No numbers were entered."),
        |max| println!("The maximum number entered is: {}", max),
    );
}
