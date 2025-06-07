//! # Multilingual Month Translator
//!
//! This module implements an interactive program that translates numeric month values
//! into their names in different languages.
//!
//! ## Features
//!
//! - **Language Selection**: Allows users to choose between English and Spanish
//! - **Input Validation**: Ensures valid month numbers through robust error handling
//! - **Multilingual Support**: Provides month names in the user's selected language
//! - **Complete Coverage**: Handles all twelve months with proper translations
//! - **Localized Messages**: Displays prompts and error messages in the selected language
use std::io::Write;

enum Language {
    English,
    Spanish,
}

fn prompt_for_language() -> Language {
    loop {
        print!("Enter your preferred language (E for English or S for Spanish): ");
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
            "E" => return Language::English,
            "S" => return Language::Spanish,
            _ => {
                println!("Invalid input. Please enter 'E' or 'S'.");
            }
        }
    }
}

fn prompt_for_month_num(prompt: &str, err: &str) -> u8 {
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

        if let Ok(value) = input.trim().parse::<u8>() {
            if !(1..=12).contains(&value) {
                println!("{err}");
                continue;
            }
            return value;
        } else {
            println!("{err}");
        }
    }
}

fn get_month_name(month_num: u8, language: Language) -> String {
    match language {
        Language::English => match month_num {
            1 => "January".to_string(),
            2 => "February".to_string(),
            3 => "March".to_string(),
            4 => "April".to_string(),
            5 => "May".to_string(),
            6 => "June".to_string(),
            7 => "July".to_string(),
            8 => "August".to_string(),
            9 => "September".to_string(),
            10 => "October".to_string(),
            11 => "November".to_string(),
            12 => "December".to_string(),
            _ => "Invalid month".to_string(),
        },
        Language::Spanish => match month_num {
            1 => "Enero".to_string(),
            2 => "Febrero".to_string(),
            3 => "Marzo".to_string(),
            4 => "Abril".to_string(),
            5 => "Mayo".to_string(),
            6 => "Junio".to_string(),
            7 => "Julio".to_string(),
            8 => "Agosto".to_string(),
            9 => "Septiembre".to_string(),
            10 => "Octubre".to_string(),
            11 => "Noviembre".to_string(),
            12 => "Diciembre".to_string(),
            _ => "Mes inválido".to_string(),
        },
    }
}

fn main() {
    let language = prompt_for_language();
    let (prompt_msg, error_msg) = match language {
        Language::English => (
            "Please enter the number of the month:",
            "Invalid input. Please enter a number in the range [1, 12].",
        ),
        Language::Spanish => (
            "Por favor, introduzca el número del mes:",
            "Entrada no válida. Por favor, introduzca un número en el rango [1, 12].",
        ),
    };
    let month_num = prompt_for_month_num(prompt_msg, error_msg);
    let output_msg = match language {
        Language::English => "The name of the month is",
        Language::Spanish => "El nombre del mes es",
    };

    println!("{} {}.", output_msg, get_month_name(month_num, language));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_month_name_handles_english_months() {
        // Test each month in English
        assert_eq!(get_month_name(1, Language::English), "January");
        assert_eq!(get_month_name(2, Language::English), "February");
        assert_eq!(get_month_name(3, Language::English), "March");
        assert_eq!(get_month_name(4, Language::English), "April");
        assert_eq!(get_month_name(5, Language::English), "May");
        assert_eq!(get_month_name(6, Language::English), "June");
        assert_eq!(get_month_name(7, Language::English), "July");
        assert_eq!(get_month_name(8, Language::English), "August");
        assert_eq!(get_month_name(9, Language::English), "September");
        assert_eq!(get_month_name(10, Language::English), "October");
        assert_eq!(get_month_name(11, Language::English), "November");
        assert_eq!(get_month_name(12, Language::English), "December");
    }

    #[test]
    fn get_month_name_handles_spanish_months() {
        // Test each month in Spanish
        assert_eq!(get_month_name(1, Language::Spanish), "Enero");
        assert_eq!(get_month_name(2, Language::Spanish), "Febrero");
        assert_eq!(get_month_name(3, Language::Spanish), "Marzo");
        assert_eq!(get_month_name(4, Language::Spanish), "Abril");
        assert_eq!(get_month_name(5, Language::Spanish), "Mayo");
        assert_eq!(get_month_name(6, Language::Spanish), "Junio");
        assert_eq!(get_month_name(7, Language::Spanish), "Julio");
        assert_eq!(get_month_name(8, Language::Spanish), "Agosto");
        assert_eq!(get_month_name(9, Language::Spanish), "Septiembre");
        assert_eq!(get_month_name(10, Language::Spanish), "Octubre");
        assert_eq!(get_month_name(11, Language::Spanish), "Noviembre");
        assert_eq!(get_month_name(12, Language::Spanish), "Diciembre");
    }

    #[test]
    fn get_month_name_handles_invalid_inputs() {
        // Test out-of-range month numbers
        assert_eq!(get_month_name(0, Language::English), "Invalid month");
        assert_eq!(get_month_name(13, Language::English), "Invalid month");
        assert_eq!(get_month_name(255, Language::English), "Invalid month");

        // Test out-of-range month numbers in Spanish
        assert_eq!(get_month_name(0, Language::Spanish), "Mes inválido");
        assert_eq!(get_month_name(13, Language::Spanish), "Mes inválido");
        assert_eq!(get_month_name(255, Language::Spanish), "Mes inválido");
    }
}
