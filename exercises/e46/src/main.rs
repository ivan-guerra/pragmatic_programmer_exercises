//! # Word Frequency Counter
//!
//! This module implements a text analysis tool that counts and displays the frequency
//! of words in a document with a visual histogram representation.
//!
//! ## Features
//!
//! - **File Processing**: Reads text content from files for analysis
//! - **Text Normalization**: Handles punctuation, possessive forms, and case normalization
//! - **Word Frequency Analysis**: Counts occurrences of each unique word in the text
//! - **Visual Output**: Displays word frequencies as horizontal histogram bars
//! - **Sorted Results**: Presents words in descending order by frequency
//!
//! The application reads text from an input file, processes and normalizes the words,
//! counts their frequencies, and then displays a formatted histogram that visually
//! represents the relative frequency of each word in the document.
use anyhow::anyhow;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

fn read_file_content(file_path: &PathBuf) -> anyhow::Result<String> {
    let mut file =
        File::open(file_path).map_err(|_| anyhow!("Unable to open file {:?}", file_path))?;
    let mut content = String::new();

    file.read_to_string(&mut content)
        .map_err(|e| anyhow!("Unable to read file content: {}", e))?;
    Ok(content)
}

fn clean_suffix(word: &str) -> String {
    let re = Regex::new(r"('s)?[.;,!?]*$").unwrap();
    re.replace(word, "").to_string()
}

fn count_word_freq(content: &str) -> HashMap<String, u32> {
    let words: Vec<String> = content
        .split_whitespace()
        .map(|s| clean_suffix(s.to_lowercase().as_str()))
        .collect();
    let mut word_freq = HashMap::new();

    for word in words {
        *word_freq.entry(word).or_insert(0) += 1;
    }
    word_freq
}

fn plot_histogram(word_freq: &HashMap<String, u32>) {
    let mut counts: Vec<(&String, &u32)> = word_freq.iter().collect();
    let max_length = counts.iter().map(|kv| kv.0.len()).max().unwrap_or(0);

    counts.sort_by(|a, b| b.1.cmp(a.1));
    for kv in &counts {
        let padding = " ".repeat(max_length - kv.0.len() + 1);
        println!("{}{}: {}", kv.0, padding, "*".repeat(*kv.1 as usize));
    }
}

fn main() -> anyhow::Result<()> {
    let file_path = PathBuf::from("exercises/e46/inputs/words.txt");
    let content = read_file_content(&file_path)?;
    let word_freq = count_word_freq(&content);

    plot_histogram(&word_freq);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_suffix_with_period() {
        let result = clean_suffix("hello.");
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_clean_suffix_with_comma() {
        let result = clean_suffix("hello,");
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_clean_suffix_with_semicolon() {
        let result = clean_suffix("hello;");
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_clean_suffix_with_exclamation() {
        let result = clean_suffix("hello!");
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_clean_suffix_with_question_mark() {
        let result = clean_suffix("hello?");
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_clean_suffix_with_possessive() {
        let result = clean_suffix("John's");
        assert_eq!(result, "John");
    }

    #[test]
    fn test_clean_suffix_with_possessive_and_punctuation() {
        let result = clean_suffix("John's.");
        assert_eq!(result, "John");
    }

    #[test]
    fn test_clean_suffix_with_multiple_punctuation() {
        let result = clean_suffix("hello!?");
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_clean_suffix_with_no_suffix() {
        let result = clean_suffix("hello");
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_clean_suffix_with_empty_string() {
        let result = clean_suffix("");
        assert_eq!(result, "");
    }

    #[test]
    fn test_count_word_freq_with_single_occurrence() {
        let content = "hello world";
        let word_freq = count_word_freq(content);

        assert_eq!(word_freq.len(), 2);
        assert_eq!(word_freq.get("hello"), Some(&1));
        assert_eq!(word_freq.get("world"), Some(&1));
    }

    #[test]
    fn test_count_word_freq_with_multiple_occurrences() {
        let content = "hello hello world";
        let word_freq = count_word_freq(content);

        assert_eq!(word_freq.len(), 2);
        assert_eq!(word_freq.get("hello"), Some(&2));
        assert_eq!(word_freq.get("world"), Some(&1));
    }

    #[test]
    fn test_count_word_freq_with_punctuation() {
        let content = "hello, world! hello.";
        let word_freq = count_word_freq(content);

        assert_eq!(word_freq.len(), 2);
        assert_eq!(word_freq.get("hello"), Some(&2));
        assert_eq!(word_freq.get("world"), Some(&1));
    }

    #[test]
    fn test_count_word_freq_with_capitalization() {
        let content = "Hello World hello";
        let word_freq = count_word_freq(content);

        assert_eq!(word_freq.len(), 2);
        assert_eq!(word_freq.get("hello"), Some(&2));
        assert_eq!(word_freq.get("world"), Some(&1));
    }

    #[test]
    fn test_count_word_freq_with_possessive() {
        let content = "John's book and Mary's pen";
        let word_freq = count_word_freq(content);

        assert_eq!(word_freq.len(), 5);
        assert_eq!(word_freq.get("john"), Some(&1));
        assert_eq!(word_freq.get("book"), Some(&1));
        assert_eq!(word_freq.get("and"), Some(&1));
        assert_eq!(word_freq.get("mary"), Some(&1));
        assert_eq!(word_freq.get("pen"), Some(&1));
    }

    #[test]
    fn test_count_word_freq_with_empty_string() {
        let content = "";
        let word_freq = count_word_freq(content);

        assert_eq!(word_freq.len(), 0);
    }

    #[test]
    fn test_count_word_freq_with_whitespace_only() {
        let content = "   \n\t   ";
        let word_freq = count_word_freq(content);

        assert_eq!(word_freq.len(), 0);
    }

    #[test]
    fn test_count_word_freq_with_mixed_content() {
        let content = "The quick brown fox jumps over the lazy dog. The fox is quick!";
        let word_freq = count_word_freq(content);

        assert_eq!(word_freq.len(), 9);
        assert_eq!(word_freq.get("the"), Some(&3));
        assert_eq!(word_freq.get("quick"), Some(&2));
        assert_eq!(word_freq.get("brown"), Some(&1));
        assert_eq!(word_freq.get("fox"), Some(&2));
        assert_eq!(word_freq.get("jumps"), Some(&1));
        assert_eq!(word_freq.get("over"), Some(&1));
        assert_eq!(word_freq.get("lazy"), Some(&1));
        assert_eq!(word_freq.get("dog"), Some(&1));
        assert_eq!(word_freq.get("is"), Some(&1));
    }
}
