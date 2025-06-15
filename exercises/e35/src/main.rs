//! # Contestant Winner Selector
//!
//! This module implements a command-line application that randomly selects winners
//! from a list of contestant names provided by user input.
//!
//! ## Features
//!
//! - **Interactive Input**: Allows users to enter contestant names one by one
//! - **Random Selection**: Uses randomization to fairly select winners from the contestant pool
//! - **Multiple Winners**: Automatically selects multiple winners (half the contestants + 1)
//! - **No Duplicates**: Ensures the same person cannot win multiple times
//!
//! The application prompts for contestant names until the user enters a blank line,
//! then randomly selects and announces the winners.
use rand::seq::IndexedRandom;
use std::io::{self, Write};

fn prompt_for_name() -> Option<String> {
    print!("Enter a contestant name (or blank to save and exit): ");
    io::stdout().flush().unwrap();

    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();
    if name.trim().is_empty() {
        None
    } else {
        Some(name.trim().to_string())
    }
}

fn main() {
    let mut names = Vec::new();
    while let Some(name) = prompt_for_name() {
        names.push(name);
    }

    let num_winners = (names.len() / 2) + 1;
    let mut rng = rand::rng();
    for i in 0..num_winners {
        let winner_name = names.choose(&mut rng).cloned();
        if let Some(winner) = winner_name {
            println!("The winner {} is {}!", i + 1, winner);
            names.retain(|n| n != &winner);
        } else {
            println!("No contestants were entered.");
        }
    }
}
