//! # Tax Calculator
//!
//! This module implements an interactive tax calculator that applies sales tax based on state.
//!
//! ## Features
//!
//! - **State Recognition**: Identifies all 50 US states by full name or abbreviation
//! - **Tax Calculation**: Applies appropriate sales tax based on the state (Wisconsin: 5.5%)
//! - **User Interaction**: Prompts for order amount and state with input validation
use once_cell::sync::Lazy;
use std::collections::HashSet;
use std::io::Write;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct State {
    name: String,
    abbreviation: String,
}

impl PartialEq<&str> for State {
    fn eq(&self, other: &&str) -> bool {
        self.name.to_lowercase() == other.to_lowercase()
            || self.abbreviation.to_lowercase() == other.to_lowercase()
    }
}

impl PartialEq<String> for State {
    fn eq(&self, other: &String) -> bool {
        self.name.to_lowercase() == other.to_lowercase()
            || self.abbreviation.to_lowercase() == other.to_lowercase()
    }
}

static STATE_NAMES: Lazy<HashSet<State>> = Lazy::new(|| {
    let mut set = HashSet::new();
    // Add all 50 US states with their abbreviations
    set.insert(State {
        name: String::from("Alabama"),
        abbreviation: String::from("AL"),
    });
    set.insert(State {
        name: String::from("Alaska"),
        abbreviation: String::from("AK"),
    });
    set.insert(State {
        name: String::from("Arizona"),
        abbreviation: String::from("AZ"),
    });
    set.insert(State {
        name: String::from("Arkansas"),
        abbreviation: String::from("AR"),
    });
    set.insert(State {
        name: String::from("California"),
        abbreviation: String::from("CA"),
    });
    set.insert(State {
        name: String::from("Colorado"),
        abbreviation: String::from("CO"),
    });
    set.insert(State {
        name: String::from("Connecticut"),
        abbreviation: String::from("CT"),
    });
    set.insert(State {
        name: String::from("Delaware"),
        abbreviation: String::from("DE"),
    });
    set.insert(State {
        name: String::from("Florida"),
        abbreviation: String::from("FL"),
    });
    set.insert(State {
        name: String::from("Georgia"),
        abbreviation: String::from("GA"),
    });
    set.insert(State {
        name: String::from("Hawaii"),
        abbreviation: String::from("HI"),
    });
    set.insert(State {
        name: String::from("Idaho"),
        abbreviation: String::from("ID"),
    });
    set.insert(State {
        name: String::from("Illinois"),
        abbreviation: String::from("IL"),
    });
    set.insert(State {
        name: String::from("Indiana"),
        abbreviation: String::from("IN"),
    });
    set.insert(State {
        name: String::from("Iowa"),
        abbreviation: String::from("IA"),
    });
    set.insert(State {
        name: String::from("Kansas"),
        abbreviation: String::from("KS"),
    });
    set.insert(State {
        name: String::from("Kentucky"),
        abbreviation: String::from("KY"),
    });
    set.insert(State {
        name: String::from("Louisiana"),
        abbreviation: String::from("LA"),
    });
    set.insert(State {
        name: String::from("Maine"),
        abbreviation: String::from("ME"),
    });
    set.insert(State {
        name: String::from("Maryland"),
        abbreviation: String::from("MD"),
    });
    set.insert(State {
        name: String::from("Massachusetts"),
        abbreviation: String::from("MA"),
    });
    set.insert(State {
        name: String::from("Michigan"),
        abbreviation: String::from("MI"),
    });
    set.insert(State {
        name: String::from("Minnesota"),
        abbreviation: String::from("MN"),
    });
    set.insert(State {
        name: String::from("Mississippi"),
        abbreviation: String::from("MS"),
    });
    set.insert(State {
        name: String::from("Missouri"),
        abbreviation: String::from("MO"),
    });
    set.insert(State {
        name: String::from("Montana"),
        abbreviation: String::from("MT"),
    });
    set.insert(State {
        name: String::from("Nebraska"),
        abbreviation: String::from("NE"),
    });
    set.insert(State {
        name: String::from("Nevada"),
        abbreviation: String::from("NV"),
    });
    set.insert(State {
        name: String::from("New Hampshire"),
        abbreviation: String::from("NH"),
    });
    set.insert(State {
        name: String::from("New Jersey"),
        abbreviation: String::from("NJ"),
    });
    set.insert(State {
        name: String::from("New Mexico"),
        abbreviation: String::from("NM"),
    });
    set.insert(State {
        name: String::from("New York"),
        abbreviation: String::from("NY"),
    });
    set.insert(State {
        name: String::from("North Carolina"),
        abbreviation: String::from("NC"),
    });
    set.insert(State {
        name: String::from("North Dakota"),
        abbreviation: String::from("ND"),
    });
    set.insert(State {
        name: String::from("Ohio"),
        abbreviation: String::from("OH"),
    });
    set.insert(State {
        name: String::from("Oklahoma"),
        abbreviation: String::from("OK"),
    });
    set.insert(State {
        name: String::from("Oregon"),
        abbreviation: String::from("OR"),
    });
    set.insert(State {
        name: String::from("Pennsylvania"),
        abbreviation: String::from("PA"),
    });
    set.insert(State {
        name: String::from("Rhode Island"),
        abbreviation: String::from("RI"),
    });
    set.insert(State {
        name: String::from("South Carolina"),
        abbreviation: String::from("SC"),
    });
    set.insert(State {
        name: String::from("South Dakota"),
        abbreviation: String::from("SD"),
    });
    set.insert(State {
        name: String::from("Tennessee"),
        abbreviation: String::from("TN"),
    });
    set.insert(State {
        name: String::from("Texas"),
        abbreviation: String::from("TX"),
    });
    set.insert(State {
        name: String::from("Utah"),
        abbreviation: String::from("UT"),
    });
    set.insert(State {
        name: String::from("Vermont"),
        abbreviation: String::from("VT"),
    });
    set.insert(State {
        name: String::from("Virginia"),
        abbreviation: String::from("VA"),
    });
    set.insert(State {
        name: String::from("Washington"),
        abbreviation: String::from("WA"),
    });
    set.insert(State {
        name: String::from("West Virginia"),
        abbreviation: String::from("WV"),
    });
    set.insert(State {
        name: String::from("Wisconsin"),
        abbreviation: String::from("WI"),
    });
    set.insert(State {
        name: String::from("Wyoming"),
        abbreviation: String::from("WY"),
    });
    set
});

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
            if value < 0.0 {
                println!("Please enter a non-negative number.");
                continue;
            }
            return value;
        } else {
            println!("Invalid input. Please enter a valid number.");
        }
    }
}

