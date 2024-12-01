use std::fs;

pub fn solve_day2() {
    let file_path = "src/2024_02.txt";

    match fs::read_to_string(file_path) {
        Ok(contents) => {
            
           
        }
        Err(e) => {
            println!("Error reading file {}: {}", file_path, e);
        }
    }
}