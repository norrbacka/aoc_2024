use std::fs;

pub fn solve() {
    let file_path = "src/2024_01.txt";

    match fs::read_to_string(file_path) {
        Ok(contents) => {
            
            // parse data

            let mut first_numbers: Vec<i32> = Vec::new();
            let mut second_numbers: Vec<i32> = Vec::new();

            contents
                .lines()
                .for_each(|line| {
                    let parts: Vec<u32> = line
                        .split("   ")
                        .filter_map(|s| s.trim().parse().ok())
                        .collect();
                    first_numbers.push(parts[0] as i32);
                    second_numbers.push(parts[1] as i32);
                });

            // part 1

            let mut sorted_first_numbers: Vec<i32> = first_numbers.clone();
            let mut sorted_second_numbers: Vec<i32> = second_numbers.clone();

            sorted_first_numbers.sort();
            sorted_second_numbers.sort();
            
            let mut day11: i32 = 0;
            for i in 0..sorted_first_numbers.len() {
                day11 = day11 + (sorted_first_numbers[i] - sorted_second_numbers[i]).abs();
            }
            println!("Day 1.1: {}", day11);

            // part 2

            let mut similarity: i32 = 0;
            for i in 0..first_numbers.len() {

                let current_number: i32 = first_numbers[i];
                let frequencies: i32 = second_numbers.iter().filter(|&n| *n == current_number).count() as i32;

                similarity = similarity + (frequencies * current_number);
            }
            println!("Day 1.2: {}", similarity);
        }
        Err(e) => {
            println!("Error reading file {}: {}", file_path, e);
        }
    }
}