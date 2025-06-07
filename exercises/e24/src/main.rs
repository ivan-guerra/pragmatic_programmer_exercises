//! # Anagram Checker
//!
//! This module implements an interactive command-line tool that determines whether two strings
//! are anagrams of each other.
//!
//! ## Features
//!
//! - **User Interaction**: Prompts the user for two strings to compare
//! - **Anagram Detection**: Analyzes strings to determine if they are anagrams
//! - **Case Sensitivity**: Preserves case when determining anagram status
//! - **Special Character Support**: Considers spaces and special characters in comparisons
//! - **Input Validation**: Ensures non-empty string inputs with proper error messages
//! - **Comprehensive Testing**: Includes test cases for various anagram scenarios
use std::io::Write;

fn prompt_for_string(prompt: &str) -> String {
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
        let trimmed_input = input.trim().to_string();
        if !trimmed_input.is_empty() {
            return trimmed_input;
        } else {
            println!("Input cannot be empty. Please try again.");
        }
    }
}

fn is_anagram(s1: &str, s2: &str) -> bool {
    let mut chars1: Vec<char> = s1.chars().collect();
    let mut chars2: Vec<char> = s2.chars().collect();

    chars1.sort_unstable();
    chars2.sort_unstable();
    chars1 == chars2
}
fn main() {
    println!("Enter two strings to check if they are anagrams.");
    let str1 = prompt_for_string("Enter the first string:");
    let str2 = prompt_for_string("Enter the second string:");
    if is_anagram(&str1, &str2) {
        println!("The strings '{}' and '{}' are anagrams.", str1, str2);
    } else {
        println!("The strings '{}' and '{}' are not anagrams.", str1, str2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_anagram_identifies_anagrams_correctly() {
        // Basic anagram pairs
        assert!(is_anagram("listen", "silent"));
        assert!(is_anagram("heart", "earth"));
        assert!(is_anagram("night", "thing"));
        assert!(is_anagram("secure", "rescue"));
    }

    #[test]
    fn is_anagram_handles_case_sensitivity() {
        // Case should matter for anagrams
        assert!(!is_anagram("Listen", "Silent"));
        assert!(!is_anagram("HEART", "earth"));
        assert!(!is_anagram("Night", "Thing"));
    }

    #[test]
    fn is_anagram_handles_special_characters() {
        // Special characters and spaces should be part of the anagram
        assert!(!is_anagram("jim morrison", "mr mojo risin"));
        assert!(!is_anagram("Tom Marvolo Riddle", "I am Lord Voldemort"));
        assert!(!is_anagram("Astronomer", "Moon starer"));
    }

    #[test]
    fn is_anagram_identifies_non_anagrams() {
        // Different length strings
        assert!(!is_anagram("hello", "world"));
        assert!(!is_anagram("rust", "trust"));
        assert!(!is_anagram("test", "tests"));

        // Same length but different characters
        assert!(!is_anagram("rust", "dust"));
        assert!(!is_anagram("code", "cope"));
    }

    #[test]
    fn is_anagram_handles_empty_and_single_character_inputs() {
        // Empty strings are anagrams of each other
        assert!(is_anagram("", ""));

        // Single character strings
        assert!(is_anagram("a", "a"));
        assert!(!is_anagram("a", "b"));
    }
}