fn prompt_for_state() -> State {
    loop {
        print!("What is the state? ");
        let mut input = String::new();
        if let Err(e) = std::io::stdout().flush() {
            eprintln!("Error: {}", e);
            continue;
        }

        if let Err(e) = std::io::stdin().read_line(&mut input) {
            eprintln!("Error: {}", e);
            continue;
        }

        let input = input.trim().to_lowercase();
        for state in STATE_NAMES.iter() {
            if state.name.to_lowercase() == input || state.abbreviation.to_lowercase() == input {
                return state.clone();
            }
        }
        println!("Invalid state name or abbreviation. Please try again.");
    }
}

fn calculate_total(order_amount: f64, state: &State) -> f64 {
    let tax_rate = match state.abbreviation.as_str() {
        "WI" => 0.055, // Wisconsin
        _ => 0.0,      // Default tax rate for other states
    };
    let tax_amount = order_amount * tax_rate;
    order_amount + tax_amount
}

fn main() {
    let order_amount = prompt_for_float("What is the order amount?");
    let state = prompt_for_state();
    println!(
        "The total is ${:.2}.",
        calculate_total(order_amount, &state)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_total_applies_wisconsin_tax() {
        let wi_state = State {
            name: String::from("Wisconsin"),
            abbreviation: String::from("WI"),
        };
        assert_eq!(calculate_total(100.0, &wi_state), 105.5); // $100 with 5.5% tax
        assert_eq!(calculate_total(50.0, &wi_state), 52.75); // $50 with 5.5% tax
    }

    #[test]
    fn calculate_total_no_tax_for_other_states() {
        let ca_state = State {
            name: String::from("California"),
            abbreviation: String::from("CA"),
        };
        assert_eq!(calculate_total(100.0, &ca_state), 100.0); // No tax for California

        let tx_state = State {
            name: String::from("Texas"),
            abbreviation: String::from("TX"),
        };
        assert_eq!(calculate_total(75.0, &tx_state), 75.0); // No tax for Texas
    }

    #[test]
    fn calculate_total_handles_zero_values() {
        let wi_state = State {
            name: String::from("Wisconsin"),
            abbreviation: String::from("WI"),
        };
        assert_eq!(calculate_total(0.0, &wi_state), 0.0); // $0 order should result in $0 total

        let ny_state = State {
            name: String::from("New York"),
            abbreviation: String::from("NY"),
        };
        assert_eq!(calculate_total(0.0, &ny_state), 0.0); // $0 order should result in $0 total
    }
}
