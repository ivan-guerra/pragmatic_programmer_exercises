//! # Password Strength Validator
//!
//! This module implements an interactive GUI application that evaluates password strength
//! based on established security criteria.
//!
//! ## Features
//!
//! - **Interactive Interface**: Real-time visual feedback on password strength
//! - **Comprehensive Validation**: Evaluates length, character diversity, and complexity
//! - **Multi-level Strength Classification**: Categorizes passwords into four strength levels
//! - **Visual Indicators**: Color-coded feedback based on password strength
//! - **Security Rules**: Enforces modern password security best practices
//! - **Validation Logic**: Clear criteria for each password strength level
use eframe::egui::{self};

enum PasswordStrength {
    VeryWeak,
    Weak,
    Strong,
    VeryStrong,
}

#[derive(Debug, Default)]
struct PasswordValidator {
    password: String,
}

impl PasswordValidator {
    fn is_very_weak(&self) -> bool {
        if self.password.is_empty() {
            return true;
        }
        self.password.len() < 8 && self.password.chars().all(|c| c.is_numeric())
    }

    fn is_weak(&self) -> bool {
        if self.password.is_empty() {
            return false;
        }
        self.password.len() < 8 && self.password.chars().all(|c| c.is_alphabetic())
    }

    fn is_strong(&self) -> bool {
        if self.password.is_empty() {
            return false;
        }
        self.password.len() >= 8
            && self.password.chars().any(|c| c.is_numeric())
            && self.password.chars().any(|c| c.is_alphabetic())
            && self.password.chars().all(|c| c.is_alphanumeric())
    }

    fn is_very_strong(&self) -> bool {
        if self.password.is_empty() {
            return false;
        }
        self.password.len() >= 8
            && self.password.chars().any(|c| c.is_numeric())
            && self.password.chars().any(|c| c.is_alphabetic())
            && self.password.chars().any(|c| !c.is_alphanumeric())
    }

    fn get_password_strength(&self) -> PasswordStrength {
        if self.is_very_weak() {
            PasswordStrength::VeryWeak
        } else if self.is_weak() {
            PasswordStrength::Weak
        } else if self.is_strong() {
            PasswordStrength::Strong
        } else if self.is_very_strong() {
            PasswordStrength::VeryStrong
        } else {
            PasswordStrength::Weak // Default case for invalid passwords
        }
    }
}

impl eframe::App for PasswordValidator {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Enter Password:");
            ui.text_edit_singleline(&mut self.password);

            if !self.password.is_empty() {
                match self.get_password_strength() {
                    PasswordStrength::VeryWeak => {
                        ui.colored_label(egui::Color32::RED, "Very Weak Password");
                    }
                    PasswordStrength::Weak => {
                        ui.colored_label(egui::Color32::YELLOW, "Weak Password");
                    }
                    PasswordStrength::Strong => {
                        ui.colored_label(egui::Color32::DARK_GREEN, "Strong Password");
                    }
                    PasswordStrength::VeryStrong => {
                        ui.colored_label(egui::Color32::GREEN, "Very Strong Password");
                    }
                }
            }
        });
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([400.0, 100.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Password Validator",
        options,
        Box::new(|_| Ok(Box::<PasswordValidator>::default())),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_very_weak_identifies_passwords_correctly() {
        let validator = PasswordValidator {
            password: "123456".to_string(),
        };
        assert!(validator.is_very_weak());

        let validator = PasswordValidator {
            password: "12345678".to_string(),
        };
        assert!(!validator.is_very_weak()); // Long enough but only numbers

        let validator = PasswordValidator {
            password: "123abc".to_string(),
        };
        assert!(!validator.is_very_weak()); // Contains letters

        let validator = PasswordValidator {
            password: "".to_string(),
        };
        assert!(validator.is_very_weak()); // Empty string
    }

    #[test]
    fn is_weak_identifies_passwords_correctly() {
        let validator = PasswordValidator {
            password: "abcdef".to_string(),
        };
        assert!(validator.is_weak());

        let validator = PasswordValidator {
            password: "PASSWORD".to_string(),
        };
        assert!(!validator.is_weak());

        let validator = PasswordValidator {
            password: "abcdefgh".to_string(),
        };
        assert!(!validator.is_weak()); // Long enough but only letters

        let validator = PasswordValidator {
            password: "abc123".to_string(),
        };
        assert!(!validator.is_weak()); // Contains numbers
    }

    #[test]
    fn is_strong_identifies_passwords_correctly() {
        let validator = PasswordValidator {
            password: "abcd1234".to_string(),
        };
        assert!(validator.is_strong());

        let validator = PasswordValidator {
            password: "Pass1234".to_string(),
        };
        assert!(validator.is_strong());

        let validator = PasswordValidator {
            password: "pass123".to_string(),
        };
        assert!(!validator.is_strong()); // Not long enough

        let validator = PasswordValidator {
            password: "password".to_string(),
        };
        assert!(!validator.is_strong()); // No numbers

        let validator = PasswordValidator {
            password: "12345678".to_string(),
        };
        assert!(!validator.is_strong()); // No letters

        let validator = PasswordValidator {
            password: "Pass123!".to_string(),
        };
        assert!(!validator.is_strong()); // Contains special character
    }

    #[test]
    fn is_very_strong_identifies_passwords_correctly() {
        let validator = PasswordValidator {
            password: "abcd123!".to_string(),
        };
        assert!(validator.is_very_strong());

        let validator = PasswordValidator {
            password: "P@ssw0rd".to_string(),
        };
        assert!(validator.is_very_strong());

        let validator = PasswordValidator {
            password: "pass123".to_string(),
        };
        assert!(!validator.is_very_strong()); // Not long enough

        let validator = PasswordValidator {
            password: "password!".to_string(),
        };
        assert!(!validator.is_very_strong()); // No numbers

        let validator = PasswordValidator {
            password: "Pass1234".to_string(),
        };
        assert!(!validator.is_very_strong()); // No special characters
    }

    #[test]
    fn get_password_strength_returns_correct_strength() {
        // Very weak passwords (numbers only, less than 8 chars)
        let validator = PasswordValidator {
            password: "123456".to_string(),
        };
        assert!(matches!(
            validator.get_password_strength(),
            PasswordStrength::VeryWeak
        ));

        // Weak passwords (letters only, less than 8 chars)
        let validator = PasswordValidator {
            password: "abcdef".to_string(),
        };
        assert!(matches!(
            validator.get_password_strength(),
            PasswordStrength::Weak
        ));

        // Strong passwords (letters and numbers, at least 8 chars, alphanumeric only)
        let validator = PasswordValidator {
            password: "abcd1234".to_string(),
        };
        assert!(matches!(
            validator.get_password_strength(),
            PasswordStrength::Strong
        ));

        // Very strong passwords (letters, numbers, and special chars, at least 8 chars)
        let validator = PasswordValidator {
            password: "P@ssw0rd".to_string(),
        };
        assert!(matches!(
            validator.get_password_strength(),
            PasswordStrength::VeryStrong
        ));

        // Default case
        let validator = PasswordValidator {
            password: "a1!".to_string(),
        }; // Too short with mixed types
        assert!(matches!(
            validator.get_password_strength(),
            PasswordStrength::Weak
        ));
    }
}
