//! # Website Structure Generator
//!
//! This module implements a command-line application for creating basic website directory
//! structures with customizable components based on user preferences.
//!
//! ## Features
//!
//! - **Interactive Configuration**: Prompts users for site name, author, and folder preferences
//! - **Directory Creation**: Generates properly nested folder structure for website projects
//! - **HTML Generation**: Creates a starter index.html file with proper metadata and basic content
//! - **Optional Components**: Supports conditional creation of CSS and JavaScript directories
//! - **Error Handling**: Provides graceful error reporting for file system operations
//!
//! The application guides users through defining a website structure, creates the
//! directories and starter files according to specifications, and confirms successful
//! creation with appropriate feedback.
use std::{
    io::{self, Write},
    path::{Path, PathBuf},
};

struct SiteConfig {
    name: String,
    author: String,
    has_js_folder: bool,
    has_css_folder: bool,
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

fn create_directory(path: &PathBuf) -> std::io::Result<()> {
    if !path.exists() {
        std::fs::create_dir_all(path)?;
    }
    Ok(())
}

fn create_index_html(path: &Path, config: &SiteConfig) -> std::io::Result<()> {
    let index_path = path.join("index.html");
    let content = format!(
        "<!DOCTYPE html>
<html lang=\"en\">
<head>
    <meta charset=\"UTF-8\">
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
    <meta name=\"author\" content=\"{}\">
    <title>{}</title>
</head>
<body>
    <h1>Welcome to {}</h1>
    <p>Created by {}</p>
</body>
</html>",
        config.author, config.name, config.name, config.author
    );
    std::fs::write(index_path, content)?;
    Ok(())
}

fn create_site_structure(config: &SiteConfig) -> std::io::Result<()> {
    // let the base path be the current working directory plus the site name
    let base_path = std::env::current_dir()?.join(&config.name);
    create_directory(&base_path)?;

    create_index_html(&base_path, config)?;

    if config.has_js_folder {
        create_directory(&base_path.join("js"))?;
    }

    if config.has_css_folder {
        create_directory(&base_path.join("css"))?;
    }

    Ok(())
}

fn main() {
    let config = SiteConfig {
        name: prompt_for_str("Site name: "),
        author: prompt_for_str("Author: "),
        has_js_folder: prompt_for_yes_no("Do you want a folder for JavaScript: "),
        has_css_folder: prompt_for_yes_no("Do you want a folder for CSS: "),
    };

    if let Err(e) = create_site_structure(&config) {
        eprintln!("Error creating site structure: {}", e);
    } else {
        println!("Site structure created successfully in '{}'.", config.name);
    }
}
