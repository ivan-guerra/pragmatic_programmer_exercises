//! # Tax Calculator with County-based Taxation
//!
//! This module implements an interactive tax calculation system that applies different tax rates
//! based on state and county information.
//!
//! ## Features
//!
//! - **State & County Support**: Handles specific tax rates for different states and counties
//! - **Input Validation**: Ensures valid states and counties through interactive prompting
//! - **Case-Insensitive Matching**: Recognizes state names and abbreviations regardless of case
//! - **Complete US Coverage**: Includes all 50 US states with their official abbreviations
//! - **Precise Calculation**: Applies appropriate tax rates based on geographic location
//!
//! ## Tax Structure
//!
//! - **Wisconsin**: Base state rate of 0% with county-specific rates:
//!   - Eau Claire County: 0.5%
//!   - Dunn County: 0.4%
//! - **Illinois**: Flat state rate of 8% with no county-specific adjustments
//! - **Other States**: No tax applied
use once_cell::sync::Lazy;
use std::collections::{HashMap, HashSet};
use std::io::Write;

type CountyName = String;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct StateName {
    full_name: String,
    abbreviation: String,
}

struct StateTax {
    tax_rate: f64,
    counties: HashMap<CountyName, f64>,
}

static TAXABLE_STATES: Lazy<HashMap<StateName, StateTax>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert(
        StateName {
            full_name: "Wisconsin".to_string(),
            abbreviation: "WI".to_string(),
        },
        StateTax {
            tax_rate: 0.0,
            counties: HashMap::from([
                ("Eau Claire".to_string(), 0.005),
                ("Dunn".to_string(), 0.004),
            ]),
        },
    );
    m.insert(
        StateName {
            full_name: "Illinois".to_string(),
            abbreviation: "IL".to_string(),
        },
        StateTax {
            tax_rate: 0.08,
            counties: HashMap::new(),
        },
    );
    m
});

static VALID_STATE_NAMES: Lazy<HashSet<StateName>> = Lazy::new(|| {
    let mut set = HashSet::new();

    // Add all 50 US states with their abbreviations
    set.insert(StateName {
        full_name: "Alabama".to_string(),
        abbreviation: "AL".to_string(),
    });
    set.insert(StateName {
        full_name: "Alaska".to_string(),
        abbreviation: "AK".to_string(),
    });
    set.insert(StateName {
        full_name: "Arizona".to_string(),
        abbreviation: "AZ".to_string(),
    });
    set.insert(StateName {
        full_name: "Arkansas".to_string(),
        abbreviation: "AR".to_string(),
    });
    set.insert(StateName {
        full_name: "California".to_string(),
        abbreviation: "CA".to_string(),
    });
    set.insert(StateName {
        full_name: "Colorado".to_string(),
        abbreviation: "CO".to_string(),
    });
    set.insert(StateName {
        full_name: "Connecticut".to_string(),
        abbreviation: "CT".to_string(),
    });
    set.insert(StateName {
        full_name: "Delaware".to_string(),
        abbreviation: "DE".to_string(),
    });
    set.insert(StateName {
        full_name: "Florida".to_string(),
        abbreviation: "FL".to_string(),
    });
    set.insert(StateName {
        full_name: "Georgia".to_string(),
        abbreviation: "GA".to_string(),
    });
    set.insert(StateName {
        full_name: "Hawaii".to_string(),
        abbreviation: "HI".to_string(),
    });
    set.insert(StateName {
        full_name: "Idaho".to_string(),
        abbreviation: "ID".to_string(),
    });
    set.insert(StateName {
        full_name: "Illinois".to_string(),
        abbreviation: "IL".to_string(),
    });
    set.insert(StateName {
        full_name: "Indiana".to_string(),
        abbreviation: "IN".to_string(),
    });
    set.insert(StateName {
        full_name: "Iowa".to_string(),
        abbreviation: "IA".to_string(),
    });
    set.insert(StateName {
        full_name: "Kansas".to_string(),
        abbreviation: "KS".to_string(),
    });
    set.insert(StateName {
        full_name: "Kentucky".to_string(),
        abbreviation: "KY".to_string(),
    });
    set.insert(StateName {
        full_name: "Louisiana".to_string(),
        abbreviation: "LA".to_string(),
    });
    set.insert(StateName {
        full_name: "Maine".to_string(),
        abbreviation: "ME".to_string(),
    });
    set.insert(StateName {
        full_name: "Maryland".to_string(),
        abbreviation: "MD".to_string(),
    });
    set.insert(StateName {
        full_name: "Massachusetts".to_string(),
        abbreviation: "MA".to_string(),
    });
    set.insert(StateName {
        full_name: "Michigan".to_string(),
        abbreviation: "MI".to_string(),
    });
    set.insert(StateName {
        full_name: "Minnesota".to_string(),
        abbreviation: "MN".to_string(),
    });
    set.insert(StateName {
        full_name: "Mississippi".to_string(),
        abbreviation: "MS".to_string(),
    });
    set.insert(StateName {
        full_name: "Missouri".to_string(),
        abbreviation: "MO".to_string(),
    });
    set.insert(StateName {
        full_name: "Montana".to_string(),
        abbreviation: "MT".to_string(),
    });
    set.insert(StateName {
        full_name: "Nebraska".to_string(),
        abbreviation: "NE".to_string(),
    });
    set.insert(StateName {
        full_name: "Nevada".to_string(),
        abbreviation: "NV".to_string(),
    });
    set.insert(StateName {
        full_name: "New Hampshire".to_string(),
        abbreviation: "NH".to_string(),
    });
    set.insert(StateName {
        full_name: "New Jersey".to_string(),
        abbreviation: "NJ".to_string(),
    });
    set.insert(StateName {
        full_name: "New Mexico".to_string(),
        abbreviation: "NM".to_string(),
    });
    set.insert(StateName {
        full_name: "New York".to_string(),
        abbreviation: "NY".to_string(),
    });
    set.insert(StateName {
        full_name: "North Carolina".to_string(),
        abbreviation: "NC".to_string(),
    });
    set.insert(StateName {
        full_name: "North Dakota".to_string(),
        abbreviation: "ND".to_string(),
    });
    set.insert(StateName {
        full_name: "Ohio".to_string(),
        abbreviation: "OH".to_string(),
    });
    set.insert(StateName {
        full_name: "Oklahoma".to_string(),
        abbreviation: "OK".to_string(),
    });
    set.insert(StateName {
        full_name: "Oregon".to_string(),
        abbreviation: "OR".to_string(),
    });
    set.insert(StateName {
        full_name: "Pennsylvania".to_string(),
        abbreviation: "PA".to_string(),
    });
    set.insert(StateName {
        full_name: "Rhode Island".to_string(),
        abbreviation: "RI".to_string(),
    });
    set.insert(StateName {
        full_name: "South Carolina".to_string(),
        abbreviation: "SC".to_string(),
    });
    set.insert(StateName {
        full_name: "South Dakota".to_string(),
        abbreviation: "SD".to_string(),
    });
    set.insert(StateName {
        full_name: "Tennessee".to_string(),
        abbreviation: "TN".to_string(),
    });
    set.insert(StateName {
        full_name: "Texas".to_string(),
        abbreviation: "TX".to_string(),
    });
    set.insert(StateName {
        full_name: "Utah".to_string(),
        abbreviation: "UT".to_string(),
    });
    set.insert(StateName {
        full_name: "Vermont".to_string(),
        abbreviation: "VT".to_string(),
    });
    set.insert(StateName {
        full_name: "Virginia".to_string(),
        abbreviation: "VA".to_string(),
    });
    set.insert(StateName {
        full_name: "Washington".to_string(),
        abbreviation: "WA".to_string(),
    });
    set.insert(StateName {
        full_name: "West Virginia".to_string(),
        abbreviation: "WV".to_string(),
    });
    set.insert(StateName {
        full_name: "Wisconsin".to_string(),
        abbreviation: "WI".to_string(),
    });
    set.insert(StateName {
        full_name: "Wyoming".to_string(),
        abbreviation: "WY".to_string(),
    });

    set
});

