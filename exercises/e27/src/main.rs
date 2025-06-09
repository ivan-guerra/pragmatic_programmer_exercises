//! # Employee Information Validator
//!
//! This module implements an interactive GUI application that validates employee
//! information form entries based on specific formatting requirements.
//!
//! ## Features
//!
//! - **Real-time Validation**: Validates input fields as users complete their entries
//! - **Format Requirements**: Enforces specific formats for names, IDs, and postal codes
//! - **Immediate Feedback**: Displays clear error messages for invalid entries
//! - **Focus-based Validation**: Performs validation when users move between fields
//! - **Regular Expression Patterns**: Uses regex patterns for precise format validation
//! - **Field-specific Rules**: Implements different validation rules for each input type
use eframe::egui::{self};
use regex::Regex;

#[derive(Debug, Default)]
struct EmployeeInfo {
    first_name: String,
    last_name: String,
    employee_id: String,
    zipcode: String,
    first_name_error: bool,
    last_name_error: bool,
    employee_id_error: bool,
    zipcode_error: bool,
}

impl EmployeeInfo {
    fn is_valid_first_name(&self) -> bool {
        let re = Regex::new(r"^[A-Za-z]{2,}$").unwrap();
        re.is_match(&self.first_name)
    }

    fn is_valid_last_name(&self) -> bool {
        let re = Regex::new(r"^[A-Za-z]{2,}$").unwrap();
        re.is_match(&self.last_name)
    }

    fn is_valid_employee_id(&self) -> bool {
        let re = Regex::new(r"^[A-Za-z]{2}-\d{4}$").unwrap();
        re.is_match(&self.employee_id)
    }

    fn is_valid_zipcode(&self) -> bool {
        let re = Regex::new(r"^\d{5}$").unwrap();
        re.is_match(&self.zipcode)
    }
}

impl eframe::App for EmployeeInfo {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Enter the first name:");
            let first_name_response = ui.add(egui::TextEdit::singleline(&mut self.first_name));
            if first_name_response.changed() {
                self.first_name_error = false;
            } else if first_name_response.lost_focus() && !self.first_name.is_empty() {
                self.first_name_error = !self.is_valid_first_name();
            }
            if self.first_name_error {
                ui.label("Invalid first name. Must be at least 2 letters.");
            }

            ui.label("Enter the last name:");
            let last_name_response = ui.add(egui::TextEdit::singleline(&mut self.last_name));
            if last_name_response.changed() {
                self.last_name_error = false;
            } else if last_name_response.lost_focus() && !self.last_name.is_empty() {
                self.last_name_error = !self.is_valid_last_name();
            }
            if self.last_name_error {
                ui.label("Invalid last name. Must be at least 2 letters.");
            }

            ui.label("Enter the employee ID (format: AA-1234):");
            let employee_id_response = ui.add(egui::TextEdit::singleline(&mut self.employee_id));
            if employee_id_response.changed() {
                self.employee_id_error = false;
            } else if employee_id_response.lost_focus() && !self.employee_id.is_empty() {
                self.employee_id_error = !self.is_valid_employee_id();
            }
            if self.employee_id_error {
                ui.label("Invalid employee ID. Must be in format AA-1234.");
            }

