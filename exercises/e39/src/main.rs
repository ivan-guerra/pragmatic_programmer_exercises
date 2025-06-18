//! # Employee List Sorting Application
//!
//! This module implements a command-line application for managing and organizing
//! employee data with various sorting capabilities.
//!
//! ## Features
//!
//! - **CSV Data Import**: Reads employee records from a CSV file
//! - **Interactive Sorting**: Allows users to select different sort criteria
//! - **Multiple Sort Options**: Sort by name, position, or separation date
//! - **Formatted Output**: Presents employee data in a clean, tabular format
//!
//! The application loads employee data from a CSV file, prompts the user to select
//! a sorting criterion, and displays the sorted results in a formatted table.
use chrono::NaiveDate;
use serde::Deserialize;
use std::path::PathBuf;

enum SortCriterion {
    FirstName,
    LastName,
    Position,
    SeparationDate,
}

#[derive(Debug, Deserialize)]
struct Employee {
    first_name: String,
    last_name: String,
    position: String,
    separation_date: Option<NaiveDate>,
}

fn load_employees(file_path: PathBuf) -> Result<Vec<Employee>, std::io::Error> {
    let mut rdr = csv::Reader::from_path(file_path)?;
    let mut employees = Vec::new();

    for result in rdr.deserialize() {
        let employee: Employee = result?;
        employees.push(employee);
    }
    Ok(employees)
}

fn prompt_for_sort_criterion() -> SortCriterion {
    loop {
        println!("Choose a sort criterion:");
        println!("1. First Name");
        println!("2. Last Name");
        println!("3. Position");
        println!("4. Separation Date");

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        match input.trim() {
            "1" => return SortCriterion::FirstName,
            "2" => return SortCriterion::LastName,
            "3" => return SortCriterion::Position,
            "4" => return SortCriterion::SeparationDate,
            _ => println!("Invalid choice, please try again."),
        }
    }
}

fn print_employee_table(employees: &[Employee]) {
    // Print the header row
    println!("{:<20} | {:<20} | Separation Date", "Name", "Position");

    // Print the separator line under the header
    println!("{:-<20} | {:-<20} | {:-<15}", "", "", "");

    // Print each employee row
    for employee in employees {
        let full_name = format!("{} {}", employee.first_name, employee.last_name);
        let separation_date = employee
            .separation_date
            .map_or("N/A".to_string(), |d| d.to_string());
        println!(
            "{:<20} | {:<20} | {}",
            full_name, employee.position, separation_date
        );
    }
}

fn sort_employees(employees: &mut [Employee], criterion: SortCriterion) {
    match criterion {
        SortCriterion::FirstName => employees.sort_by_key(|e| e.first_name.clone()),
        SortCriterion::LastName => employees.sort_by_key(|e| e.last_name.clone()),
        SortCriterion::Position => employees.sort_by_key(|e| e.position.clone()),
        SortCriterion::SeparationDate => {
            employees.sort_by_key(|e| e.separation_date);
        }
    }
}

fn main() {
    let file_path = PathBuf::from("exercises/e39/inputs/employees.csv");
    match load_employees(file_path) {
        Ok(mut employees) => {
            let sort_criterion = prompt_for_sort_criterion();
            sort_employees(&mut employees, sort_criterion);
            print_employee_table(&employees);
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_employees_by_first_name() {
        let mut employees = vec![
            Employee {
                first_name: "John".to_string(),
                last_name: "Doe".to_string(),
                position: "Developer".to_string(),
                separation_date: None,
            },
            Employee {
                first_name: "Alice".to_string(),
                last_name: "Smith".to_string(),
                position: "Manager".to_string(),
                separation_date: None,
            },
        ];

        sort_employees(&mut employees, SortCriterion::FirstName);
        assert_eq!(employees[0].first_name, "Alice");
        assert_eq!(employees[1].first_name, "John");
    }

    #[test]
    fn sort_employees_by_last_name() {
        let mut employees = vec![
            Employee {
                first_name: "John".to_string(),
                last_name: "Smith".to_string(),
                position: "Developer".to_string(),
                separation_date: None,
            },
            Employee {
                first_name: "Alice".to_string(),
                last_name: "Doe".to_string(),
                position: "Manager".to_string(),
                separation_date: None,
            },
        ];

        sort_employees(&mut employees, SortCriterion::LastName);
        assert_eq!(employees[0].last_name, "Doe");
        assert_eq!(employees[1].last_name, "Smith");
    }

    #[test]
    fn sort_employees_by_position() {
        let mut employees = vec![
            Employee {
                first_name: "John".to_string(),
                last_name: "Smith".to_string(),
                position: "Manager".to_string(),
                separation_date: None,
            },
            Employee {
                first_name: "Alice".to_string(),
                last_name: "Doe".to_string(),
                position: "Developer".to_string(),
                separation_date: None,
            },
        ];

        sort_employees(&mut employees, SortCriterion::Position);
        assert_eq!(employees[0].position, "Developer");
        assert_eq!(employees[1].position, "Manager");
    }

    #[test]
    fn sort_employees_by_separation_date() {
        use chrono::NaiveDate;

        let date1 = NaiveDate::from_ymd_opt(2022, 1, 1).unwrap();
        let date2 = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();

        let mut employees = vec![
            Employee {
                first_name: "John".to_string(),
                last_name: "Smith".to_string(),
                position: "Developer".to_string(),
                separation_date: Some(date2),
            },
            Employee {
                first_name: "Alice".to_string(),
                last_name: "Doe".to_string(),
                position: "Manager".to_string(),
                separation_date: Some(date1),
            },
        ];

        sort_employees(&mut employees, SortCriterion::SeparationDate);
        assert_eq!(employees[0].separation_date, Some(date1));
        assert_eq!(employees[1].separation_date, Some(date2));
    }
}