fn prompt_for_county_name() -> CountyName {
    loop {
        print!("Enter the county name: ");
        let mut input = String::new();
        if let Err(e) = std::io::stdout().flush() {
            eprintln!("Error: {}", e);
            continue;
        }

        if let Err(e) = std::io::stdin().read_line(&mut input) {
            eprintln!("Error: {}", e);
            continue;
        }

        let input = input.trim();

        // Check if the county exists in any state's tax records
        for (_, state_tax) in TAXABLE_STATES.iter() {
            for county in state_tax.counties.keys() {
                if county.eq_ignore_ascii_case(input) {
                    return county.clone(); // Return the correctly cased county name
                }
            }
        }

        println!("Invalid county name. Please try again.");
    }
}

fn prompt_for_state_name() -> StateName {
    loop {
        print!("Enter the state name or abbreviation: ");
        let mut input = String::new();
        if let Err(e) = std::io::stdout().flush() {
            eprintln!("Error: {}", e);
            continue;
        }

        if let Err(e) = std::io::stdin().read_line(&mut input) {
            eprintln!("Error: {}", e);
            continue;
        }

        let input = input.trim();
        for state in VALID_STATE_NAMES.iter() {
            if state.full_name.eq_ignore_ascii_case(input)
                || state.abbreviation.eq_ignore_ascii_case(input)
            {
                return state.clone();
            }
        }

        println!("Invalid state name or abbreviation. Please try again.");
    }
}

fn prompt_for_amount() -> f64 {
    loop {
        print!("Enter the amount: ");
        let mut input = String::new();
        if let Err(e) = std::io::stdout().flush() {
            eprintln!("Error: {}", e);
            continue;
        }

        if let Err(e) = std::io::stdin().read_line(&mut input) {
            eprintln!("Error: {}", e);
            continue;
        }

        match input.trim().parse::<f64>() {
            Ok(value) => {
                if value >= 0.0 {
                    return value;
                } else {
                    println!("Please enter a non-negative dollar amount.");
                }
            }
            Err(_) => println!("Invalid input. Please enter a valid dollar amount."),
        }
    }
}

fn calculate_tax(amount: f64, state_name: &StateName) -> f64 {
    let wisconsin = StateName {
        full_name: "Wisconsin".to_string(),
        abbreviation: "WI".to_string(),
    };
    let illinois = StateName {
        full_name: "Illinois".to_string(),
        abbreviation: "IL".to_string(),
    };

    match state_name {
        name if name == &wisconsin => {
            let state_tax = TAXABLE_STATES.get(&wisconsin).unwrap();
            let county_name = prompt_for_county_name();
            let county_tax = state_tax.counties.get(&county_name).cloned().unwrap_or(0.0);
            amount * (state_tax.tax_rate + county_tax)
        }
        name if name == &illinois => {
            let state_tax = TAXABLE_STATES.get(&illinois).unwrap();
            amount * state_tax.tax_rate
        }
        _ => 0.0,
    }
}

fn main() {
    let amount = prompt_for_amount();
    let state_name = prompt_for_state_name();

    if TAXABLE_STATES.contains_key(&state_name) {
        let tax = calculate_tax(amount, &state_name);
        let total = amount + tax;

        println!("The tax is ${:.2}.", tax);
        println!("The total amount is ${:.2}.", total);
    } else {
        println!("The total amount is ${:.2}.", amount);
    }
}
