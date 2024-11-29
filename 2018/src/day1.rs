use std::fs;
use std::collections::HashSet;

pub fn solve_day1() {
    let file_path = "src/2018_01.txt";

    match fs::read_to_string(file_path) {
        Ok(contents) => {
            let frequencies: Vec<i32> = 
                contents
                    .split(' ')
                    .map(|word| word.replace("+", ""))
                    .map(|word| word.parse::<i32>().unwrap_or(0))
                    .collect();

            let ending_frequency: String = 
                frequencies.iter().sum::<i32>().to_string();

            println!("Day 1.1: {}", ending_frequency);

            let mut current_frequency = 0;
            let mut seen_frequencies = HashSet::new();
            let mut found_repetition = false;

            while !found_repetition {
                for frequency in &frequencies {
                    current_frequency += frequency;
                    if seen_frequencies.contains(&current_frequency) {
                        found_repetition = true;
                        break;
                    }
                    seen_frequencies.insert(current_frequency);
                }
            }

            println!("Day 1.2: {}", current_frequency);
        }
        Err(e) => {
            println!("Error reading file {}: {}", file_path, e);
        }
    }
}