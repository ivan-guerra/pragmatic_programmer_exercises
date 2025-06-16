//! # Secure Password Generator
//!
//! This module implements a command-line application for generating customized secure
//! passwords based on user-defined requirements for character composition.
//!
//! ## Features
//!
//! - **Customizable Composition**: Allows users to specify the number of alphabetic characters,
//!   special characters, and digits
//! - **Random Generation**: Uses cryptographically secure randomization for password creation
//! - **Character Shuffling**: Randomizes the order of characters to enhance security
//! - **Clipboard Integration**: Automatically copies the generated password to the system clipboard
//! - **Type Safety**: Uses strongly-typed components to represent password character types
//!
//! The application prompts the user for password composition requirements, generates a
//! password that meets those requirements, and automatically copies it to the clipboard
//! for convenient and secure use.
use arboard::Clipboard;
use once_cell::sync::Lazy;
use rand::prelude::IndexedRandom;
use rand::seq::SliceRandom;
use std::io::{self, Write};

static SPECIAL_CHARS: Lazy<Vec<char>> = Lazy::new(|| {
    vec![
        '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '_', '=', '+', '{', '}', '[', ']',
        ':', ';', '"', '\'', '<', '>', ',', '.', '?', '/', '\\',
    ]
});

static DIGITS: Lazy<Vec<char>> =
    Lazy::new(|| vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']);

static ALPHABET: Lazy<Vec<char>> = Lazy::new(|| {
    let mut alphabet = Vec::new();
    for c in 'a'..='z' {
        alphabet.push(c);
        alphabet.push(c.to_ascii_uppercase());
    }
    alphabet
});

#[derive(Debug, Clone)]
enum PasswordComponent {
    AlphaChar,
    Digit,
    SpecialChar,
}

fn prompt_for_components() -> Result<Vec<PasswordComponent>, std::io::Error> {
    let mut components = Vec::new();
    let mut input = String::new();

    print!("How many alphabetic characters do you want in your password? (default: 8): ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;
    let num_chars: usize = input.trim().parse().unwrap_or(8);
    components.extend(vec![PasswordComponent::AlphaChar; num_chars]);

    input.clear();
    print!("How many special characters do you want in your password? (default: 0): ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;
    let num_specials: usize = input.trim().parse().unwrap_or(0);
    components.extend(vec![PasswordComponent::SpecialChar; num_specials]);

    input.clear();
    print!("How many digits do you want in your password? (default: 0): ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;
    let num_digits: usize = input.trim().parse().unwrap_or(0);
    components.extend(vec![PasswordComponent::Digit; num_digits]);

    Ok(components)
}

fn generate_password(mut components: Vec<PasswordComponent>) -> String {
    let mut rng = rand::rng();

    components.shuffle(&mut rng);
    components
        .iter()
        .map(|component| match component {
            PasswordComponent::AlphaChar => *ALPHABET.choose(&mut rng).unwrap(),
            PasswordComponent::Digit => *DIGITS.choose(&mut rng).unwrap(),
            PasswordComponent::SpecialChar => *SPECIAL_CHARS.choose(&mut rng).unwrap(),
        })
        .collect()
}

fn main() {
    let components = prompt_for_components();
    if let Ok(components) = components {
        if components.is_empty() {
            println!("No components selected. Exiting.");
            return;
        }

        let password = generate_password(components);
        let mut clipboard = Clipboard::new().expect("Failed to access clipboard");
        clipboard
            .set_text(password)
            .expect("Failed to set clipboard text");
        println!("password copied to clipboard");
    } else if let Err(e) = components {
        eprintln!("Error reading components: {}", e);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_password_creates_correct_length() {
        let components = vec![
            PasswordComponent::AlphaChar,
            PasswordComponent::Digit,
            PasswordComponent::SpecialChar,
        ];

        let password = generate_password(components);
        assert_eq!(password.len(), 3);
    }

    #[test]
    fn generate_password_includes_requested_components() {
        let components = vec![
            PasswordComponent::AlphaChar,
            PasswordComponent::Digit,
            PasswordComponent::SpecialChar,
            PasswordComponent::AlphaChar,
        ];

        let password = generate_password(components);

        assert_eq!(password.len(), 4);

        let has_alpha = password.chars().any(|c| c.is_alphabetic());
        let has_digit = password.chars().any(|c| c.is_numeric());
        let has_special = password.chars().any(|c| !c.is_alphanumeric());

        assert!(has_alpha, "Password should contain alphabetic characters");
        assert!(has_digit, "Password should contain digits");
        assert!(has_special, "Password should contain special characters");
    }

    #[test]
    fn generate_password_handles_empty_components() {
        let components = vec![];
        let password = generate_password(components);
        assert!(
            password.is_empty(),
            "Password should be empty when no components are provided"
        );
    }
}
