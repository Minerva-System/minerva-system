use std::fs;

pub fn slurp(filename: &str) -> String {
    fs::read_to_string(filename).expect(&format!("Unable to read file \"{}\"", filename))
}
