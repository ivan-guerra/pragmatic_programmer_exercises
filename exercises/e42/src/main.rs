//! # Employee Salary Reporting Tool
//!
//! This module provides functionality for loading, sorting, and displaying employee
//! salary information from CSV data. It supports data processing with the following
//! features:
//!
//! - **CSV Parsing**: Reads employee records from headerless CSV files
//! - **Salary-Based Sorting**: Sorts employees in descending order by salary
//! - **Formatted Output**: Presents employee data in a clean, properly aligned tabular format
//! - **Number Formatting**: Uses locale-aware formatting for salary values
//!
//! The application reads employee records (first name, last name, and salary),
//! sorts them from highest to lowest salary, and displays the results in a
//! formatted table with dynamically sized columns.
use num_format::{Locale, ToFormattedString};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Employee {
    first_name: String,
    last_name: String,
    salary: u32,
}

fn read_employees_csv(file_path: &str) -> Result<Vec<Employee>, Box<dyn std::error::Error>> {
    let file = std::fs::File::open(file_path)?;
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(file);

    let mut employees = Vec::new();

    for result in rdr.deserialize() {
        let employee: Employee = result?;
        employees.push(employee);
    }

    Ok(employees)
}

fn sort_by_salary(employees: &mut [Employee]) {
    employees.sort_by(|a, b| {
        b.salary
            .partial_cmp(&a.salary)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
}

fn print_employees(employees: &[Employee]) {
    // Find the maximum width needed for each column
    let max_last_width = employees
        .iter()
        .map(|e| e.last_name.len())
        .max()
        .unwrap_or(4) // "Last" header length
        .max(4)
        + 1;

    let max_first_width = employees
        .iter()
        .map(|e| e.first_name.len())
        .max()
        .unwrap_or(5) // "First" header length
        .max(5)
        + 1;

    let max_salary_width = employees
        .iter()
        .map(|e| e.salary.to_formatted_string(&Locale::en).to_string().len())
        .max()
        .unwrap_or(6) // "Salary" header length
        .max(6)
        + 1;

    // Print the headers
    println!(
        "{:<width_last$}{:<width_first$}{:<width_salary$}",
        "Last",
        "First",
        "Salary",
        width_last = max_last_width,
        width_first = max_first_width,
        width_salary = max_salary_width
    );

    // Print a separator line
    println!(
        "{:-<width_last$}{:-<width_first$}{:-<width_salary$}",
        "",
        "",
        "",
        width_last = max_last_width,
        width_first = max_first_width,
        width_salary = max_salary_width
    );

    // Print each employee
    for employee in employees {
        println!(
            "{:<width_last$}{:<width_first$}${:<width_salary$}",
            employee.last_name,
            employee.first_name,
            employee.salary.to_formatted_string(&Locale::en),
            width_last = max_last_width,
            width_first = max_first_width,
            width_salary = max_salary_width
        );
    }
}

fn main() {
    let file_path = "exercises/e42/inputs/employees.csv";

    match read_employees_csv(file_path) {
        Ok(mut employees) => {
            if employees.is_empty() {
                println!("No employees found in the file.");
                return;
            }
            sort_by_salary(&mut employees);
            print_employees(&employees);
        }
        Err(e) => eprintln!("Error reading employees: {}", e),
    }
}
