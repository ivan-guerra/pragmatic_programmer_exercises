//! # Name Greeter
//!
//! This module implements an interactive name greeting application
//! that prompts for a user's name and generates personalized greetings.
//!
//! ## Features
//!
//! - **Input Validation**: Ensures names contain only alphabetic characters and spaces
//! - **Personalized Greetings**: Provides different greetings based on the first letter of the name
//! - **Case Handling**: Processes names in a case-insensitive manner
//! - **Unicode Support**: Properly handles non-ASCII characters in names
//! - **Whitespace Handling**: Trims excess whitespace from user input
//! - **Error Handling**: Provides clear feedback for invalid inputs
use std::io::Write;

fn is_valid_name(name: &str) -> bool {
    !name.trim().is_empty() && name.chars().all(|c| c.is_alphabetic() || c.is_whitespace())
}

fn prompt_for_name() -> String {
    loop {
        print!("What is your name? ");
        let mut input = String::new();
        if let Err(e) = std::io::stdout().flush() {
            eprintln!("Error: {}", e);
            continue;
        }

        if let Err(e) = std::io::stdin().read_line(&mut input) {
            eprintln!("Error: {}", e);
            continue;
        }

        if is_valid_name(&input) {
            return input.trim().to_string();
        } else {
            println!("Invalid name. Please enter a valid name containing only alphabetic characters and spaces.");
        }
    }
}

fn generate_greeting(name: &str) -> String {
    // Provide one of two greetings based on where the first character of the name falls in the
    // alphabet. Names starting with letters A-M get one greeting, and names starting with N-Z get
    // another.
    let first_char = name.chars().next().unwrap_or(' ');
    if first_char.is_alphabetic() && first_char.to_ascii_lowercase() < 'n' {
        format!("Hello, {}! Nice to meet you!", name)
    } else {
        format!("Hello, {}! It's great to see you!", name)
    }
}

fn main() {
    let name = prompt_for_name();
    println!("{}", generate_greeting(&name));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_valid_name_accepts_single_word_name() {
        assert!(is_valid_name("John"));
        assert!(is_valid_name("Mary"));
        assert!(is_valid_name("Alexandria"));
    }

    #[test]
    fn is_valid_name_accepts_multi_word_names() {
        assert!(is_valid_name("John Doe"));
        assert!(is_valid_name("Mary Jane Watson"));
        assert!(is_valid_name("James Robert Smith"));
    }

    #[test]
    fn is_valid_name_rejects_empty_strings() {
        assert!(!is_valid_name(""));
        assert!(!is_valid_name("   "));
        assert!(!is_valid_name("\t\n"));
    }

    #[test]
    fn is_valid_name_rejects_names_with_numbers() {
        assert!(!is_valid_name("John2"));
        assert!(!is_valid_name("Mary 123"));
        assert!(!is_valid_name("Agent007"));
    }

    #[test]
    fn is_valid_name_rejects_names_with_symbols() {
        assert!(!is_valid_name("John!"));
        assert!(!is_valid_name("Mary-Jane"));
        assert!(!is_valid_name("Smith@example.com"));
        assert!(!is_valid_name("O'Reilly"));
    }

    #[test]
    fn is_valid_name_handles_whitespace_correctly() {
        assert!(is_valid_name("John Doe"));
        assert!(is_valid_name("   John   ")); // Will be trimmed
        assert!(is_valid_name("\tJohn\n")); // Will be trimmed
    }

    #[test]
    fn is_valid_name_handles_non_ascii_letters_correctly() {
        assert!(is_valid_name("José"));
        assert!(is_valid_name("Søren"));
        assert!(is_valid_name("Naïve"));
    }

    #[test]
    fn generate_greeting_provides_first_greeting_for_a_to_m_names() {
        assert_eq!(generate_greeting("Adam"), "Hello, Adam! Nice to meet you!");
        assert_eq!(generate_greeting("John"), "Hello, John! Nice to meet you!");
        assert_eq!(generate_greeting("Mary"), "Hello, Mary! Nice to meet you!");
    }

    #[test]
    fn generate_greeting_provides_second_greeting_for_n_to_z_names() {
        assert_eq!(
            generate_greeting("Nancy"),
            "Hello, Nancy! It's great to see you!"
        );
        assert_eq!(
            generate_greeting("Peter"),
            "Hello, Peter! It's great to see you!"
        );
        assert_eq!(
            generate_greeting("Zoe"),
            "Hello, Zoe! It's great to see you!"
        );
    }

    #[test]
    fn generate_greeting_handles_case_insensitive_comparisons() {
        assert_eq!(generate_greeting("adam"), "Hello, adam! Nice to meet you!");
        assert_eq!(generate_greeting("MARY"), "Hello, MARY! Nice to meet you!");
        assert_eq!(
            generate_greeting("Nathan"),
            "Hello, Nathan! It's great to see you!"
        );
    }

    #[test]
    fn generate_greeting_handles_empty_and_non_alphabetic_first_characters() {
        assert_eq!(generate_greeting(""), "Hello, ! It's great to see you!");
        assert_eq!(
            generate_greeting("123John"),
            "Hello, 123John! It's great to see you!"
        );
        assert_eq!(
            generate_greeting(" Alice"),
            "Hello,  Alice! It's great to see you!"
        );
    }
}
