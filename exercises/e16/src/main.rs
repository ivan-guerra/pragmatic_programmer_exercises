//! # Driving Age Verification
//!
//! This module implements a simple command-line application that determines
//! whether a user is legally eligible to drive based on their age input.
//!
//! ## Features
//!
//! - **Age Input Collection**: Prompts the user for their age
//! - **Input Validation**: Ensures valid numeric age through error handling
//! - **Eligibility Check**: Compares user age against legal driving age
//! - **Clear Feedback**: Provides immediate response on driving eligibility
use std::io::Write;

fn prompt_for_age() -> u32 {
    loop {
        print!("What is your age? ");
        let mut input = String::new();
        if let Err(e) = std::io::stdout().flush() {
            eprintln!("Error: {}", e);
            continue;
        }

        if let Err(e) = std::io::stdin().read_line(&mut input) {
            eprintln!("Error: {}", e);
            continue;
        }

        if let Ok(value) = input.trim().parse::<u32>() {
            return value;
        } else {
            println!("Invalid input. Please enter a valid positive integer.");
        }
    }
}

fn main() {
    const LEGAL_AGE: u32 = 16;
    let age = prompt_for_age();

    if age >= LEGAL_AGE {
        println!("You are old enough to legally drive.");
    } else {
        println!("You are not old enough to legally drive.");
    }
}
