//! # Product Inventory Management System
//!
//! This module implements a command-line application for managing product inventory
//! by providing lookup, display, and addition capabilities for product records.
//!
//! ## Features
//!
//! - **JSON Data Storage**: Reads from and writes to JSON files for persistent product data
//! - **Interactive Search**: Allows users to look up products by name
//! - **Product Details**: Displays formatted product information including price and quantity
//! - **Inventory Expansion**: Supports adding new products when items aren't found
//! - **Data Validation**: Ensures proper input formats for prices and quantities
//!
//! The application loads a product inventory from JSON, enables users to search for
//! specific items by name, shows detailed product information, and offers the option
//! to add missing products with the system maintaining persistence across sessions.
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::io::{self, Write};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
struct Product {
    name: String,
    price: f64,
    quantity: u32,
}

impl Display for Product {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Name: {}\nPrice: ${:.2}\nQuantity on hand: {}",
            self.name, self.price, self.quantity
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct ProductList {
    products: Vec<Product>,
}

fn read_products_json(file_path: &PathBuf) -> Result<ProductList, std::io::Error> {
    let file = std::fs::File::open(file_path)?;
    let reader = std::io::BufReader::new(file);
    let products: ProductList = serde_json::from_reader(reader)?;
    Ok(products)
}

fn write_products_json(file_path: &PathBuf, products: &ProductList) -> Result<(), std::io::Error> {
    let file = std::fs::File::create(file_path)?;
    serde_json::to_writer(file, products)?;
    Ok(())
}

fn prompt_for_str(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn prompt_for_yes_no(prompt: &str) -> bool {
    loop {
        let response = prompt_for_str(prompt);
        match response.to_lowercase().as_str() {
            "yes" | "y" => return true,
            "no" | "n" => return false,
            _ => println!("Please answer 'yes' or 'no'."),
        }
    }
}

fn prompt_for_product(name: &str) -> Product {
    let price: f64 = loop {
        let input = prompt_for_str("Enter product price: ");
        match input.parse() {
            Ok(value) => break value,
            Err(_) => println!("Invalid price. Please enter a valid number."),
        }
    };

    let quantity: u32 = loop {
        let input = prompt_for_str("Enter product quantity: ");
        match input.parse() {
            Ok(value) => break value,
            Err(_) => println!("Invalid quantity. Please enter a valid number."),
        }
    };

    Product {
        name: name.to_string(),
        price,
        quantity,
    }
}

fn main() {
    let file_path = PathBuf::from("exercises/e44/inputs/products.json");
    match read_products_json(&file_path) {
        Ok(mut product_list) => loop {
            let product_name = prompt_for_str("Enter product name (or 'exit' to quit): ");
            if product_name.to_lowercase() == "exit" {
                break;
            }

            if let Some(product) = product_list
                .products
                .iter_mut()
                .find(|p| p.name.to_lowercase() == product_name.to_lowercase())
            {
                println!("{product}");
            } else {
                println!("Product '{product_name}' not found.");
                let add_item = prompt_for_yes_no("Would you like to add this product? (yes/no): ");
                if add_item {
                    let new_product = prompt_for_product(&product_name);
                    product_list.products.push(new_product);
                    write_products_json(&file_path, &product_list)
                        .expect("Failed to write product");
                }
            }
        },
        Err(e) => {
            eprintln!(
                "Failed to read products from {:?}: {}",
                file_path.display(),
                e
            );
        }
    }
}
