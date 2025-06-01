//! # Currency Converter
//!
//! This module implements an interactive currency conversion application that
//! fetches real-time exchange rates and converts USD to various foreign currencies.
//!
//! ## Features
//!
//! - **Live Exchange Rates**: Fetches current rates from a web API
//! - **Multiple Currencies**: Supports conversion to numerous international currencies
//! - **Tabular Display**: Shows available currency options in a formatted table
//! - **User Interaction**: Provides clear prompts for country selection and amount input
//! - **Error Handling**: Gracefully handles API connection issues and invalid inputs
use reqwest::blocking::get;
use serde::Deserialize;
use std::collections::HashMap;
use std::io::Write;

type CountryCode = String;
type ExchangeRate = f64;
type USDExchangeRates = HashMap<CountryCode, ExchangeRate>;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct ExchangeRateResponse {
    success: bool,
    terms: String,
    privacy: String,
    timestamp: u64,
    source: String,
    quotes: HashMap<String, f64>,
}

fn get_exchange_rates(api_key: &str) -> Result<USDExchangeRates, Box<dyn std::error::Error>> {
    let url = format!("https://api.exchangerate.host/live?access_key={}", api_key);
    let response = get(url)?.json::<ExchangeRateResponse>()?;

    if response.success {
        let mut rates = USDExchangeRates::new();
        for (key, value) in response.quotes {
            if let Some(stripped_key) = key.strip_prefix("USD") {
                // Only include rates for USD to other currencies
                let country_code = stripped_key.to_string(); // Extract country code
                rates.insert(country_code, value);
            } else {
                eprintln!("Skipping non-USD rate: {}", key);
            }
        }
        Ok(rates)
    } else {
        Err("Failed to fetch exchange rates".into())
    }
}

fn prompt_for_country_code(countries: Vec<CountryCode>) -> CountryCode {
    loop {
        println!("Available country codes:");
        // Calculate column width based on the longest country code plus padding
        let max_width = countries.iter().map(|c| c.len()).max().unwrap_or(0) + 2;
        let cols = 80 / max_width; // Number of columns that fit in 80 chars

        // Print countries in columns
        for (i, country) in countries.iter().enumerate() {
            print!("{:width$}", country, width = max_width);
            if (i + 1) % cols == 0 || i == countries.len() - 1 {
                println!(); // New line after filling columns or at the end
            }
        }
        print!("Enter the country code to convert from USD: ");
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
        if countries.contains(&input) {
            return input;
        } else {
            println!("Invalid country code. Please try again.");
        }
    }
}

fn prompt_for_currency(prompt: &str) -> f64 {
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

fn main() {
    // Hardcoding an API key for exchangerate.host
    let api_key = "eddb40086e959186440a6ed499d04de1";
    let exchange_rates = match get_exchange_rates(api_key) {
        Ok(rates) => rates,
        Err(e) => {
            eprintln!("Error fetching exchange rates: {}", e);
            return;
        }
    };
    let country_codes: Vec<CountryCode> = {
        let mut keys = exchange_rates.keys().cloned().collect::<Vec<_>>();
        keys.sort();
        keys
    };
    let country_code = prompt_for_country_code(country_codes);
    let usd_amount = prompt_for_currency("Enter the amount in USD to convert:");

    println!(
        "{:.2} USD at an exchange rate of {} will give you {:.2} {}.",
        usd_amount,
        exchange_rates[&country_code],
        usd_amount * exchange_rates[&country_code],
        country_code
    );
}
