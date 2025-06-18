//! # Even Number Filter
//!
//! This module implements a command-line application for parsing a list of numbers
//! from a file and identifying the even numbers within that list.
//!
//! ## Features
//!
//! - **File I/O**: Reads numeric data from a text file
//! - **Number Filtering**: Identifies and extracts even numbers from the input
//! - **Error Handling**: Gracefully manages file access and parsing errors
//! - **Formatted Output**: Presents the results in a clear, readable format
//!
//! The application reads a list of numbers from a file, filters out the odd numbers,
//! and displays the even numbers in a formatted string, handling cases where no even
//! numbers are found.
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

fn print_even_nums(file_path: PathBuf) -> Result<Vec<u32>, std::io::Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut times = Vec::new();

    for line in reader.lines() {
        let time_str = line?;
        if let Ok(time) = time_str.trim().parse::<u32>() {
            if time & 1 != 1 {
                times.push(time);
            }
        }
    }
    Ok(times)
}
fn main() {
    let file_path = PathBuf::from("exercises/e38/inputs/numbers.txt");
    match print_even_nums(file_path) {
        Ok(even_nums) => {
            if even_nums.is_empty() {
                println!("No even numbers found in the file.");
            } else {
                let even_numbers_str = even_nums
                    .iter()
                    .map(|num| num.to_string())
                    .collect::<Vec<String>>()
                    .join(" ");
                println!("The even numbers are: {}.", even_numbers_str);
            }
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}
