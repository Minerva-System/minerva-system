//! This module wraps functions related to file reading.

use std::fs;

/// Slurp an entire text file into memory, as a string.
/// If the file cannot be found, panics.
pub fn slurp(filename: &str) -> String {
    fs::read_to_string(filename).unwrap_or_else(|_| panic!("Unable to read file \"{}\"", filename))
}
