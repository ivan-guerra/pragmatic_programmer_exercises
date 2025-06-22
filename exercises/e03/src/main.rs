//! # Famous Quotes Collection
//!
//! This module implements a simple program that stores and displays famous quotes
//! from notable historical figures.
//!
//! ## Features
//!
//! - **Static Collection**: Uses `once_cell::Lazy` to initialize quotes only when accessed
//! - **Organized Data**: Stores multiple quotes per author in a HashMap
//! - **Formatted Output**: Displays quotes with author attribution
use once_cell::sync::Lazy;
use std::collections::HashMap;

static QUOTES: Lazy<HashMap<&str, Vec<&str>>> = Lazy::new(|| {
    let mut m: HashMap<&str, Vec<&str>> = HashMap::new();
    m.insert(
        "Albert Einstein",
        vec![
            "Life is like riding a bicycle. To keep your balance, you must keep moving.",
            "Imagination is more important than knowledge.",
            "A person who never made a mistake never tried anything new.",
        ],
    );
    m.insert("John F. Kennedy", vec![
        "Ask not what your country can do for you, ask what you can do for your country.",
        "Change is the law of life. And those who look only to the past or present are certain to miss the future.",
        "The time to repair the roof is when the sun is shining.",
    ]);
    m.insert(
        "Dennis Ritchie",
        vec![
            "The only way to learn a new programming language is by writing programs in it.",
            "C is quirky, flawed, and an enormous success.",
            "The best way to predict the future is to invent it.",
        ],
    );
    m
});

fn main() {
    for (author, quotes) in QUOTES.iter() {
        for quote in quotes {
            println!("{} says, \"{}\"", author, quote);
        }
        println!();
    }
}
