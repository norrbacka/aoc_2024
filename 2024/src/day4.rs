use std::fs;
use regex::Regex;


pub fn solve() {
    let file_path = "src/2024_04.txt";

    match fs::read_to_string(file_path) {
        Ok(contents) => {
            
            let parsed: Vec<Vec<String>> = contents
                .lines()
                .map(|line| line
                    .chars()
                    .map(|c| c.to_string())
                    .collect::<Vec<String>>())
                .collect();

            let mut count: i32 = 0;

            let y_size = parsed.len();
            let x_size = parsed[0].len();

            let mut words: Vec<String> = Vec::<String>::new();

            for y in 0..y_size {

                let mut normal = parsed[y][0..x_size].join("");
                words.push(normal.clone());

                let mut reversed = normal.chars().rev().collect();
                words.push(reversed);
            }

            for x in 0..x_size {
                let mut column_chars = Vec::new();
                for y in 0..y_size {
                    column_chars.push(parsed[y][x].clone());
                }
                let normal = column_chars.join("");
                words.push(normal.clone());
            
                let reversed = normal.chars().rev().collect();
                words.push(reversed);
            }

            for d in 0..(x_size + y_size - 1) {
                let mut diagonal_chars = Vec::new();
            
                for row in 0..=d {
                    let col = d - row;
            
                    if row < y_size && col < x_size {
                        diagonal_chars.push(parsed[row][col].clone());
                    }
                }
            
                let new_word = diagonal_chars.join("");
                if new_word.len() >= 4 {
                    words.push(new_word.clone());
                    words.push(new_word.chars().rev().collect());
                }
            }

            let min_diff = -(y_size as isize - 1);
            let max_diff = x_size as isize - 1;
            
            for d in (min_diff..=max_diff).rev() {
                let mut diagonal_chars = Vec::new();
            
                for row in 0..y_size {
                    let col_i = row as isize + d;
                    if col_i >= 0 && (col_i as usize) < x_size {
                        diagonal_chars.push(parsed[row][col_i as usize].clone());
                    }
                }
            
                let word = diagonal_chars.join("");
                if !word.is_empty() {
                    let new_word = diagonal_chars.join("");
                    if new_word.len() >= 4 {
                        words.push(new_word.clone());
                        words.push(new_word.chars().rev().collect());
                    }
                }
            }          

            let count: usize = words
                .iter()
                .map(|x| x.matches("XMAS").count())
                .sum();
            println!("Day 4.1 {}", count);

            words.clear();

            for i in 0..(x_size - 2) {
                for j in 0..(y_size - 2) {
                    let mut grid = Vec::new();
                    for k in 0..3 {
                        grid.push(parsed[i + k][j..j + 3].to_vec());
                    }
                    words.push(grid
                        .into_iter()
                        .map(|x| x.join(""))
                        .collect::<Vec<String>>()
                        .join(""));
                }
            }

            let regexes = [
                "M.S.A.M.S",
                "S.S.A.M.M",
                "S.M.A.S.M",
                "M.M.A.S.S"
            ];

            let compiled_regexes: Vec<Regex> = regexes
            .iter()
            .map(|&pattern| Regex::new(pattern).unwrap())
            .collect();

            let words_matched: Vec<(String, Vec<String>)> = words.iter()
                .map(|word| {
                    let matching_regexes: Vec<String> = compiled_regexes.iter()
                        .filter(|regex| regex.is_match(word))
                        .map(|regex| regex.as_str().to_string())
                        .collect();
                    (word.to_string(), matching_regexes)
                })
                .filter(|(_, matching_regexes)| !matching_regexes.is_empty()) 
                .collect();
            
            println!("Day 4.2: {}", words_matched.len());
        }
        Err(e) => {
            println!("Error reading file {}: {}", file_path, e);
        }
    }
}