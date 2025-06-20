//! # Employee Records Search System
//!
//! This module implements a command-line application for searching employee records
//! using various criteria to filter and locate specific employee information.
//!
//! ## Features
//!
//! - **CSV Data Import**: Reads employee records from a CSV file
//! - **Multiple Search Criteria**: Supports searching by name, position, or separation date
//! - **Flexible Matching**: Uses case-insensitive partial matching for text searches
//! - **Date-based Filtering**: Finds employees who left within the last six months
//! - **Formatted Output**: Displays results in a clear, tabular format
//!
//! The application loads employee data, prompts the user to select a search criterion,
//! accepts search parameters, and displays matching records in a formatted table.
use chrono::{Local, Months, NaiveDate};
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
struct Employee {
    first_name: String,
    last_name: String,
    position: String,
    separation_date: Option<NaiveDate>,
}

fn search_by_name<'a>(employees: &'a [Employee], name: &str) -> Vec<&'a Employee> {
    employees
        .iter()
        .filter(|e| {
            e.first_name.to_lowercase().contains(&name.to_lowercase())
                || e.last_name.to_lowercase().contains(&name.to_lowercase())
        })
        .collect()
}

fn search_by_position<'a>(employees: &'a [Employee], position: &str) -> Vec<&'a Employee> {
    employees
        .iter()
        .filter(|e| e.position.to_lowercase().contains(&position.to_lowercase()))
        .collect()
}

fn search_by_separation_date(employees: &[Employee]) -> Vec<&Employee> {
    let today = Local::now().date_naive();
    let six_months_ago = today
        .checked_sub_months(Months::new(6))
        .expect("Date underflowed");

    employees
        .iter()
        .filter(|e| {
            if let Some(date) = e.separation_date {
                date > six_months_ago
            } else {
                false
            }
        })
        .collect()
}

enum SearchCriterion {
    Name,
    Position,
    SeparationDate,
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

fn prompt_for_search_criterion() -> SearchCriterion {
    loop {
        println!("Choose a search criterion:");
        println!("1. Name");
        println!("2. Position");
        println!("3. Separation Date");

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        match input.trim() {
            "1" => return SearchCriterion::Name,
            "2" => return SearchCriterion::Position,
            "3" => return SearchCriterion::SeparationDate,
            _ => println!("Invalid choice, please try again."),
        }
    }
}

fn print_employee_table(employees: &[&Employee]) {
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

fn main() {
    let file_path = PathBuf::from("exercises/e39/inputs/employees.csv");
    match load_employees(file_path) {
        Ok(employees) => {
            let search_criterion = prompt_for_search_criterion();
            match search_criterion {
                SearchCriterion::Name => {
                    println!("Enter a name to search for:");
                    let mut name = String::new();
                    std::io::stdin()
                        .read_line(&mut name)
                        .expect("Failed to read line");
                    let results = search_by_name(&employees, name.trim());
                    if results.is_empty() {
                        println!("No employees found with that name.");
                    } else {
                        print_employee_table(&results);
                    }
                }
                SearchCriterion::Position => {
                    println!("Enter a position to search for:");
                    let mut position = String::new();
                    std::io::stdin()
                        .read_line(&mut position)
                        .expect("Failed to read line");
                    let results = search_by_position(&employees, position.trim());
                    if results.is_empty() {
                        println!("No employees found with that position.");
                    } else {
                        print_employee_table(&results);
                    }
                }
                SearchCriterion::SeparationDate => {
                    let results = search_by_separation_date(&employees);
                    if results.is_empty() {
                        println!("No employees found with a separation date in the last 6 months.");
                    } else {
                        print_employee_table(&results);
                    }
                }
            }
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_employees_by_name() {
        let employees = vec![
            Employee {
                first_name: "John".to_string(),
                last_name: "Doe".to_string(),
                position: "Developer".to_string(),
                separation_date: None,
            },
            Employee {
                first_name: "Jane".to_string(),
                last_name: "Smith".to_string(),
                position: "Manager".to_string(),
                separation_date: None,
            },
            Employee {
                first_name: "Bob".to_string(),
                last_name: "Johnson".to_string(),
                position: "Developer".to_string(),
                separation_date: None,
            },
            Employee {
                first_name: "John".to_string(),
                last_name: "Smith".to_string(),
                position: "Designer".to_string(),
                separation_date: None,
            },
            Employee {
                first_name: "Alice".to_string(),
                last_name: "Johnson".to_string(),
                position: "Tester".to_string(),
                separation_date: None,
            },
        ];

        // Single match by first name
        let results = search_by_name(&employees, "jane");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].first_name, "Jane");

        // Single match by last name
        let results = search_by_name(&employees, "doe");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].last_name, "Doe");

        // Multiple matches by first name
        let results = search_by_name(&employees, "john");
        assert_eq!(results.len(), 4);
        assert!(results
            .iter()
            .all(|e| e.first_name.contains("John") || e.last_name.contains("John")));

        // Multiple matches by last name
        let results = search_by_name(&employees, "son");
        assert_eq!(results.len(), 2);
        assert!(results.iter().all(|e| e.last_name.contains("son")));

        // Multiple matches across first and last names
        let results = search_by_name(&employees, "smith");
        assert_eq!(results.len(), 2);

        // No matches
        let results = search_by_name(&employees, "Xavier");
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn search_employees_by_position() {
        let employees = vec![
            Employee {
                first_name: "John".to_string(),
                last_name: "Doe".to_string(),
                position: "Developer".to_string(),
                separation_date: None,
            },
            Employee {
                first_name: "Jane".to_string(),
                last_name: "Smith".to_string(),
                position: "Manager".to_string(),
                separation_date: None,
            },
            Employee {
                first_name: "Bob".to_string(),
                last_name: "Johnson".to_string(),
                position: "Developer".to_string(),
                separation_date: None,
            },
            Employee {
                first_name: "Alice".to_string(),
                last_name: "Brown".to_string(),
                position: "Senior Developer".to_string(),
                separation_date: None,
            },
            Employee {
                first_name: "Chris".to_string(),
                last_name: "Wilson".to_string(),
                position: "Team Manager".to_string(),
                separation_date: None,
            },
        ];

        // Multiple exact matches
        let results = search_by_position(&employees, "developer");
        assert_eq!(results.len(), 3);
        assert!(results.iter().all(|e| e.position.contains("Developer")));

        // Multiple partial matches
        let results = search_by_position(&employees, "dev");
        assert_eq!(results.len(), 3);
        assert!(results
            .iter()
            .all(|e| e.position.to_lowercase().contains("dev")));

        // Multiple matches with different capitalizations
        let results = search_by_position(&employees, "manager");
        assert_eq!(results.len(), 2);
        assert!(results
            .iter()
            .all(|e| e.position.to_lowercase().contains("manager")));

        // Single match
        let results = search_by_position(&employees, "senior");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].position, "Senior Developer");

