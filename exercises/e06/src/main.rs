//! # Retirement Calculator
//!
//! This module implements an interactive retirement planning application that
//! calculates when a user can retire based on their current age and desired retirement age.
//!
//! ## Features
//!
//! - **Age Input Validation**: Ensures entered ages are valid positive numbers
//! - **Retirement Planning**: Calculates years until retirement and retirement year
//! - **Early Retirement Detection**: Identifies when users should already be retired
//! - **User Interaction**: Provides clear prompts and feedback during input
//! - **Error Handling**: Gracefully handles invalid inputs with appropriate messages
use chrono::Datelike;
use std::io::Write;

fn prompt_for_age(question: &str) -> u32 {
    loop {
        print!("{question} ");
        let mut input = String::new();
        if let Err(e) = std::io::stdout().flush() {
            eprintln!("Error: {}", e);
            continue;
        }

        if let Err(e) = std::io::stdin().read_line(&mut input) {
            eprintln!("Error: {}", e);
            continue;
        }

        if let Ok(age) = input.trim().parse::<u32>() {
            if age == 0 {
                println!("Age cannot be zero. Please enter a valid age.");
                continue;
            }
            return age;
        } else {
            println!("Invalid age. Please enter a valid number.");
        }
    }
}

fn get_retirement_year(years_to_retirement: u32) -> u32 {
    let current_year = chrono::Utc::now().date_naive().year() as u32;
    current_year + years_to_retirement
}

fn main() {
    let curr_age = prompt_for_age("What is your current age?");
    let retirement_age = prompt_for_age("At what age do you plan to retire?");
    if retirement_age <= curr_age {
        println!("You should already be retired by now!");
    } else {
        let years_left = retirement_age - curr_age;
        println!("You have {years_left} years left until you can retire.");
        let retirement_year = get_retirement_year(years_left);
        println!("You will be able to retire in the year {retirement_year}.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Datelike;

    #[test]
    fn get_retirement_year_handles_zero_years() {
        let current_year = chrono::Utc::now().year() as u32;
        assert_eq!(get_retirement_year(0), current_year);
    }

    #[test]
    fn get_retirement_year_calculates_future_year_correctly() {
        let current_year = chrono::Utc::now().year() as u32;
        assert_eq!(get_retirement_year(10), current_year + 10);
        assert_eq!(get_retirement_year(25), current_year + 25);
        assert_eq!(get_retirement_year(40), current_year + 40);
    }

    #[test]
    fn get_retirement_year_handles_large_values() {
        let current_year = chrono::Utc::now().year() as u32;
        assert_eq!(get_retirement_year(100), current_year + 100);
    }
}
