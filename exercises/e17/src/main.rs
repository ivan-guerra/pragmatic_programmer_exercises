//! # Blood Alcohol Content Calculator
//!
//! This module implements a command-line application that calculates a user's
//! blood alcohol content (BAC) based on their weight, gender, alcohol consumption,
//! and time since their last drink.
//!
//! ## Features
//!
//! - **Gender-Based Calculation**: Applies different alcohol distribution ratios based on gender
//! - **Input Validation**: Ensures valid numeric inputs through robust error handling
//! - **Legal Limit Check**: Determines if the calculated BAC is above or below the legal limit
//! - **Time Consideration**: Factors in hours since last drink to account for alcohol metabolism
//!
//! The formula used is: BAC = (A × 5.14 / W × r) - (0.015 × H) where:
//! - A = Total alcohol consumed in ounces
//! - W = Body weight in pounds
//! - r = Alcohol distribution ratio (0.73 for men, 0.66 for women)
//! - H = Hours since last drink
use std::io::Write;

enum Gender {
    Male,
    Female,
}

fn prompt_for_weight() -> f64 {
    loop {
        print!("Enter the weight in pounds: ");
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

fn prompt_for_gender() -> Gender {
    loop {
        print!("Enter your gender (M for male or F for female): ");
        let mut input = String::new();
        if let Err(e) = std::io::stdout().flush() {
            eprintln!("Error: {}", e);
            continue;
        }
        if let Err(e) = std::io::stdin().read_line(&mut input) {
            eprintln!("Error: {}", e);
            continue;
        }

        let input = input.trim().to_uppercase();
        match input.as_str() {
            "M" => return Gender::Male,
            "F" => return Gender::Female,
            _ => {
                println!("Invalid input. Please enter 'M' or 'F'.");
            }
        }
    }
}

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

fn calculate_bac(weight_lb: f64, gender: Gender, hours: f64, total_alcohol_oz: f64) -> f64 {
    let r = match gender {
        Gender::Male => 0.73,
        Gender::Female => 0.66,
    };
    (total_alcohol_oz * 5.14 / weight_lb * r) - (0.015 * hours)
}

fn main() {
    let weight_lb = prompt_for_weight();
    let gender = prompt_for_gender();
    let hours_since_last_drink =
        prompt_for_float("How many hours have passed since your last drink?");
    let total_alcohol_oz = prompt_for_float("How many ounces of alcohol have you consumed?");

    const BAC_LIMIT: f64 = 0.08;
    let bac = calculate_bac(weight_lb, gender, hours_since_last_drink, total_alcohol_oz);
    if bac >= BAC_LIMIT {
        println!("You are over the legal limit with a BAC of {:.2}.", bac);
    } else {
        println!("You are within the legal limit with a BAC of {:.2}.", bac);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_bac_handles_male_values() {
        // Test with male gender
        let weight = 160.0;
        let gender = Gender::Male;
        let hours = 2.0;
        let alcohol_oz = 5.0;

        // (5.0 * 5.14 / 160.0 * 0.73) - (0.015 * 2.0) = 0.117 - 0.03 = 0.087
        let expected = 0.087;
        let actual = calculate_bac(weight, gender, hours, alcohol_oz);

        assert!((actual - expected).abs() < 0.001);
    }

    #[test]
    fn calculate_bac_handles_female_values() {
        // Test with female gender
        let weight = 140.0;
        let gender = Gender::Female;
        let hours = 1.0;
        let alcohol_oz = 4.0;

        // (4.0 * 5.14 / 140.0 * 0.66) - (0.015 * 1.0) = 0.097 - 0.015 = 0.082
        let expected = 0.082;
        let actual = calculate_bac(weight, gender, hours, alcohol_oz);

        assert!((actual - expected).abs() < 0.001);
    }

    #[test]
    fn calculate_bac_handles_zero_alcohol() {
        let weight = 180.0;
        let gender = Gender::Male;
        let hours = 3.0;
        let alcohol_oz = 0.0;

        // (0.0 * 5.14 / 180.0 * 0.73) - (0.015 * 3.0) = 0.0 - 0.045 = -0.045
        let expected = -0.045;
        let actual = calculate_bac(weight, gender, hours, alcohol_oz);

        assert!((actual - expected).abs() < 0.001);
    }

    #[test]
    fn calculate_bac_handles_zero_hours() {
        let weight = 200.0;
        let gender = Gender::Female;
        let hours = 0.0;
        let alcohol_oz = 6.0;

        // (6.0 * 5.14 / 200.0 * 0.66) - (0.015 * 0.0) = 0.102 - 0.0 = 0.102
        let expected = 0.102;
        let actual = calculate_bac(weight, gender, hours, alcohol_oz);

        assert!((actual - expected).abs() < 0.001);
    }
}