        // No matches
        let results = search_by_position(&employees, "CEO");
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn search_employees_by_separation_date() {
        let today = Local::now().date_naive();
        let seven_months_ago = today
            .checked_sub_months(Months::new(7))
            .expect("Date underflowed");
        let five_months_ago = today
            .checked_sub_months(Months::new(5))
            .expect("Date underflowed");
        let four_months_ago = today
            .checked_sub_months(Months::new(4))
            .expect("Date underflowed");
        let three_months_ago = today
            .checked_sub_months(Months::new(3))
            .expect("Date underflowed");
        let one_month_ago = today
            .checked_sub_months(Months::new(1))
            .expect("Date underflowed");

        let employees = vec![
            Employee {
                first_name: "John".to_string(),
                last_name: "Doe".to_string(),
                position: "Developer".to_string(),
                separation_date: Some(seven_months_ago), // Outside 6-month window
            },
            Employee {
                first_name: "Jane".to_string(),
                last_name: "Smith".to_string(),
                position: "Manager".to_string(),
                separation_date: Some(five_months_ago), // Inside 6-month window
            },
            Employee {
                first_name: "Bob".to_string(),
                last_name: "Johnson".to_string(),
                position: "Developer".to_string(),
                separation_date: Some(four_months_ago), // Inside 6-month window
            },
            Employee {
                first_name: "Alice".to_string(),
                last_name: "Brown".to_string(),
                position: "Designer".to_string(),
                separation_date: None, // No separation date
            },
            Employee {
                first_name: "Chris".to_string(),
                last_name: "Wilson".to_string(),
                position: "Tester".to_string(),
                separation_date: Some(three_months_ago), // Inside 6-month window
            },
            Employee {
                first_name: "Sarah".to_string(),
                last_name: "Taylor".to_string(),
                position: "Analyst".to_string(),
                separation_date: Some(one_month_ago), // Inside 6-month window
            },
        ];

        let results = search_by_separation_date(&employees);
        assert_eq!(results.len(), 4); // Should include all separation dates within last 6 months

        // Verify the correct employees are included
        let result_names: Vec<String> = results.iter().map(|e| e.first_name.clone()).collect();

        assert!(result_names.contains(&"Jane".to_string()));
        assert!(result_names.contains(&"Bob".to_string()));
        assert!(result_names.contains(&"Chris".to_string()));
        assert!(result_names.contains(&"Sarah".to_string()));

        // Verify excluded employees
        assert!(!result_names.contains(&"John".to_string())); // Outside window
        assert!(!result_names.contains(&"Alice".to_string())); // No separation date
    }
}
