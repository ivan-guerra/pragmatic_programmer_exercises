//! # Word Replacement Utility
//!
//! This module implements a text processing application for systematically replacing
//! specified words across multiple files and directories with proper tracking.
//!
//! ## Features
//!
//! - **Bulk Text Processing**: Processes multiple files across directory structures
//! - **Pattern-Based Replacement**: Uses regular expressions for accurate word boundary detection
//! - **Configuration File**: Reads replacement pairs from a dedicated configuration file
//! - **Recursive Directory Traversal**: Handles nested directory structures
//! - **Replacement Tracking**: Counts and reports the number of replacements for each word
//!
//! The application reads a list of word replacements from a configuration file,
//! traverses a specified directory structure, applies the word replacements to all
//! text files, and provides a summary report of the replacements made.
use anyhow::anyhow;
use std::collections::HashMap;
use std::io::{BufRead, Write};
use std::path::PathBuf;

fn read_replacement_file(file_path: &PathBuf) -> Result<HashMap<String, String>, std::io::Error> {
    let file = std::fs::File::open(file_path)?;
    let reader = std::io::BufReader::new(file);
    let mut replacements = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() == 2 {
            replacements.insert(parts[0].trim().to_string(), parts[1].trim().to_string());
        }
    }
    Ok(replacements)
}

fn read_text_file(file_path: &PathBuf) -> Result<String, std::io::Error> {
    let file = std::fs::File::open(file_path)?;
    let reader = std::io::BufReader::new(file);
    let mut content = String::new();

    for line in reader.lines() {
        content.push_str(&line?);
        content.push('\n');
    }
    Ok(content)
}

fn write_text_file(file_path: &PathBuf, content: &str) -> Result<(), std::io::Error> {
    let mut file = std::fs::File::create(file_path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn replace_words(
    content: &str,
    replacements: &HashMap<String, String>,
) -> (String, HashMap<String, u32>) {
    let mut result = content.to_string();
    let mut replacement_counts = HashMap::new();

    for (old_word, new_word) in replacements {
        let old_word_pattern = format!(r"\b{}\b", regex::escape(old_word));
        let re = regex::Regex::new(&old_word_pattern).unwrap();
        let count = re.find_iter(&result).count() as u32;

        result = re.replace_all(&result, new_word).to_string();
        replacement_counts.insert(old_word.clone(), count);
    }

    (result, replacement_counts)
}

fn replace_words_in_dir(
    dir_path: &PathBuf,
    replacements: &HashMap<String, String>,
    replacement_cnts: &mut HashMap<String, u32>,
) -> anyhow::Result<()> {
    for entry in std::fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let content = read_text_file(&path)?;
            let (updated_content, local_replacements) = replace_words(&content, replacements);

            write_text_file(&path, &updated_content)?;
            for (word, count) in local_replacements {
                *replacement_cnts.entry(word).or_insert(0) += count;
            }
        } else if path.is_dir() {
            replace_words_in_dir(&path, replacements, replacement_cnts)?;
        }
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let replacement_file = PathBuf::from("exercises/e45/inputs/replacements.txt");
    let input_dir = PathBuf::from("exercises/e45/inputs/test");

    let replacements = read_replacement_file(&replacement_file)
        .map_err(|e| anyhow!("Error reading replacement file: {}", e))?;
    let mut replacement_counts = HashMap::new();

    replace_words_in_dir(&input_dir, &replacements, &mut replacement_counts)
        .map_err(|e| anyhow!("Error processing directory: {}", e))?;
    for (word, count) in &replacement_counts {
        println!("Replaced '{}' {} time(s).", word, count);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_words_with_exact_matches() {
        let content = "The quick brown fox jumps over the lazy dog.";
        let mut replacements = HashMap::new();
        replacements.insert("quick".to_string(), "speedy".to_string());
        replacements.insert("lazy".to_string(), "sleeping".to_string());

        let (result, counts) = replace_words(content, &replacements);

        assert_eq!(result, "The speedy brown fox jumps over the sleeping dog.");
        assert_eq!(counts.get("quick"), Some(&1));
        assert_eq!(counts.get("lazy"), Some(&1));
    }

    #[test]
    fn test_replace_words_with_multiple_occurrences() {
        let content = "The cat and the other cat sat on the mat.";
        let mut replacements = HashMap::new();
        replacements.insert("cat".to_string(), "dog".to_string());
        replacements.insert("mat".to_string(), "rug".to_string());

        let (result, counts) = replace_words(content, &replacements);

        assert_eq!(result, "The dog and the other dog sat on the rug.");
        assert_eq!(counts.get("cat"), Some(&2));
        assert_eq!(counts.get("mat"), Some(&1));
    }

    #[test]
    fn test_replace_words_with_word_boundaries() {
        let content = "The category of cat includes the catamaran.";
        let mut replacements = HashMap::new();
        replacements.insert("cat".to_string(), "dog".to_string());

        let (result, counts) = replace_words(content, &replacements);

        // Only standalone "cat" should be replaced, not parts of larger words
        assert_eq!(result, "The category of dog includes the catamaran.");
        assert_eq!(counts.get("cat"), Some(&1));
    }

    #[test]
    fn test_replace_words_with_no_matches() {
        let content = "The quick brown fox jumps over the lazy dog.";
        let mut replacements = HashMap::new();
        replacements.insert("elephant".to_string(), "giraffe".to_string());
        replacements.insert("zebra".to_string(), "lion".to_string());

        let (result, counts) = replace_words(content, &replacements);

        assert_eq!(result, "The quick brown fox jumps over the lazy dog.");
        assert_eq!(counts.get("elephant"), Some(&0));
        assert_eq!(counts.get("zebra"), Some(&0));
    }

    #[test]
    fn test_replace_words_with_empty_content() {
        let content = "";
        let mut replacements = HashMap::new();
        replacements.insert("word".to_string(), "replacement".to_string());

        let (result, counts) = replace_words(content, &replacements);

        assert_eq!(result, "");
        assert_eq!(counts.get("word"), Some(&0));
    }

    #[test]
    fn test_replace_words_with_punctuation() {
        let content = "Hello, world! How are you today?";
        let mut replacements = HashMap::new();
        replacements.insert("Hello".to_string(), "Hi".to_string());
        replacements.insert("world".to_string(), "planet".to_string());

        let (result, counts) = replace_words(content, &replacements);

        assert_eq!(result, "Hi, planet! How are you today?");
        assert_eq!(counts.get("Hello"), Some(&1));
        assert_eq!(counts.get("world"), Some(&1));
    }
}
