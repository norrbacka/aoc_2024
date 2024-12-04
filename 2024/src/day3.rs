use std::fs;
use regex::Regex;

fn process_matches(contents: &str) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").expect("Invalid regex");
    let matches = re.find_iter(&contents).map(|m| {
        let matched_str = m.as_str(); 
        println!("Matched string: {}", matched_str); 

        let binding = matched_str
            .replace("mul(", "")
            .replace(")", "");
        let num: Vec<&str> = binding.split(",").collect();
        let num1: i32 = num[0].parse().expect("Failed");
        let num2: i32 = num[1].parse().expect("Failed");
        
        return (num1 * num2);
    }).collect::<Vec<i32>>();

    let mut count: i32 = 0;
    for i in 0..matches.len() {
        count = count + matches[i];
    }
    count
}

pub fn solve() {
    let file_path = "src/2024_03.txt";

    match fs::read_to_string(file_path) {
        Ok(contents) => {
            let count = process_matches(&contents); 
            println!("Day 3.1: {}", count);

            let re_remove_disabled = Regex::new(r"don't\(\s*.*?\s*do\(\)").expect("Invalid regex");
            let cleaned_contents = re_remove_disabled.replace_all(&contents, "").to_string(); 
            let count2 = process_matches(&cleaned_contents); 
            println!("Day 3.2: {}", count2);
        }
        Err(e) => {
            println!("Error reading file {}: {}", file_path, e);
        }
    }
}