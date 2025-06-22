//! # Pizza Calculator
//!
//! This module implements an interactive pizza ordering application that
//! calculates the number of pizzas needed based on people and slices per person.
//!
//! ## Features
//!
//! - **Party Size Input**: Collects the number of people in the party
//! - **Serving Size Input**: Determines how many slices each person wants
//! - **Optimal Pizza Calculation**: Calculates the minimum number of whole pizzas needed
//! - **Rounding Logic**: Always rounds up to ensure everyone gets enough slices
//! - **Input Validation**: Ensures valid numeric inputs through robust error handling
use std::io::Write;

fn prompt_for_uint(prompt: &str) -> u32 {
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

        if let Ok(value) = input.trim().parse::<u32>() {
            return value;
        } else {
            println!("Invalid input. Please enter a valid positive integer.");
        }
    }
}

fn calculate_num_pizzas(num_people: u32, slices_per_person: u32) -> u32 {
    if slices_per_person == 0 {
        return 0; // Avoid division by zero
    }
    (num_people * slices_per_person + 7) / 8 // Round up to nearest whole pizza
}

fn main() {
    let num_people = prompt_for_uint("How many people are in your party?");
    let num_pizzas = prompt_for_uint("How many slices per person?");
    let total_pizzas = calculate_num_pizzas(num_people, num_pizzas);
    println!(
        "You will need {} pizzas to feed {} people with {} slices each.",
        total_pizzas, num_people, num_pizzas
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_num_pizzas_handles_exact_division() {
        // Test cases where people * slices is exactly divisible by 8
        assert_eq!(calculate_num_pizzas(2, 4), 1); // 8 slices needed, 1 pizza
        assert_eq!(calculate_num_pizzas(4, 2), 1); // 8 slices needed, 1 pizza
        assert_eq!(calculate_num_pizzas(4, 4), 2); // 16 slices needed, 2 pizzas
    }

    #[test]
    fn calculate_num_pizzas_handles_inexact_division() {
        // Test cases where people * slices is not exactly divisible by 8
        assert_eq!(calculate_num_pizzas(3, 2), 1); // 6 slices needed, 1 pizza
        assert_eq!(calculate_num_pizzas(5, 2), 2); // 10 slices needed, 2 pizzas
        assert_eq!(calculate_num_pizzas(9, 1), 2); // 9 slices needed, 2 pizzas
    }

    #[test]
    fn calculate_num_pizzas_handles_zero_values() {
        // Test edge cases with zero values
        assert_eq!(calculate_num_pizzas(0, 5), 0); // 0 people means 0 pizzas
        assert_eq!(calculate_num_pizzas(5, 0), 0); // 0 slices per person means 0 pizzas
        assert_eq!(calculate_num_pizzas(0, 0), 0); // 0 people and 0 slices means 0 pizzas
    }

    #[test]
    fn calculate_num_pizzas_handles_large_values() {
        // Test with larger numbers
        assert_eq!(calculate_num_pizzas(20, 3), 8); // 60 slices needed, 8 pizzas
        assert_eq!(calculate_num_pizzas(100, 2), 25); // 200 slices needed, 25 pizzas
    }

    #[test]
    fn calculate_num_pizzas_rounds_up_correctly() {
        // Test proper rounding behavior (should round up)
        assert_eq!(calculate_num_pizzas(1, 1), 1); // 1 slice needed, still need 1 pizza
        assert_eq!(calculate_num_pizzas(1, 9), 2); // 9 slices needed, 2 pizzas
        assert_eq!(calculate_num_pizzas(3, 3), 2); // 9 slices needed, 2 pizzas
    }
}
