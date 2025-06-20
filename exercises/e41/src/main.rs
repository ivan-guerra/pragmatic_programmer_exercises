//! # Name Sorter
//!
//! This module provides functionality for reading, sorting, and writing names from text files.
//! It supports processing name data with the following features:
//!
//! - **File I/O**: Reads from and writes to text files in CSV format
//! - **Name Parsing**: Processes comma-separated name entries (last name, first name)
//! - **Case-Insensitive Sorting**: Sorts names alphabetically by last name, then by first name
//! - **Structured Data**: Maintains first and last name as separate fields
//!
//! The application reads names from a specified input file, sorts them alphabetically
//! in a case-insensitive manner (primary sort by last name, secondary by first name),
//! and writes the sorted list to an output file.
use std::io::{BufRead, Write};
use std::path::PathBuf;

struct Name {
    first_name: String,
    last_name: String,
}

fn read_names(file_path: &PathBuf) -> Result<Vec<Name>, std::io::Error> {
    let file = std::fs::File::open(file_path)?;
    let reader = std::io::BufReader::new(file);
    let mut names = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() == 2 {
            names.push(Name {
                first_name: parts[1].trim().to_string(),
                last_name: parts[0].trim().to_string(),
            });
        }
    }
    Ok(names)
}

fn write_names(file_path: &PathBuf, names: &[Name]) -> Result<(), std::io::Error> {
    let mut file = std::fs::File::create(file_path)?;
    for name in names {
        writeln!(file, "{}, {}", name.last_name, name.first_name)?;
    }
    Ok(())
}

fn sort_names(names: &mut [Name]) {
    names.sort_by(|a, b| {
        a.last_name
            .to_lowercase()
            .cmp(&b.last_name.to_lowercase())
            .then(
                a.first_name
                    .to_lowercase()
                    .cmp(&b.first_name.to_lowercase()),
            )
    });
}

fn main() {
    let file_path = PathBuf::from("exercises/e41/data/names.txt");

    match read_names(&file_path) {
        Ok(mut names) => {
            if names.is_empty() {
                println!("No names found in the file.");
                return;
            }
            sort_names(&mut names);
            let output_file_path = PathBuf::from("exercises/e41/data/sorted_names.txt");
            if let Err(e) = write_names(&output_file_path, &names) {
                eprintln!("Error writing sorted names to file: {}", e);
            } else {
                println!(
                    "Names sorted and written to {:?} successfully.",
                    output_file_path
                );
            }
        }
        Err(e) => eprintln!("Error reading names from file: {}", e),
    }
}
