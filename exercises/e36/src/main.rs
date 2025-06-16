//! # Time Statistics Calculator
//!
//! This module implements a command-line application for analyzing execution time
//! measurements by calculating and displaying standard statistical metrics.
//!
//! ## Features
//!
//! - **File I/O**: Reads execution time data from a text file
//! - **Statistical Analysis**: Calculates mean, standard deviation, minimum, and maximum values
//! - **Error Handling**: Gracefully handles file access and parsing errors
//! - **NaN Handling**: Properly filters out NaN values when calculating min and max
//!
//! The application reads time measurements from a file, computes key statistical metrics,
//! and presents them in a clear, formatted output for performance analysis.
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

fn read_times_from_file(file_path: PathBuf) -> Result<Vec<f64>, std::io::Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut times = Vec::new();

    for line in reader.lines() {
        let time_str = line?;
        if let Ok(time) = time_str.trim().parse::<f64>() {
            times.push(time);
        }
    }
    Ok(times)
}

fn compute_mean(times: &[f64]) -> f64 {
    if times.is_empty() {
        return 0.0;
    }
    let sum: f64 = times.iter().sum();
    sum / times.len() as f64
}

fn compute_std_deviation(times: &[f64], mean: f64) -> f64 {
    if times.len() < 2 {
        return 0.0;
    }
    let variance: f64 =
        times.iter().map(|&t| (t - mean).powi(2)).sum::<f64>() / (times.len()) as f64;
    variance.sqrt()
}

fn max(times: &[f64]) -> f64 {
    times
        .iter()
        .cloned()
        .filter(|&v| !v.is_nan())
        .fold(f64::NEG_INFINITY, f64::max)
}

fn min(times: &[f64]) -> f64 {
    times
        .iter()
        .cloned()
        .filter(|&v| !v.is_nan())
        .fold(f64::INFINITY, f64::min)
}

fn print_statistics(times: &[f64]) {
    if times.is_empty() {
        println!("No times available to compute statistics.");
        return;
    }

    let mean = compute_mean(times);
    let std_dev = compute_std_deviation(times, mean);
    let max = max(times);
    let min = min(times);

    println!("The average is {:.2}ms", mean);
    println!("The minimum time is {:.2}ms", min);
    println!("The maximum time is {:.2}ms", max);
    println!("The standard deviation is {:.2}ms", std_dev);
}

fn main() {
    let file_path = PathBuf::from("exercises/e36/inputs/times.txt");

    match read_times_from_file(file_path) {
        Ok(times) => {
            print_statistics(&times);
        }
        Err(e) => {
            eprintln!("Error reading times from file: {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_mean_handles_empty_list() {
        let empty_times: Vec<f64> = vec![];
        assert_eq!(compute_mean(&empty_times), 0.0);
    }

    #[test]
    fn compute_mean_calculates_average_correctly() {
        let times = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(compute_mean(&times), 3.0);
    }

    #[test]
    fn compute_std_deviation_handles_empty_list() {
        let empty_times: Vec<f64> = vec![];
        assert_eq!(compute_std_deviation(&empty_times, 0.0), 0.0);
    }

    #[test]
    fn compute_std_deviation_handles_single_element() {
        let times = vec![5.0];
        assert_eq!(compute_std_deviation(&times, 5.0), 0.0);
    }

    #[test]
    fn compute_std_deviation_calculates_correctly() {
        let times = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
        let mean = compute_mean(&times);
        // Expected standard deviation is 2.0
        assert!((compute_std_deviation(&times, mean) - 2.0).abs() < 0.001);
    }

    #[test]
    fn min_handles_regular_values() {
        let times = vec![3.0, 1.0, 7.0, 5.0];
        assert_eq!(min(&times), 1.0);
    }

    #[test]
    fn min_ignores_nan_values() {
        let times = vec![3.0, 1.0, f64::NAN, 5.0];
        assert_eq!(min(&times), 1.0);
    }

    #[test]
    fn max_handles_regular_values() {
        let times = vec![3.0, 1.0, 7.0, 5.0];
        assert_eq!(max(&times), 7.0);
    }

    #[test]
    fn max_ignores_nan_values() {
        let times = vec![3.0, 10.0, f64::NAN, 5.0];
        assert_eq!(max(&times), 10.0);
    }
}
