//! # Secure Login System
//!
//! This module implements a command-line authentication system that uses bcrypt
//! hashed passwords to securely verify user credentials.
//!
//! ## Features
//!
//! - **Secure Password Storage**: Uses bcrypt hashing to protect user passwords
//! - **Password Masking**: Hides password input during entry
//!
//! The system verifies user credentials against a pre-defined set of bcrypt-hashed
//! passwords stored in memory.
use bcrypt::verify;
use once_cell::sync::Lazy;
use rpassword::read_password;
use std::collections::HashMap;

static USERS: Lazy<HashMap<String, String>> = Lazy::new(|| {
    let mut m = HashMap::new();
    // password: hello
    m.insert(
        "alice".to_string(),
        "$2b$12$2jP33spRZpG0cuc/ZqtHs.zkIFnk5nvlkYJXm71Aoa1GXGcOl39z2".to_string(),
    );
    // password: world
    m.insert(
        "bob".to_string(),
        "$2b$12$oeKean9q91hYXzHNBNMah.PKgS3.HMau4sse2UgzaS1bgvY5aYJwK".to_string(),
    );
    // password: qwerty
    m.insert(
        "tom".to_string(),
        "$2b$12$MKPGObt5PmpFPlj5tEjKfeiQvRW5Jo0pmcNdWGg5iTBoKpkXvSfxm".to_string(),
    );
    m
});

fn get_username() -> String {
    println!("Enter your username:");
    let mut username = String::new();
    std::io::stdin()
        .read_line(&mut username)
        .expect("Failed to read line");
    username.trim().to_string()
}

fn get_password() -> String {
    println!("Enter your password:");
    read_password().expect("Failed to read password")
}

fn main() {
    println!("Welcome to the secure login system!");

    let username = get_username();
    if !USERS.contains_key(&username) {
        println!("User not found: {}", username);
        return;
    }
    let password = get_password();

    let hashed_password = USERS.get(&username).unwrap();
    match verify(&password, hashed_password) {
        Ok(true) => {
            println!("Login successful for user: {}", username);
        }
        Ok(false) => {
            println!("Invalid password for user: {}", username);
        }
        Err(e) => {
            println!("Error verifying password: {}", e);
        }
    }
}
