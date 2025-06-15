//! # Employee Management System
//!
//! This module implements a command-line application for managing employee records
//! by allowing users to remove employees from a list stored in a text file.
//!
//! ## Features
//!
//! - **File I/O**: Reads from and writes to a text file to maintain employee records
//! - **Interactive Interface**: Prompts users to enter names for removal
//! - **Validation**: Verifies if employees exist before attempting removal
//! - **Persistence**: Saves the updated list back to the file when complete
//!
//! The application loads an existing employee list, allows the user to remove employees
//! interactively, and then saves the updated list when finished.
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, Write};
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

fn read_names_from_file(file_path: PathBuf) -> Result<HashSet<String>, std::io::Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut names = HashSet::new();

    for line in reader.lines() {
        let name = line?;
        if !name.trim().is_empty() {
            names.insert(name.trim().to_string());
        }
    }
    Ok(names)
}

fn write_names_to_file(file_path: PathBuf, names: &HashSet<String>) -> Result<(), std::io::Error> {
    let mut file = File::create(file_path)?;
    names.iter().for_each(|name| {
        writeln!(file, "{}", name).unwrap();
    });
    Ok(())
}

fn prompt_for_name() -> Option<String> {
    print!("Enter an employee name to remove (or blank to save and exit): ");
    io::stdout().flush().unwrap();

    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();
    if name.trim().is_empty() {
        None
    } else {
        Some(name.trim().to_string())
    }
}

fn print_employees(names: &HashSet<String>) {
    if names.is_empty() {
        println!("No employees found.");
    } else {
        println!("There are {} employees:", names.len());
        names.iter().for_each(|name| println!("{}", name));
    }
}

fn main() {
    let file_path = PathBuf::from("exercises/e34/inputs/employees.txt");

    if let Ok(mut names) = read_names_from_file(file_path.clone()) {
        if names.is_empty() {
            println!("No employees found in the file. Please add some names first.");
            return;
        }
        print_employees(&names);

        while let Some(name) = prompt_for_name() {
            if names.contains(&name) {
                names.remove(&name);
            } else {
                println!("Employee '{}' not found in the list.", name);
            }

            if names.is_empty() {
                break;
            }

            print_employees(&names);
        }

        if let Err(e) = write_names_to_file(file_path, &names) {
            eprintln!("Error writing to file: {}", e);
        } else {
            println!("Updated employee list saved successfully.");
        }
    } else if let Err(e) = read_names_from_file(file_path) {
        eprintln!("Error reading from file: {}", e);
    }
}
