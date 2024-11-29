use std::fs;
use std::collections::HashSet;

pub fn solve_day1() {
    let file_path = "src/2024_01.txt";

    match fs::read_to_string(file_path) {
        Ok(contents) => {
            
            println!("Day 1.21: hello");
        }
        Err(e) => {
            println!("Error reading file {}: {}", file_path, e);
        }
    }
}