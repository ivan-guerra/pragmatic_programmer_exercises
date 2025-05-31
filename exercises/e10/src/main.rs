//! # Self-Checkout System
//!
//! This module implements an interactive self-checkout application that
//! calculates the total price of items with appropriate sales tax.
//!
//! ## Features
//!
//! - **Multi-Item Entry**: Allows users to input multiple items with quantity and price
//! - **Subtotal Calculation**: Computes the pre-tax cost of all items
//! - **Tax Calculation**: Applies configurable tax rates to purchases
//! - **Receipt Generation**: Creates a formatted receipt with subtotal, tax, and total
//! - **Item Validation**: Ensures valid quantities and prices are entered
use std::fmt::Display;
use std::io::Write;

struct PurchaseItem {
    quanity: u32,
    price_per_item: f64,
}

struct PurchaseReceipt {
    items: Vec<PurchaseItem>,
}

impl PurchaseReceipt {
    fn total_cost(&self) -> f64 {
        if self.items.is_empty() {
            return 0.0;
        }

        self.items
            .iter()
            .map(|item| item.quanity as f64 * item.price_per_item)
            .sum()
    }

    fn tax(&self, tax_rate: f64) -> f64 {
        self.total_cost() * tax_rate
    }

    fn total_with_tax(&self, tax_rate: f64) -> f64 {
        self.total_cost() + self.tax(tax_rate)
    }
}

impl Display for PurchaseReceipt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const TAX_RATE: f64 = 0.055;
        write!(f, "Subtotal: ${:.2}", self.total_cost())?;
        write!(f, "\nTax: ${:.2}", self.tax(TAX_RATE))?;
        write!(f, "\nTotal: ${:.2}", self.total_with_tax(TAX_RATE))
    }
}

fn prompt_for_purchase_items() -> PurchaseReceipt {
    let mut items = Vec::new();
    let mut item_number = 1;
    loop {
        print!("Enter the quantity of item {item_number} (or 'done' to finish): ");
        if let Err(e) = std::io::stdout().flush() {
            eprintln!("Error: {}", e);
            continue;
        }

        let mut input = String::new();
        if let Err(e) = std::io::stdin().read_line(&mut input) {
            eprintln!("Error: {}", e);
            continue;
        }

        let input = input.trim();
        if input.eq_ignore_ascii_case("done") {
            break;
        }

        if let Ok(quantity) = input.parse::<u32>() {
            print!("Enter the price of item {item_number}: ");
            if let Err(e) = std::io::stdout().flush() {
                eprintln!("Error: {}", e);
                continue;
            }

            let mut price_input = String::new();
            if let Err(e) = std::io::stdin().read_line(&mut price_input) {
                eprintln!("Error: {}", e);
                continue;
            }

            if let Ok(price_per_item) = price_input.trim().parse::<f64>() {
                items.push(PurchaseItem {
                    quanity: quantity,
                    price_per_item,
                });
                item_number += 1;
            } else {
                println!("Invalid price. Please enter a valid number.");
            }
        } else {
            println!("Invalid quantity. Please enter a valid number.");
        }
    }
    PurchaseReceipt { items }
}

fn main() {
    let receipt = prompt_for_purchase_items();
    println!("{}", receipt);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_receipt() -> PurchaseReceipt {
        PurchaseReceipt {
            items: vec![
                PurchaseItem {
                    quanity: 2,
                    price_per_item: 10.0,
                },
                PurchaseItem {
                    quanity: 1,
                    price_per_item: 15.0,
                },
                PurchaseItem {
                    quanity: 3,
                    price_per_item: 5.0,
                },
            ],
        }
    }

    #[test]
    fn total_cost_calculates_sum_correctly() {
        let receipt = create_test_receipt();
        // (2 * 10.0) + (1 * 15.0) + (3 * 5.0) = 20.0 + 15.0 + 15.0 = 50.0
        assert_eq!(receipt.total_cost(), 50.0);
    }

    #[test]
    fn total_cost_handles_empty_receipt() {
        let receipt = PurchaseReceipt { items: vec![] };
        assert_eq!(receipt.total_cost(), 0.0);
    }

    #[test]
    fn tax_calculates_correct_amount() {
        let receipt = create_test_receipt();
        // 50.0 * 0.05 = 2.5
        assert_eq!(receipt.tax(0.05), 2.5);
        // 50.0 * 0.1 = 5.0
        assert_eq!(receipt.tax(0.1), 5.0);
        // 50.0 * 0.0 = 0.0
        assert_eq!(receipt.tax(0.0), 0.0);
    }

    #[test]
    fn tax_handles_empty_receipt() {
        let receipt = PurchaseReceipt { items: vec![] };
        assert_eq!(receipt.tax(0.05), 0.0);
    }

    #[test]
    fn total_with_tax_calculates_correct_amount() {
        let receipt = create_test_receipt();
        // 50.0 + (50.0 * 0.05) = 50.0 + 2.5 = 52.5
        assert_eq!(receipt.total_with_tax(0.05), 52.5);
        // 50.0 + (50.0 * 0.1) = 50.0 + 5.0 = 55.0
        assert_eq!(receipt.total_with_tax(0.1), 55.0);
        // 50.0 + (50.0 * 0.0) = 50.0 + 0.0 = 50.0
        assert_eq!(receipt.total_with_tax(0.0), 50.0);
    }

    #[test]
    fn total_with_tax_handles_empty_receipt() {
        let receipt = PurchaseReceipt { items: vec![] };
        assert_eq!(receipt.total_with_tax(0.05), 0.0);
    }

    #[test]
    fn display_formats_receipt_correctly() {
        let receipt = create_test_receipt();
        let display_string = format!("{}", receipt);

        // The tax rate in the Display implementation is 0.055 (5.5%)
        // Subtotal: $50.00
        // Tax: $2.75 (50.0 * 0.055)
        // Total: $52.75 (50.0 + 2.75)

        assert!(display_string.contains("Subtotal: $50.00"));
        assert!(display_string.contains("Tax: $2.75"));
        assert!(display_string.contains("Total: $52.75"));
    }

    #[test]
    fn display_handles_empty_receipt() {
        let receipt = PurchaseReceipt { items: vec![] };
        let display_string = format!("{}", receipt);

        dbg!(&display_string);
        assert!(display_string.contains("Subtotal: $0.00"));
        assert!(display_string.contains("Tax: $0.00"));
        assert!(display_string.contains("Total: $0.00"));
    }
}