            ui.label("Enter the zipcode (5 digits):");
            let zipcode_response = ui.add(egui::TextEdit::singleline(&mut self.zipcode));
            if zipcode_response.changed() {
                self.zipcode_error = false;
            } else if zipcode_response.lost_focus() && !self.zipcode.is_empty() {
                self.zipcode_error = !self.is_valid_zipcode();
            }
            if self.zipcode_error {
                ui.label("Invalid zipcode. Must be exactly 5 digits.");
            }
        });
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([400.0, 250.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Employee Information Validator",
        options,
        Box::new(|_| Ok(Box::<EmployeeInfo>::default())),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_valid_first_name_validates_correctly() {
        let info = EmployeeInfo {
            first_name: "John".to_string(),
            ..Default::default()
        };
        assert!(info.is_valid_first_name());

        let info = EmployeeInfo {
            first_name: "J".to_string(), // Too short
            ..Default::default()
        };
        assert!(!info.is_valid_first_name());

        let info = EmployeeInfo {
            first_name: "John123".to_string(), // Contains numbers
            ..Default::default()
        };
        assert!(!info.is_valid_first_name());

        let info = EmployeeInfo {
            first_name: "John Doe".to_string(), // Contains space
            ..Default::default()
        };
        assert!(!info.is_valid_first_name());

        let info = EmployeeInfo {
            first_name: "".to_string(), // Empty
            ..Default::default()
        };
        assert!(!info.is_valid_first_name());
    }

    #[test]
    fn is_valid_last_name_validates_correctly() {
        let info = EmployeeInfo {
            last_name: "Smith".to_string(),
            ..Default::default()
        };
        assert!(info.is_valid_last_name());

        let info = EmployeeInfo {
            last_name: "S".to_string(), // Too short
            ..Default::default()
        };
        assert!(!info.is_valid_last_name());

        let info = EmployeeInfo {
            last_name: "Smith123".to_string(), // Contains numbers
            ..Default::default()
        };
        assert!(!info.is_valid_last_name());

        let info = EmployeeInfo {
            last_name: "Smith Jones".to_string(), // Contains space
            ..Default::default()
        };
        assert!(!info.is_valid_last_name());

        let info = EmployeeInfo {
            last_name: "".to_string(), // Empty
            ..Default::default()
        };
        assert!(!info.is_valid_last_name());
    }

    #[test]
    fn is_valid_employee_id_validates_correctly() {
        let info = EmployeeInfo {
            employee_id: "AB-1234".to_string(),
            ..Default::default()
        };
        assert!(info.is_valid_employee_id());

        let info = EmployeeInfo {
            employee_id: "ABC-1234".to_string(), // Too many letters
            ..Default::default()
        };
        assert!(!info.is_valid_employee_id());

        let info = EmployeeInfo {
            employee_id: "A-1234".to_string(), // Too few letters
            ..Default::default()
        };
        assert!(!info.is_valid_employee_id());

        let info = EmployeeInfo {
            employee_id: "AB-123".to_string(), // Too few digits
            ..Default::default()
        };
        assert!(!info.is_valid_employee_id());

        let info = EmployeeInfo {
            employee_id: "AB-12345".to_string(), // Too many digits
            ..Default::default()
        };
        assert!(!info.is_valid_employee_id());

        let info = EmployeeInfo {
            employee_id: "AB1234".to_string(), // Missing hyphen
            ..Default::default()
        };
        assert!(!info.is_valid_employee_id());

        let info = EmployeeInfo {
            employee_id: "12-ABCD".to_string(), // Swapped format
            ..Default::default()
        };
        assert!(!info.is_valid_employee_id());

        let info = EmployeeInfo {
            employee_id: "".to_string(), // Empty
            ..Default::default()
        };
        assert!(!info.is_valid_employee_id());
    }

    #[test]
    fn is_valid_zipcode_validates_correctly() {
        let info = EmployeeInfo {
            zipcode: "12345".to_string(),
            ..Default::default()
        };
        assert!(info.is_valid_zipcode());

        let info = EmployeeInfo {
            zipcode: "1234".to_string(), // Too few digits
            ..Default::default()
        };
        assert!(!info.is_valid_zipcode());

        let info = EmployeeInfo {
            zipcode: "123456".to_string(), // Too many digits
            ..Default::default()
        };
        assert!(!info.is_valid_zipcode());

        let info = EmployeeInfo {
            zipcode: "ABCDE".to_string(), // Contains letters
            ..Default::default()
        };
        assert!(!info.is_valid_zipcode());

        let info = EmployeeInfo {
            zipcode: "123-45".to_string(), // Contains special character
            ..Default::default()
        };
        assert!(!info.is_valid_zipcode());

        let info = EmployeeInfo {
            zipcode: "".to_string(), // Empty
            ..Default::default()
        };
        assert!(!info.is_valid_zipcode());
    }
}
